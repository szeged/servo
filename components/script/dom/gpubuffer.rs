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
use crate::dom::bindings::trace::RootedTraceableBox;
use crate::dom::globalscope::GlobalScope;
use crate::dom::gpu::response_async;
use crate::dom::gpu::AsyncWGPUListener;
use crate::dom::promise::Promise;
use crate::realms::InRealm;
use crate::script_runtime::JSContext as SafeJSContext;
use dom_struct::dom_struct;
use ipc_channel::ipc;
use js::jsapi::Heap;
use js::jsapi::JSObject;
use js::jsval::{JSVal, ObjectValue};
use js::typedarray::{ArrayBuffer, CreateWith};
use std::cell::Cell;
use std::ptr::NonNull;
use std::rc::Rc;
use webgpu::{WebGPU, WebGPUBuffer, WebGPUDevice, WebGPURequest, WebGPUResponse};

// https://gpuweb.github.io/gpuweb/#buffer-state
#[derive(MallocSizeOf)]
pub enum GPUBufferState {
    MappedForReading,
    MappedForWriting,
    MappedPendingForReading,
    MappedPendingForWriting,
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
    #[ignore_malloc_size_of = "channels are hard"]
    mapping: RootedTraceableBox<Heap<*mut JSObject>>,
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
        mapping: RootedTraceableBox<Heap<*mut JSObject>>,
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
            mapping,
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
        mapping: RootedTraceableBox<Heap<*mut JSObject>>,
    ) -> DomRoot<GPUBuffer> {
        reflect_dom_object(
            Box::new(GPUBuffer::new_inherited(
                channel, buffer, device, state, size, usage, valid, mapping,
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
        let array_buffer = match ArrayBuffer::from(self.mapping.get()) {
            Ok(array_buffer) => array_buffer.to_vec(),
            _ => unimplemented!(),
        };
        self.channel
            .0
            .send(WebGPURequest::UnmapBuffer(
                self.device.0,
                self.id(),
                self.usage(),
                self.size(),
                array_buffer,
            ))
            .unwrap();
        *self.state.borrow_mut() = GPUBufferState::Unmapped;
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpubuffer-destroy
    fn Destroy(&self) {
        match *self.state.borrow() {
            GPUBufferState::MappedForReading | GPUBufferState::MappedForWriting => {
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
    /// https://gpuweb.github.io/gpuweb/#dom-gpubuffer-mapreadasync
    fn MapReadAsync(&self, comp: InRealm) -> Rc<Promise> {
        // TODO(zakorgy) Add missing Step 1
        // Step 2
        let promise = Promise::new_in_current_realm(&self.global(), comp);
        // TODO(zakorgy) Add missing Step 3
        // Step 4
        *self.state.borrow_mut() = GPUBufferState::MappedPendingForReading;
        let cx = self.global().get_cx();

        // Step 5.1
        rooted!(in(*cx) let mut array_buffer_ptr = std::ptr::null_mut::<JSObject>());
        if unsafe {
            ArrayBuffer::create(
                *cx,
                CreateWith::Length(self.size as u32),
                array_buffer_ptr.handle_mut(),
            )
        }.is_err() {
            promise.reject_error(Error::Operation);
            return promise;
        }
        self.mapping.set(array_buffer_ptr.get());
        let sender = response_async(&promise, self);
        if self.channel
            .0
            .send(WebGPURequest::MapReadAsync(
                sender,
                self.buffer.0,
                self.device.0,
                self.usage,
                self.size,
            )).is_err() {
                promise.reject_error(Error::Operation);
                return promise;
            }

        //Step 6
        promise
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

impl AsyncWGPUListener for GPUBuffer {
    #[allow(unsafe_code)]
    fn handle_response(&self, response: WebGPUResponse, promise: &Rc<Promise>) {
        match response {
            // https://gpuweb.github.io/gpuweb/#dom-gpubuffer-mapreadasync
            WebGPUResponse::MapReadAsync(bytes) => unsafe {
                let array_buffer = match ArrayBuffer::from(self.mapping.get()) {
                    Ok(mut array_buffer) => {
                        // Step 5.2
                        array_buffer.update(&bytes);
                        // Step 5.3
                        *self.state.borrow_mut() = GPUBufferState::MappedForReading;
                        // Step 5.4
                        promise.resolve_native(&ObjectValue(*array_buffer.underlying_object()));
                    },
                    _ => promise.reject_error(Error::Operation),
                };
            },
            _ => promise.reject_error(Error::Operation),
        }
    }
}
