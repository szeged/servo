/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::GPUQueueBinding::{self, GPUQueueMethods};
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use webgpu::{WebGPU, WebGPUQueue};

#[dom_struct]
pub struct GPUQueue {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "channels are hard"]
    channel: WebGPU,
    label: DomRefCell<Option<DOMString>>,
    queue: WebGPUQueue,
}

impl GPUQueue {
    pub fn new_inherited(channel: WebGPU, queue: WebGPUQueue) -> GPUQueue {
        GPUQueue {
            channel,
            reflector_: Reflector::new(),
            label: DomRefCell::new(None),
            queue,
        }
    }

    pub fn new(global: &GlobalScope, channel: WebGPU, queue: WebGPUQueue) -> DomRoot<GPUQueue> {
        reflect_dom_object(
            Box::new(GPUQueue::new_inherited(channel, queue)),
            global,
            GPUQueueBinding::Wrap,
        )
    }
}

impl GPUQueueMethods for GPUQueue {
    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn GetLabel(&self) -> Option<DOMString> {
        self.label.borrow().clone()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn SetLabel(&self, value: Option<DOMString>) {
        *self.label.borrow_mut() = value;
    }
}
