use std::path::PathBuf;

use ash::vk;
use rust_engine_3d::utilities::system::enum_to_string;
use rust_engine_3d::vulkan_context::render_pass::{
    RenderPassDataCreateInfo,
    PipelineDataCreateInfo,
};
use rust_engine_3d::vulkan_context::descriptor::{
    DescriptorDataCreateInfo,
    DescriptorResourceType,
};

use crate::renderer::render_target::RenderTargetType;
use crate::renderer::project_renderer::ProjectRenderer;

pub fn get_render_pass_data_create_info(_project_renderer: &ProjectRenderer) -> RenderPassDataCreateInfo {
    let render_pass_name = String::from("ray_tracing");
    let pipeline_data_create_infos = vec![
        PipelineDataCreateInfo {
            _pipeline_data_create_info_name: String::from("ray_tracing"),
            _pipeline_ray_generation_shader_file: PathBuf::from("ray_tracing/triangle.rgen"),
            _pipeline_ray_closet_hit_shader_file: PathBuf::from("ray_tracing/triangle.rchit"),
            _pipeline_ray_miss_shader_file: PathBuf::from("ray_tracing/triangle.rmiss"),
            _pipeline_bind_point: vk::PipelineBindPoint::RAY_TRACING_KHR,
            _descriptor_data_create_infos: vec![
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 0,
                    _descriptor_name: "Top-Level AccelerationStructure".to_string(),
                    _descriptor_resource_type: DescriptorResourceType::AccelerationStructure,
                    _descriptor_shader_stage: vk::ShaderStageFlags::RAYGEN_NV,
                    ..Default::default()
                },
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 1,
                    _descriptor_name: enum_to_string(&RenderTargetType::SceneColor),
                    _descriptor_resource_type: DescriptorResourceType::StorageRenderTarget,
                    _descriptor_shader_stage: vk::ShaderStageFlags::RAYGEN_NV,
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    ];

    RenderPassDataCreateInfo {
        _render_pass_create_info_name: render_pass_name.clone(),
        _pipeline_data_create_infos: pipeline_data_create_infos,
        ..Default::default()
    }
}