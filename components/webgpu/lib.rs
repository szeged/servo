/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate log;
#[macro_use]
pub extern crate wgpu_core as wgpu;

use ipc_channel::ipc::{self, IpcReceiver, IpcSender};
use malloc_size_of::{MallocSizeOf, MallocSizeOfOps};
use servo_config::pref;
use smallvec::SmallVec;
use wgpu::command::Binder;
use wgpu::hub::{GfxBackend, Token};

#[derive(Debug, Deserialize, Serialize)]
pub enum WebGPUResponse {
    RequestAdapter(String, WebGPUAdapter, WebGPU),
    RequestDevice(WebGPUDevice, WebGPUQueue, wgpu::instance::DeviceDescriptor),
    MapReadAsync(Vec<u8>),
}

pub type WebGPUResponseResult = Result<WebGPUResponse, String>;

#[derive(Debug, Deserialize, Serialize)]
pub enum WebGPURequest {
    RequestAdapter(
        IpcSender<WebGPUResponseResult>,
        wgpu::instance::RequestAdapterOptions,
        SmallVec<[wgpu::id::AdapterId; 4]>,
    ),
    RequestDevice(
        IpcSender<WebGPUResponseResult>,
        WebGPUAdapter,
        wgpu::instance::DeviceDescriptor,
        wgpu::id::DeviceId,
    ),
    Exit(IpcSender<()>),
    CreateComputePipeline(
        IpcSender<WebGPUComputePipeline>,
        WebGPUDevice,
        wgpu::id::ComputePipelineId,
        wgpu::id::PipelineLayoutId,
        wgpu::id::ShaderModuleId,
        Vec<i8>,
    ),
    CreateBuffer(
        IpcSender<WebGPUBuffer>,
        WebGPUDevice,
        wgpu::id::BufferId,
        wgpu::resource::BufferDescriptor,
    ),
    CreateBufferMapped(
        IpcSender<(WebGPUBuffer)>,
        WebGPUDevice,
        wgpu::id::BufferId,
        wgpu::resource::BufferDescriptor,
    ),
    CreateBindGroup(
        IpcSender<WebGPUBindGroup>,
        WebGPUDevice,
        wgpu::id::BindGroupId,
        WebGPUBindGroupLayout,
        Vec<wgpu::binding_model::BindGroupBinding>,
    ),
    CreateBindGroupLayout(
        IpcSender<WebGPUBindGroupLayout>,
        WebGPUDevice,
        wgpu::id::BindGroupLayoutId,
        Vec<wgpu::binding_model::BindGroupLayoutBinding>,
    ),
    CreatePipelineLayout(
        IpcSender<WebGPUPipelineLayout>,
        WebGPUDevice,
        wgpu::id::PipelineLayoutId,
        Vec<wgpu::id::BindGroupLayoutId>,
    ),
    CreateShaderModule(
        IpcSender<WebGPUShaderModule>,
        WebGPUDevice,
        wgpu::id::ShaderModuleId,
        Vec<u32>,
    ),
    MapReadAsync(
        IpcSender<WebGPUResponseResult>,
        wgpu::id::BufferId,
        wgpu::id::DeviceId,
        u32,
        u64,
    ),
    UnmapBuffer(wgpu::id::DeviceId, WebGPUBuffer, u32, u64, Vec<u8>),
    DestroyBuffer(WebGPUBuffer),
    CreateCommandEncoder(
        IpcSender<WebGPUCommandEncoder>,
        WebGPUDevice,
        // TODO(zakorgy): Serialize CommandEncoderDescriptor in wgpu-core
        // wgpu::command::CommandEncoderDescriptor,
        wgpu::id::CommandEncoderId,
    ),
    CopyBuffer(
        wgpu::id::CommandEncoderId,
        wgpu::id::BufferId,
        wgpu::BufferAddress,
        wgpu::id::BufferId,
        wgpu::BufferAddress,
        wgpu::BufferAddress,
    ),
    CommandEncoderFinish(
        IpcSender<WebGPUCommandBuffer>,
        wgpu::id::CommandEncoderId,
        // TODO(zakorgy): Serialize CommandBufferDescriptor in wgpu-core
        // wgpu::CommandBufferDescriptor,
    ),
    Submit(wgpu::id::QueueId, Vec<wgpu::id::CommandBufferId>),
    RunComputePass(
        IpcSender<()>,
        wgpu::id::CommandEncoderId,
        Vec<ComputeCommand>,
    ),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ComputeCommand {
    SetBindGroup {
        index: u32,
        bind_group_id: wgpu::id::BindGroupId,
        dynamic_offsets: Vec<u64>,
    },
    SetComputePipeline(wgpu::id::ComputePipelineId),
    Dispatch([u32; 3]),
    End,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebGPU(pub IpcSender<WebGPURequest>);

impl WebGPU {
    pub fn new() -> Option<Self> {
        if !pref!(dom.webgpu.enabled) {
            return None;
        }
        let (sender, receiver) = match ipc::channel() {
            Ok(sender_and_receiver) => sender_and_receiver,
            Err(e) => {
                warn!(
                    "Failed to create sender and receiciver for WGPU thread ({})",
                    e
                );
                return None;
            },
        };
        let sender_clone = sender.clone();

        if let Err(e) = std::thread::Builder::new()
            .name("WGPU".to_owned())
            .spawn(move || {
                WGPU::new(receiver, sender_clone).run();
            })
        {
            warn!("Failed to spwan WGPU thread ({})", e);
            return None;
        }
        Some(WebGPU(sender))
    }

    pub fn exit(&self, sender: IpcSender<()>) -> Result<(), &'static str> {
        self.0
            .send(WebGPURequest::Exit(sender))
            .map_err(|_| "Failed to send Exit message")
    }
}

struct WGPU {
    receiver: IpcReceiver<WebGPURequest>,
    sender: IpcSender<WebGPURequest>,
    global: wgpu::hub::Global<()>,
    adapters: Vec<WebGPUAdapter>,
    devices: Vec<WebGPUDevice>,
    // Track invalid adapters https://gpuweb.github.io/gpuweb/#invalid
    _invalid_adapters: Vec<WebGPUAdapter>,
}

impl WGPU {
    fn new(receiver: IpcReceiver<WebGPURequest>, sender: IpcSender<WebGPURequest>) -> Self {
        WGPU {
            receiver,
            sender,
            global: wgpu::hub::Global::new("wgpu-core"),
            adapters: Vec::new(),
            devices: Vec::new(),
            _invalid_adapters: Vec::new(),
        }
    }

    fn deinit(self) {
        self.global.delete()
    }

    fn run(mut self) {
        while let Ok(msg) = self.receiver.recv() {
            match msg {
                WebGPURequest::RequestAdapter(sender, options, ids) => {
                    let adapter_id = if let Some(pos) = self
                        .adapters
                        .iter()
                        .position(|adapter| ids.contains(&adapter.0))
                    {
                        self.adapters[pos].0
                    } else {
                        let adapter_id = match self.global.pick_adapter(
                            &options,
                            wgpu::instance::AdapterInputs::IdSet(&ids, |id| id.backend()),
                        ) {
                            Some(id) => id,
                            None => {
                                if let Err(e) =
                                    sender.send(Err("Failed to get webgpu adapter".to_string()))
                                {
                                    warn!(
                                        "Failed to send response to WebGPURequest::RequestAdapter ({})",
                                        e
                                    )
                                }
                                return;
                            },
                        };
                        adapter_id
                    };
                    let adapter = WebGPUAdapter(adapter_id);
                    self.adapters.push(adapter);
                    let global = &self.global;
                    let info = gfx_select!(adapter_id => global.adapter_get_info(adapter_id));
                    if let Err(e) = sender.send(Ok(WebGPUResponse::RequestAdapter(
                        info.name,
                        adapter,
                        WebGPU(self.sender.clone()),
                    ))) {
                        warn!(
                            "Failed to send response to WebGPURequest::RequestAdapter ({})",
                            e
                        )
                    }
                },
                WebGPURequest::RequestDevice(sender, adapter, descriptor, id) => {
                    let global = &self.global;
                    let id = gfx_select!(id => global.adapter_request_device(
                        adapter.0,
                        &descriptor,
                        id
                    ));
                    let device = WebGPUDevice(id);
                    // Note: (zakorgy) Note sure if sending the queue is needed at all,
                    // since wgpu-core uses the same id for the device and the queue
                    let queue = WebGPUQueue(id);
                    self.devices.push(device);
                    if let Err(e) =
                        sender.send(Ok(WebGPUResponse::RequestDevice(device, queue, descriptor)))
                    {
                        warn!(
                            "Failed to send response to WebGPURequest::RequestDevice ({})",
                            e
                        )
                    }
                },
                WebGPURequest::CreateBuffer(sender, device, id, descriptor) => {
                    let global = &self.global;
                    let buffer_id =
                        gfx_select!(id => global.device_create_buffer(device.0, &descriptor, id));
                    let buffer = WebGPUBuffer(buffer_id);
                    if let Err(e) = sender.send(buffer) {
                        warn!(
                            "Failed to send response to WebGPURequest::CreateBuffer ({})",
                            e
                        )
                    }
                },
                WebGPURequest::CreateBufferMapped(sender, device, id, descriptor) => {
                    let global = &self.global;
                    let buffer_size = descriptor.size as usize;

                    let (buffer_id, _arr_buff_ptr) = gfx_select!(id =>
                        global.device_create_buffer_mapped(device.0, &descriptor, id));
                    let buffer = WebGPUBuffer(buffer_id);

                    if let Err(e) = sender.send(buffer) {
                        warn!(
                            "Failed to send response to WebGPURequest::CreateBufferMapped ({})",
                            e
                        )
                    }
                },
                WebGPURequest::UnmapBuffer(device_id, buffer, usage, size, mut array_buffer) => {
                    let global = &self.global;
                    let on_write = move |status: wgpu::resource::BufferMapAsyncStatus,
                                         ptr: *mut u8| {
                        match status {
                            wgpu::resource::BufferMapAsyncStatus::Success => {
                                unsafe {
                                    std::ptr::copy(
                                        array_buffer.as_mut_ptr(),
                                        ptr,
                                        array_buffer.len(),
                                    );
                                };
                            },
                            _ => unimplemented!(),
                        }
                    };

                    gfx_select!(buffer.0 => global.buffer_map_async(
                        buffer.0,
                        wgpu::resource::BufferUsage::from_bits(usage).unwrap(),
                        0..size,
                        wgpu::resource::BufferMapOperation::Write(Box::new(on_write))
                    ));
                    gfx_select!(device_id => global.device_poll(device_id, true));
                    gfx_select!(buffer.0 => global.buffer_unmap(buffer.0));
                },
                WebGPURequest::DestroyBuffer(buffer) => {
                    let global = &self.global;
                    gfx_select!(buffer.0 => global.buffer_destroy(buffer.0));
                },
                WebGPURequest::CreateBindGroup(sender, device, id, layout_id, bindings) => {
                    let global = &self.global;
                    let descriptor = wgpu_core::binding_model::BindGroupDescriptor {
                        layout: layout_id.0,
                        bindings: bindings.as_ptr(),
                        bindings_length: bindings.len(),
                    };
                    let bg_id = gfx_select!(id => global.device_create_bind_group(device.0, &descriptor, id));
                    let bind_group = WebGPUBindGroup(bg_id);

                    if let Err(e) = sender.send(bind_group) {
                        warn!(
                            "Failed to send response to WebGPURequest::CreateBindGroup ({})",
                            e
                        )
                    }
                },
                WebGPURequest::CreateBindGroupLayout(sender, device, id, bindings) => {
                    let global = &self.global;
                    let descriptor = wgpu_core::binding_model::BindGroupLayoutDescriptor {
                        bindings: bindings.as_ptr(),
                        bindings_length: bindings.len(),
                    };
                    let bgl_id = gfx_select!(id => global.device_create_bind_group_layout(device.0, &descriptor, id));
                    let bgl = WebGPUBindGroupLayout(bgl_id);

                    if let Err(e) = sender.send(bgl) {
                        warn!(
                            "Failed to send response to WebGPURequest::CreateBindGroupLayout ({})",
                            e
                        )
                    }
                },
                WebGPURequest::CreatePipelineLayout(sender, device, id, bind_group_layouts) => {
                    let global = &self.global;
                    let descriptor = wgpu_core::binding_model::PipelineLayoutDescriptor {
                        bind_group_layouts: bind_group_layouts.as_ptr(),
                        bind_group_layouts_length: bind_group_layouts.len(),
                    };
                    let pl_id = gfx_select!(id => global.device_create_pipeline_layout(device.0, &descriptor, id));
                    let pipeline_layout = WebGPUPipelineLayout(pl_id);

                    if let Err(e) = sender.send(pipeline_layout) {
                        warn!(
                            "Failed to send response to WebGPURequest::CreatePipelineLayout ({})",
                            e
                        )
                    }
                },
                WebGPURequest::CreateShaderModule(sender, device, id, program) => {
                    let global = &self.global;
                    let descriptor = wgpu_core::pipeline::ShaderModuleDescriptor {
                        code: wgpu_core::U32Array {
                            bytes: program.as_ptr(),
                            length: program.len(),
                        },
                    };
                    let sm_id = gfx_select!(id => global.device_create_shader_module(device.0, &descriptor, id));
                    let shader_module = WebGPUShaderModule(sm_id);

                    if let Err(e) = sender.send(shader_module) {
                        warn!(
                            "Failed to send response to WebGPURequest::CreateShaderModule ({})",
                            e
                        )
                    }
                },
                WebGPURequest::CreateComputePipeline(
                    sender,
                    device,
                    id,
                    layout,
                    program,
                    entry,
                ) => {
                    let global = &self.global;
                    let descriptor = wgpu_core::pipeline::ComputePipelineDescriptor {
                        layout,
                        compute_stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                            module: program,
                            entry_point: entry.as_ptr(),
                        },
                    };
                    let cp_id = gfx_select!(id => global.device_create_compute_pipeline(device.0, &descriptor, id));
                    let compute_pipeline = WebGPUComputePipeline(cp_id);

                    if let Err(e) = sender.send(compute_pipeline) {
                        warn!(
                            "Failed to send response to WebGPURequest::CreateComputePipeline ({})",
                            e
                        )
                    }
                },
                WebGPURequest::CreateCommandEncoder(sender, device, id) => {
                    let global = &self.global;
                    let id = gfx_select!(id => global.device_create_command_encoder(device.0, &Default::default(), id));
                    if let Err(e) = sender.send(WebGPUCommandEncoder(id)) {
                        warn!(
                            "Failed to send response to WebGPURequest::CreateBuffer ({})",
                            e
                        )
                    }
                },
                WebGPURequest::CopyBuffer(
                    command_encoder_id,
                    source,
                    source_offset,
                    destination,
                    destination_offset,
                    size,
                ) => {
                    let global = &self.global;
                    let _ = gfx_select!(command_encoder_id => global.command_encoder_copy_buffer_to_buffer(
                        command_encoder_id,
                        source,
                        source_offset,
                        destination,
                        destination_offset,
                        size
                    ));
                },
                WebGPURequest::CommandEncoderFinish(sender, command_encoder_id) => {
                    let global = &self.global;
                    let command_buffer_id = gfx_select!(command_encoder_id => global.command_encoder_finish(
                        command_encoder_id,
                        &wgpu::command::CommandBufferDescriptor::default()
                    ));
                    if let Err(e) = sender.send(WebGPUCommandBuffer(command_buffer_id)) {
                        warn!(
                            "Failed to send response to WebGPURequest::CommandEncoderFinish ({})",
                            e
                        )
                    }
                },
                WebGPURequest::Submit(queue_id, command_buffer_ids) => {
                    let global = &self.global;
                    let _ = gfx_select!(queue_id => global.queue_submit(
                        queue_id,
                        &command_buffer_ids
                    ));
                },
                WebGPURequest::RunComputePass(sender, command_encoder_id, commands) => {
                    let hub = match command_encoder_id.backend() {
                        #[cfg(any(
                            not(any(target_os = "ios", target_os = "macos")),
                            feature = "gfx-backend-vulkan"
                        ))]
                        wgpu::Backend::Vulkan => wgpu::backend::Vulkan::hub(&self.global),
                        #[cfg(any(target_os = "ios", target_os = "macos"))]
                        wgpu::Backend::Metal => wgpu::backend::Metal::hub(&self.global),
                        #[cfg(windows)]
                        wgpu::Backend::Dx12 => wgpu::backend::Dx12::hub(&self.global),
                        #[cfg(windows)]
                        wgpu::Backend::Dx11 => wgpu::backend::Dx11::hub(&self.global),
                        _ => return warn!("unsupported backend"),
                    };
                    let mut token = Token::root();

                    let (mut cmb_guard, mut token) = hub.command_buffers.write(&mut token);
                    let cmb = &mut cmb_guard[command_encoder_id];
                    let mut binder = Binder::new(cmb.max_bind_groups());

                    let (pipeline_layout_guard, mut token) = hub.pipeline_layouts.read(&mut token);
                    let (bind_group_guard, mut token) = hub.bind_groups.read(&mut token);
                    let (pipeline_guard, mut token) = hub.compute_pipelines.read(&mut token);
                    let (buffer_guard, mut token) = hub.buffers.read(&mut token);
                    let (texture_guard, _) = hub.textures.read(&mut token);

                    for command in commands {
                        match command {
                            ComputeCommand::Dispatch(groups) => {
                                if let Err(e) = unsafe { cmb.dispatch(groups) } {
                                    return warn!("({:?})", e);
                                }
                            },
                            ComputeCommand::SetBindGroup {
                                index,
                                bind_group_id,
                                dynamic_offsets,
                            } => {
                                if let Err(e) = unsafe {
                                    cmb.set_bind_group(
                                        command_encoder_id,
                                        index,
                                        bind_group_id,
                                        &dynamic_offsets,
                                        &*bind_group_guard,
                                        &*buffer_guard,
                                        &*texture_guard,
                                        &*pipeline_layout_guard,
                                        &mut binder,
                                    )
                                } {
                                    return warn!("({:?})", e);
                                }
                            },
                            ComputeCommand::SetComputePipeline(pipeline_id) => {
                                if let Err(e) = unsafe {
                                    cmb.set_compute_pipeline(
                                        pipeline_id,
                                        &*bind_group_guard,
                                        &*pipeline_guard,
                                        &*pipeline_layout_guard,
                                        &mut binder,
                                    )
                                } {
                                    return warn!("({:?})", e);
                                }
                            },
                            ComputeCommand::End => break,
                        }
                    }
                    if let Err(e) = sender.send(()) {
                        warn!("Failed to send response to WebGPURequest::Exit ({})", e)
                    }
                },
                WebGPURequest::MapReadAsync(sender, buffer_id, device_id, usage, size) => {
                    let global = &self.global;
                    let on_read = move |status: wgpu::resource::BufferMapAsyncStatus,
                                        ptr: *const u8| {
                        match status {
                            wgpu::resource::BufferMapAsyncStatus::Success => {
                                let mut array_buffer = Vec::with_capacity(size as usize);
                                unsafe {
                                    array_buffer.set_len(size as usize);
                                    std::ptr::copy(ptr, array_buffer.as_mut_ptr(), size as usize);
                                };
                                if let Err(e) =
                                    sender.send(Ok(WebGPUResponse::MapReadAsync(array_buffer)))
                                {
                                    warn!(
                                        "Failed to send response to WebGPURequest::MapReadAsync ({})",
                                        e
                                    )
                                }
                            },
                            _ => unimplemented!(),
                        }
                    };
                    gfx_select!(buffer_id => global.buffer_map_async(
                        buffer_id,
                        wgpu::resource::BufferUsage::from_bits(usage).unwrap(),
                        0..size,
                        wgpu::resource::BufferMapOperation::Read(Box::new(on_read))
                    ));
                    gfx_select!(device_id => global.device_poll(device_id, true));
                },
                WebGPURequest::Exit(sender) => {
                    self.deinit();
                    if let Err(e) = sender.send(()) {
                        warn!("Failed to send response to WebGPURequest::Exit ({})", e)
                    }
                    return;
                },
            }
        }
    }
}

macro_rules! webgpu_resource {
    ($name:ident, $id:ty) => {
        #[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
        pub struct $name(pub $id);

        impl MallocSizeOf for $name {
            fn size_of(&self, _ops: &mut MallocSizeOfOps) -> usize {
                0
            }
        }

        impl Eq for $name {}
    };
}

webgpu_resource!(WebGPUAdapter, wgpu::id::AdapterId);
webgpu_resource!(WebGPUDevice, wgpu::id::DeviceId);
webgpu_resource!(WebGPUBuffer, wgpu::id::BufferId);
webgpu_resource!(WebGPUBindGroup, wgpu::id::BindGroupId);
webgpu_resource!(WebGPUBindGroupLayout, wgpu::id::BindGroupLayoutId);
webgpu_resource!(WebGPUComputePipeline, wgpu::id::ComputePipelineId);
webgpu_resource!(WebGPUPipelineLayout, wgpu::id::PipelineLayoutId);
webgpu_resource!(WebGPUShaderModule, wgpu::id::ShaderModuleId);
webgpu_resource!(WebGPUCommandEncoder, wgpu::id::CommandEncoderId);
webgpu_resource!(WebGPUCommandBuffer, wgpu::id::CommandBufferId);
webgpu_resource!(WebGPUQueue, wgpu::id::QueueId);
