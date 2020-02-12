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
use ipc_channel::ipc;
use std::cell::RefCell;
use webgpu::{WebGPU, WebGPUCommandEncoder, WebGPURequest, wgpu::command::{ComputeCommand, PhantomSlice, RawPass}};

#[dom_struct]
pub struct GPUComputePassEncoder {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "channels are hard"]
    channel: WebGPU,
    label: DomRefCell<Option<DOMString>>,
    #[ignore_malloc_size_of = "wgpu handle"]
    parent: WebGPUCommandEncoder,
    #[ignore_malloc_size_of = "WIP"]
    raw_pass: RefCell<Option<RawPass>>,
}

impl GPUComputePassEncoder {
    pub fn new_inherited(channel: WebGPU, parent: WebGPUCommandEncoder) -> GPUComputePassEncoder {
        GPUComputePassEncoder {
            channel,
            reflector_: Reflector::new(),
            parent,
            label: DomRefCell::new(None),
            raw_pass: RefCell::new(Some(RawPass::new_compute(parent.0))),
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

    #[allow(unsafe_code)]
    /// https://gpuweb.github.io/gpuweb/#dom-gpucomputepassencoder-dispatch
    fn Dispatch(&self, x: u32, y: u32, z: u32) {
        println!("Servo Dispatch");
        if let Some(raw_pass) = self.raw_pass
            .borrow_mut()
            .as_mut() {
                unsafe { raw_pass.encode(&ComputeCommand::Dispatch([x, y, z])) };
            } else {
                println!("Something went wrong 1");
            }
    }

    #[allow(unsafe_code)]
    /// https://gpuweb.github.io/gpuweb/#dom-gpurenderpassencoder-endpass
    fn EndPass(&self) {
        println!("Servo EndPass");
        let (sender, receiver) = ipc::channel().unwrap();
        if let Some(raw_pass) = self.raw_pass
            .borrow_mut()
            .take() {
                let (pass_data, _) = unsafe { raw_pass.finish_compute() };

                self.channel
                    .0
                    .send(WebGPURequest::RunComputePass(
                        sender,
                        self.parent.0,
                        pass_data,
                    ))
                    .unwrap();

                let _ = receiver.recv().unwrap();
            } else {
                println!("Something went wrong 2");
            }
        let _ = receiver.recv().unwrap();
    }

    #[allow(unsafe_code)]
    /// https://gpuweb.github.io/gpuweb/#dom-gpuprogrammablepassencoder-setbindgroup
    fn SetBindGroup(&self, index: u32, bind_group: &GPUBindGroup, dynamic_offsets: Vec<u32>) {
        println!("Servo SetBindGroup");
        if let Some(raw_pass) = self.raw_pass
            .borrow_mut()
            .as_mut() {
                unsafe {
                    raw_pass.encode(&ComputeCommand::SetBindGroup {
                        index: index as u8,
                        num_dynamic_offsets: dynamic_offsets.len() as u8,
                        bind_group_id: bind_group.id().0,
                        phantom_offsets: PhantomSlice::new(),
                    });

                    raw_pass.encode_slice(
                        &dynamic_offsets,
                    );
                }
            } else {
                println!("Something went wrong 3");
            }
    }

    #[allow(unsafe_code)]
    /// https://gpuweb.github.io/gpuweb/#dom-gpucomputepassencoder-setpipeline
    fn SetPipeline(&self, pipeline: &GPUComputePipeline) {
        println!("Servo SetPipeline");
        if let Some(raw_pass) = self.raw_pass
            .borrow_mut()
            .as_mut() {
                unsafe { raw_pass.encode(&ComputeCommand::SetPipeline(pipeline.id().0)) };
            } else {
                println!("Something went wrong 4");
            }
    }
}
