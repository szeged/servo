/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::GPUCommandEncoderBinding::{
    self, GPUCommandEncoderMethods, GPUComputePassDescriptor,
};
use crate::dom::bindings::reflector::DomObject;
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use crate::dom::gpubuffer::GPUBuffer;
use crate::dom::gpucomputepassencoder::GPUComputePassEncoder;
use dom_struct::dom_struct;
use webgpu::{wgpu::command::RawPass, WebGPU, WebGPUCommandEncoder, WebGPURequest};

#[dom_struct]
pub struct GPUCommandEncoder {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "channels are hard"]
    channel: WebGPU,
    label: DomRefCell<Option<DOMString>>,
    encoder: WebGPUCommandEncoder,
}

impl GPUCommandEncoder {
    pub fn new_inherited(channel: WebGPU, encoder: WebGPUCommandEncoder) -> GPUCommandEncoder {
        GPUCommandEncoder {
            channel,
            reflector_: Reflector::new(),
            label: DomRefCell::new(None),
            encoder,
        }
    }

    pub fn new(
        global: &GlobalScope,
        channel: WebGPU,
        encoder: WebGPUCommandEncoder,
    ) -> DomRoot<GPUCommandEncoder> {
        reflect_dom_object(
            Box::new(GPUCommandEncoder::new_inherited(channel, encoder)),
            global,
            GPUCommandEncoderBinding::Wrap,
        )
    }
}

impl GPUCommandEncoderMethods for GPUCommandEncoder {
    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn GetLabel(&self) -> Option<DOMString> {
        self.label.borrow().clone()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn SetLabel(&self, value: Option<DOMString>) {
        *self.label.borrow_mut() = value;
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpucommandencoder-begincomputepass
    fn BeginComputePass(
        &self,
        _descriptor: &GPUComputePassDescriptor,
    ) -> DomRoot<GPUComputePassEncoder> {
        GPUComputePassEncoder::new(
            &self.global(),
            self.channel.clone(),
            RawPass::new_compute(self.encoder.0),
        )
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpucommandencoder-copybuffertobuffer
    fn CopyBufferToBuffer(
        &self,
        source: &GPUBuffer,
        source_offset: u64,
        destination: &GPUBuffer,
        destination_offset: u64,
        size: u64,
    ) {
        self.channel
            .0
            .send(WebGPURequest::CopyBuffer(
                self.encoder.0,
                source.id().0,
                source_offset,
                destination.id().0,
                destination_offset,
                size,
            ))
            .expect("Failed to send CopyBufferToBuffer");
    }
}
