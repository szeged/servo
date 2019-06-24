/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::gl_context::GLContextFactory;
use crate::webgl_thread::{WebGLExternalImageApi, WebGLExternalImageHandler, WebGLThread};
use canvas_traits::webgl::webgl_channel;
use canvas_traits::webgl::DOMToTextureCommand;
use canvas_traits::webgl::{WebGLChan, WebGLContextId, WebGLLockMessage, WebGLMsg, WebGLPipeline};
use canvas_traits::webgl::{WebGLReceiver, WebGLSender, WebVRCommand, WebVRRenderHandler};
use euclid::Size2D;
use fnv::FnvHashMap;
use gleam::gl;
#[cfg(target_os = "macos")]
use io_surface;
use servo_config::pref;
use std::rc::Rc;

/// WebGL Threading API entry point that lives in the constellation.
pub struct WebGLThreads(WebGLSender<WebGLMsg>);

type IOSurfaceId = u32;

impl WebGLThreads {
    /// Creates a new WebGLThreads object
    pub fn new(
        gl_factory: GLContextFactory,
        webrender_gl: Rc<dyn gl::Gl>,
        webrender_api_sender: webrender_api::RenderApiSender,
        webvr_compositor: Option<Box<dyn WebVRRenderHandler>>,
    ) -> (
        WebGLThreads,
        Box<dyn webrender::ExternalImageHandler>,
        Option<Box<dyn webrender::OutputImageHandler>>,
    ) {
        // This implementation creates a single `WebGLThread` for all the pipelines.
        let channel = WebGLThread::start(
            gl_factory,
            webrender_api_sender,
            webvr_compositor.map(|c| WebVRRenderWrapper(c)),
        );
        let output_handler = if pref!(dom.webgl.dom_to_texture.enabled) {
            Some(Box::new(OutputHandler::new(
                webrender_gl.clone(),
                channel.clone(),
            )))
        } else {
            None
        };
        let external =
            WebGLExternalImageHandler::new(WebGLExternalImages::new(webrender_gl, channel.clone()));
        (
            WebGLThreads(channel),
            Box::new(external),
            output_handler.map(|b| b as Box<_>),
        )
    }

    /// Gets the WebGLThread handle for each script pipeline.
    pub fn pipeline(&self) -> WebGLPipeline {
        // This mode creates a single thread, so the existing WebGLChan is just cloned.
        WebGLPipeline(WebGLChan(self.0.clone()))
    }

    /// Sends a exit message to close the WebGLThreads and release all WebGLContexts.
    pub fn exit(&self) -> Result<(), &'static str> {
        self.0
            .send(WebGLMsg::Exit)
            .map_err(|_| "Failed to send Exit message")
    }
}

/// Bridge between the webrender::ExternalImage callbacks and the WebGLThreads.
struct WebGLExternalImages {
    webrender_gl: Rc<dyn gl::Gl>,
    webgl_channel: WebGLSender<WebGLMsg>,
    // Mapping between an IOSurface and the texture it is bound on the WR thread
    textures: FnvHashMap<IOSurfaceId, gl::GLuint>,
    // Used to avoid creating a new channel on each received WebRender request.
    lock_channel: (
        WebGLSender<WebGLLockMessage>,
        WebGLReceiver<WebGLLockMessage>,
    ),
}

impl Drop for WebGLExternalImages {
    fn drop(&mut self) {
        for (_, texture_id) in &self.textures {
            self.webrender_gl.delete_textures(&[*texture_id]);
        }
    }
}

impl WebGLExternalImages {
    fn new(webrender_gl: Rc<dyn gl::Gl>, channel: WebGLSender<WebGLMsg>) -> Self {
        Self {
            webrender_gl,
            webgl_channel: channel,
            textures: Default::default(),
            lock_channel: webgl_channel().unwrap(),
        }
    }
}

impl WebGLExternalImageApi for WebGLExternalImages {
    fn lock(&mut self, ctx_id: WebGLContextId) -> (u32, Size2D<i32>) {
        // WebGL Thread has it's own GL command queue that we need to synchronize with the WR GL command queue.
        // The WebGLMsg::Lock message inserts a fence in the WebGL command queue.
        self.webgl_channel
            .send(WebGLMsg::Lock(ctx_id, self.lock_channel.0.clone()))
            .unwrap();
        let WebGLLockMessage {
            texture_id,
            size,
            io_surface_id,
            gl_sync,
        } = self.lock_channel.1.recv().unwrap();

        // If we have a new IOSurface bind it to a new texture on the WR thread,
        // or if it's already bound use that texture.
        // In the case of IOsurfaces we send these textures to WR.
        let texture_id = match io_surface_id {
            Some(_io_surface_id) => {
                #[cfg(target_os = "macos")]
                let gl = &self.webrender_gl;
                #[cfg(target_os = "macos")]
                let texture_id = *self.textures.entry(_io_surface_id).or_insert_with(|| {
                    let texture_id = gl.gen_textures(1)[0];
                    gl.bind_texture(gl::TEXTURE_RECTANGLE_ARB, texture_id);
                    let io_surface = io_surface::lookup(_io_surface_id);
                    io_surface.bind_to_gl_texture(size.width, size.height);
                    texture_id
                });
                texture_id
            },
            None => {
                // The next glWaitSync call is run on the WR thread and it's used to synchronize the two
                // flows of OpenGL commands in order to avoid WR using a semi-ready WebGL texture.
                // glWaitSync doesn't block WR thread, it affects only internal OpenGL subsystem.
                self.webrender_gl
                    .wait_sync(gl_sync as gl::GLsync, 0, gl::TIMEOUT_IGNORED);
                texture_id
            },
        };

        (texture_id, size)
    }

    fn unlock(&mut self, ctx_id: WebGLContextId) {
        self.webgl_channel.send(WebGLMsg::Unlock(ctx_id)).unwrap();
    }
}

/// Wrapper to send WebVR commands used in `WebGLThread`.
struct WebVRRenderWrapper(Box<dyn WebVRRenderHandler>);

impl WebVRRenderHandler for WebVRRenderWrapper {
    fn handle(
        &mut self,
        gl: &dyn gl::Gl,
        command: WebVRCommand,
        texture: Option<(u32, Size2D<i32>)>,
    ) {
        self.0.handle(gl, command, texture);
    }
}

/// struct used to implement DOMToTexture feature and webrender::OutputImageHandler trait.
type OutputHandlerData = Option<(u32, Size2D<i32>)>;
struct OutputHandler {
    webrender_gl: Rc<dyn gl::Gl>,
    webgl_channel: WebGLSender<WebGLMsg>,
    // Used to avoid creating a new channel on each received WebRender request.
    lock_channel: (
        WebGLSender<OutputHandlerData>,
        WebGLReceiver<OutputHandlerData>,
    ),
    sync_objects: FnvHashMap<webrender_api::PipelineId, gl::GLsync>,
}

impl OutputHandler {
    fn new(webrender_gl: Rc<dyn gl::Gl>, channel: WebGLSender<WebGLMsg>) -> Self {
        Self {
            webrender_gl,
            webgl_channel: channel,
            lock_channel: webgl_channel().unwrap(),
            sync_objects: Default::default(),
        }
    }
}

/// Bridge between the WR frame outputs and WebGL to implement DOMToTexture synchronization.
impl webrender::OutputImageHandler for OutputHandler {
    fn lock(
        &mut self,
        id: webrender_api::PipelineId,
    ) -> Option<(u32, webrender_api::FramebufferIntSize)> {
        // Insert a fence in the WR command queue
        let gl_sync = self
            .webrender_gl
            .fence_sync(gl::SYNC_GPU_COMMANDS_COMPLETE, 0);
        // The lock command adds a WaitSync call on the WebGL command flow.
        let command = DOMToTextureCommand::Lock(id, gl_sync as usize, self.lock_channel.0.clone());
        self.webgl_channel
            .send(WebGLMsg::DOMToTextureCommand(command))
            .unwrap();
        self.lock_channel.1.recv().unwrap().map(|(tex_id, size)| {
            (
                tex_id,
                webrender_api::FramebufferIntSize::new(size.width, size.height),
            )
        })
    }

    fn unlock(&mut self, id: webrender_api::PipelineId) {
        if let Some(gl_sync) = self.sync_objects.remove(&id) {
            // Flush the Sync object into the GPU's command queue to guarantee that it it's signaled.
            self.webrender_gl.flush();
            // Mark the sync object for deletion.
            self.webrender_gl.delete_sync(gl_sync);
        }
    }
}
