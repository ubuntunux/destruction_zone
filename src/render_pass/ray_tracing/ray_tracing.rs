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
            _pipeline_ray_tracing_shader_file: PathBuf::from("ray_tracing/triangle.rgen"),
            _pipeline_bind_point: vk::PipelineBindPoint::RAY_TRACING_KHR,
            _descriptor_data_create_infos: vec![],
            ..Default::default()
        }
    ];

    RenderPassDataCreateInfo {
        _render_pass_create_info_name: render_pass_name.clone(),
        _pipeline_data_create_infos: pipeline_data_create_infos,
        ..Default::default()
    }
}