/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::GPUBufferBinding::{
    self, GPUBufferMethods, GPUBufferSize,
};
use crate::dom::bindings::error::Error;
use crate::dom::bindings::reflector::DomObject;
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use crate::dom::gpu::response_async;
use crate::dom::gpu::AsyncWGPUListener;
use crate::dom::promise::Promise;
use crate::realms::InRealm;
use crate::script_runtime::JSContext as SafeJSContext;
use dom_struct::dom_struct;
use ipc_channel::ipc;
use js::jsapi::JSObject;
use js::jsval::{JSVal, ObjectValue};
use js::typedarray::{ArrayBuffer, CreateWith};
use std::cell::Cell;
use std::ptr::NonNull;
use std::rc::Rc;
use webgpu::{WebGPU, WebGPUBuffer, WebGPUDevice, WebGPURequest, WebGPUResponse};

#[derive(MallocSizeOf)]
pub enum GPUBufferState {
    Mapped,
    Unmapped,
    Destroyed,
}

#[dom_struct]
pub struct GPUBuffer {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "channels are hard"]
    channel: WebGPU,
    label: DomRefCell<Option<DOMString>>,
    size: GPUBufferSize,
    usage: u32,
    state: DomRefCell<GPUBufferState>,
    buffer: WebGPUBuffer,
    device: WebGPUDevice,
    valid: Cell<bool>,
}

impl GPUBuffer {
    fn new_inherited(
        channel: WebGPU,
        buffer: WebGPUBuffer,
        device: WebGPUDevice,
        state: GPUBufferState,
        size: GPUBufferSize,
        usage: u32,
        valid: bool,
    ) -> GPUBuffer {
        Self {
            reflector_: Reflector::new(),
            channel,
            label: DomRefCell::new(None),
            state: DomRefCell::new(state),
            size: size,
            usage: usage,
            valid: Cell::new(valid),
            device,
            buffer,
        }
    }

    #[allow(unsafe_code)]
    pub fn new(
        global: &GlobalScope,
        channel: WebGPU,
        buffer: WebGPUBuffer,
        device: WebGPUDevice,
        state: GPUBufferState,
        size: GPUBufferSize,
        usage: u32,
        valid: bool,
    ) -> DomRoot<GPUBuffer> {
        reflect_dom_object(
            Box::new(GPUBuffer::new_inherited(
                channel, buffer, device, state, size, usage, valid,
            )),
            global,
            GPUBufferBinding::Wrap,
        )
    }
}

impl GPUBuffer {
    pub fn id(&self) -> WebGPUBuffer {
        self.buffer
    }

    pub fn size(&self) -> GPUBufferSize {
        self.size
    }

    pub fn usage(&self) -> u32 {
        self.usage
    }
}

impl Drop for GPUBuffer {
    fn drop(&mut self) {
        self.Destroy()
    }
}

impl GPUBufferMethods for GPUBuffer {
    /// https://gpuweb.github.io/gpuweb/#dom-gpubuffer-unmap
    fn Unmap(&self) {
        self.channel
            .0
            .send(WebGPURequest::UnmapBuffer(self.buffer))
            .unwrap();
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpubuffer-destroy
    fn Destroy(&self) {
        match *self.state.borrow() {
            GPUBufferState::Mapped => {
                self.Unmap();
            },
            _ => {},
        };
        self.channel
            .0
            .send(WebGPURequest::DestroyBuffer(self.buffer))
            .unwrap();
        *self.state.borrow_mut() = GPUBufferState::Destroyed;
    }

    #[allow(unsafe_code)]
    fn MapReadAsync(&self, cx: SafeJSContext) -> NonNull<JSObject> {
        let (sender, receiver) = ipc::channel().unwrap();
        self.channel
            .0
            .send(WebGPURequest::MapReadAsync(
                sender,
                self.buffer.0,
                self.device.0,
                self.usage,
                self.size,
            ))
            .expect("Failed to send MapReadAsync request");

        let response = receiver.recv().unwrap();
        match response.unwrap() {
            WebGPUResponse::MapReadAsync(data) => {
                rooted!(in(*cx) let mut js_array_buffer = std::ptr::null_mut::<JSObject>());
                unsafe {
                    assert!(ArrayBuffer::create(
                        *cx,
                        CreateWith::Slice(&data),
                        js_array_buffer.handle_mut(),
                    )
                    .is_ok());
                    return NonNull::new_unchecked(js_array_buffer.get());
                }
            },
            _ => panic!("Wrong response"),
        }
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn GetLabel(&self) -> Option<DOMString> {
        self.label.borrow().clone()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn SetLabel(&self, value: Option<DOMString>) {
        *self.label.borrow_mut() = value;
    }
}

/*impl AsyncWGPUListener for GPUBuffer {
    #[allow(unsafe_code)]
    fn handle_response(&self, response: WebGPUResponse, promise: &Rc<Promise>) {
        match response {
            WebGPUResponse::MapReadAsync(bytes) => {
                println!("## bytes {:?}", &bytes[0..10]);
                let cx = promise.global().get_cx();
                rooted!(in(*cx) let mut array_buffer_ptr = std::ptr::null_mut::<JSObject>());
                let arraybuffer = unsafe {
                    ArrayBuffer::create(
                        *cx,
                        CreateWith::Slice(&bytes[0..10]),
                        array_buffer_ptr.handle_mut(),
                    )
                };
                match arraybuffer {
                    Ok(_) => promise.resolve_native(&ObjectValue(array_buffer_ptr.get())),
                    Err(_) => promise.reject_error(Error::Operation),
                }
            },
            _ => promise.reject_error(Error::Operation),
        }
    }
}*/
