/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::GPUComputePassEncoderBinding::{
    self, GPUComputePassEncoderMethods,
};
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use crate::dom::gpubindgroup::GPUBindGroup;
use crate::dom::gpucomputepipeline::GPUComputePipeline;
use dom_struct::dom_struct;
use std::cell::RefCell;
use webgpu::{ComputeCommand, WebGPU, WebGPUCommandEncoder, WebGPURequest};

#[dom_struct]
pub struct GPUComputePassEncoder {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "channels are hard"]
    channel: WebGPU,
    label: DomRefCell<Option<DOMString>>,
    #[ignore_malloc_size_of = "wgpu handle"]
    parent: WebGPUCommandEncoder,
    #[ignore_malloc_size_of = "WIP"]
    commands: RefCell<Vec<ComputeCommand>>,
}

impl GPUComputePassEncoder {
    pub fn new_inherited(channel: WebGPU, parent: WebGPUCommandEncoder) -> GPUComputePassEncoder {
        GPUComputePassEncoder {
            channel,
            reflector_: Reflector::new(),
            label: DomRefCell::new(None),
            parent,
            commands: RefCell::new(Vec::<ComputeCommand>::with_capacity(1)),
        }
    }

    pub fn new(
        global: &GlobalScope,
        channel: WebGPU,
        parent: WebGPUCommandEncoder,
    ) -> DomRoot<GPUComputePassEncoder> {
        reflect_dom_object(
            Box::new(GPUComputePassEncoder::new_inherited(channel, parent)),
            global,
            GPUComputePassEncoderBinding::Wrap,
        )
    }
}

impl GPUComputePassEncoderMethods for GPUComputePassEncoder {
    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn GetLabel(&self) -> Option<DOMString> {
        self.label.borrow().clone()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn SetLabel(&self, value: Option<DOMString>) {
        *self.label.borrow_mut() = value;
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpucomputepassencoder-dispatch
    fn Dispatch(&self, x: u32, y: u32, z: u32) {
        self.commands
            .borrow_mut()
            .push(ComputeCommand::Dispatch([x, y, z]));
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpurenderpassencoder-endpass
    fn EndPass(&self) {
        self.commands.borrow_mut().push(ComputeCommand::End);
        self.channel
            .0
            .send(WebGPURequest::RunComputePass(
                self.parent.0,
                self.commands.borrow_mut().drain(..).collect(),
            ))
            .unwrap();
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuprogrammablepassencoder-setbindgroup
    fn SetBindGroup(&self, index: u32, bind_group: &GPUBindGroup, dynamic_offsets: Vec<u32>) {
        self.commands
            .borrow_mut()
            .push(ComputeCommand::SetBindGroup {
                index,
                bind_group_id: bind_group.id().0,
                dynamic_offsets: dynamic_offsets.iter().map(|o| *o as u64).collect(),
            });
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpucomputepassencoder-setpipeline
    fn SetPipeline(&self, pipeline: &GPUComputePipeline) {
        self.commands
            .borrow_mut()
            .push(ComputeCommand::SetComputePipeline(pipeline.id().0));
    }
}
