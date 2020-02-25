/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::GPUAdapterBinding::{
    self, GPUAdapterMethods, GPUDeviceDescriptor, GPULimits, GPUExtensions
};
use crate::dom::bindings::error::Error;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use crate::dom::gpu::response_async;
use crate::dom::gpu::AsyncWGPUListener;
use crate::dom::gpudevice::GPUDevice;
use crate::dom::promise::Promise;
use crate::realms::InRealm;
use crate::script_runtime::JSContext as SafeJSContext;
use dom_struct::dom_struct;
use js::jsapi::{Heap, JSObject};
use std::ptr::NonNull;
use std::rc::Rc;
use webgpu::{WebGPU, WebGPUAdapter, WebGPURequest, WebGPUResponse, WebGPULimits, WebGPUExtensions};

#[dom_struct]
pub struct GPUAdapter {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "channels are hard"]
    channel: WebGPU,
    name: DOMString,
    #[ignore_malloc_size_of = "mozjs"]
    extensions: Heap<*mut JSObject>,
    adapter: WebGPUAdapter,
}

impl GPUAdapter {
    pub fn new_inherited(
        channel: WebGPU,
        name: DOMString,
        extensions: Heap<*mut JSObject>,
        adapter: WebGPUAdapter,
    ) -> GPUAdapter {
        GPUAdapter {
            reflector_: Reflector::new(),
            channel,
            name,
            extensions,
            adapter,
        }
    }

    pub fn new(
        global: &GlobalScope,
        channel: WebGPU,
        name: DOMString,
        extensions: Heap<*mut JSObject>,
        adapter: WebGPUAdapter,
    ) -> DomRoot<GPUAdapter> {
        reflect_dom_object(
            Box::new(GPUAdapter::new_inherited(
                channel, name, extensions, adapter,
            )),
            global,
            GPUAdapterBinding::Wrap,
        )
    }
}

impl GPUAdapterMethods for GPUAdapter {
    // https://gpuweb.github.io/gpuweb/#dom-gpuadapter-name
    fn Name(&self) -> DOMString {
        self.name.clone()
    }

    // https://gpuweb.github.io/gpuweb/#dom-gpuadapter-extensions
    fn Extensions(&self, _cx: SafeJSContext) -> NonNull<JSObject> {
        NonNull::new(self.extensions.get()).unwrap()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuadapter-requestdevice
    fn RequestDevice(&self, descriptor: &GPUDeviceDescriptor, comp: InRealm) -> Rc<Promise> {
        let promise = Promise::new_in_current_realm(&self.global(), comp);
        let sender = response_async(&promise, self);

        let id = self
            .global()
            .wgpu_create_device_id(self.adapter.0.backend());
        if self
            .channel
            .0
            .send(WebGPURequest::RequestDevice(sender, self.adapter, descriptor.extensions.into(), descriptor.limits.into(), id))
            .is_err()
        {
            promise.reject_error(Error::Operation);
        }
        promise
    }
}

impl AsyncWGPUListener for GPUAdapter {
    fn handle_response(&self, response: WebGPUResponse, promise: &Rc<Promise>) {
        match response {
            WebGPUResponse::RequestDevice(device_id, queue_id, limits, extensions) => {
                let device = GPUDevice::new(
                    &self.global(),
                    self.channel.clone(),
                    &self,
                    extensions.into(),
                    limits.into(),
                    device_id,
                    queue_id,
                );
                promise.resolve_native(&device);
            },
            _ => promise.reject_error(Error::NotSupported),
        }
    }
}

impl Clone for GPULimits {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for GPULimits {}

impl Into<WebGPULimits> for GPULimits {
    fn into(self) -> WebGPULimits {
        WebGPULimits {
            max_bind_groups: self.maxBindGroups,
            max_dynamic_uniform_buffers_per_pipeline_layout: self.maxDynamicUniformBuffersPerPipelineLayout,
            max_dynamic_storage_buffers_per_pipeline_layout: self.maxDynamicStorageBuffersPerPipelineLayout,
            max_sampled_textures_per_shader_stage: self.maxSampledTexturesPerShaderStage,
            max_samplers_per_shader_stage: self.maxSamplersPerShaderStage,
            max_storage_buffers_per_shader_stage: self.maxStorageBuffersPerShaderStage,
            max_storage_textures_per_shader_stage: self.maxStorageTexturesPerShaderStage,
            max_uniform_buffers_per_shader_stage: self.maxUniformBuffersPerShaderStage,
        }
    }
}

impl Into<GPULimits> for WebGPULimits {
    fn into(self) -> GPULimits {
        GPULimits {
            maxBindGroups: self.max_bind_groups,
            maxDynamicUniformBuffersPerPipelineLayout: self.max_dynamic_uniform_buffers_per_pipeline_layout,
            maxDynamicStorageBuffersPerPipelineLayout: self.max_dynamic_storage_buffers_per_pipeline_layout,
            maxSampledTexturesPerShaderStage: self.max_sampled_textures_per_shader_stage,
            maxSamplersPerShaderStage: self.max_samplers_per_shader_stage,
            maxStorageBuffersPerShaderStage: self.max_storage_buffers_per_shader_stage,
            maxStorageTexturesPerShaderStage: self.max_storage_textures_per_shader_stage,
            maxUniformBuffersPerShaderStage: self.max_uniform_buffers_per_shader_stage,
        }
    }
}

impl Clone for GPUExtensions {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for GPUExtensions {}

impl Into<WebGPUExtensions> for GPUExtensions {
    fn into(self) -> WebGPUExtensions {
        WebGPUExtensions {
            anisotropic_filtering: self.anisotropicFiltering,
        }
    }
}

impl Into<GPUExtensions> for WebGPUExtensions {
    fn into(self) -> GPUExtensions {
        GPUExtensions {
            anisotropicFiltering: self.anisotropic_filtering,
        }
    }
}
