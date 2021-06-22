use ash::vk;

use rust_engine_3d::vulkan_context::render_pass::RenderPassDataCreateInfo;

use crate::effect::effect_data::{ ParticleBlendMode, ParticleGeometryType };
use crate::renderer::project_renderer::RenderObjectType;
use crate::renderer::project_renderer::ProjectRenderer;
use crate::render_pass::{
    common,
    effect,
    fft_ocean,
    precomputed_atmosphere,
};

pub fn get_render_pass_data_create_infos(project_renderer: &ProjectRenderer) -> Vec<RenderPassDataCreateInfo> {
    vec![
        common::clear_render_target::get_render_pass_data_create_info(project_renderer, &[vk::Format::R16G16B16A16_SFLOAT], vk::Format::UNDEFINED),
        common::clear_render_target::get_render_pass_data_create_info(project_renderer, &[vk::Format::R32_SFLOAT], vk::Format::UNDEFINED),
        common::clear_render_target::get_render_pass_data_create_info(project_renderer, &[vk::Format::R32G32B32A32_SFLOAT], vk::Format::UNDEFINED),
        common::clear_render_target::get_render_pass_data_create_info(project_renderer, &[vk::Format::R16G16B16A16_SFLOAT], vk::Format::D32_SFLOAT),
        common::clear_render_target::get_render_pass_data_create_info(project_renderer, &[], vk::Format::D32_SFLOAT),
        common::clear_render_target::get_render_pass_data_create_info(
            project_renderer,
            &[vk::Format::R8G8B8A8_UNORM, vk::Format::R8G8B8A8_UNORM, vk::Format::R8G8B8A8_UNORM, vk::Format::R16G16_SFLOAT],
            vk::Format::D32_SFLOAT
        ),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_gbuffer"),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_shadow"),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_capture_height_map"),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_light_probe_depth_0"),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_light_probe_depth_1"),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_light_probe_depth_2"),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_light_probe_depth_3"),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_light_probe_depth_4"),
        common::clear_framebuffer::get_render_pass_data_create_info(project_renderer, "clear_light_probe_depth_5"),
        common::composite_gbuffer::get_render_pass_data_create_info(project_renderer),
        common::copy_cube_map::get_render_pass_data_create_info(project_renderer),
        common::downsampling::get_render_pass_data_create_info(project_renderer),
        common::generate_min_z::get_render_pass_data_create_info(project_renderer),
        common::render_bloom::get_render_pass_data_create_info(project_renderer),
        common::render_copy::get_render_pass_data_create_info(project_renderer),
        common::render_color::get_render_pass_data_create_info(project_renderer, vk::Format::R16G16B16A16_SFLOAT),
        common::render_color::get_render_pass_data_create_info(project_renderer, vk::Format::R32_SFLOAT),
        common::render_color::get_render_pass_data_create_info(project_renderer, vk::Format::R32G32B32A32_SFLOAT),
        common::render_debug::get_render_pass_data_create_info(project_renderer),
        common::render_font::get_render_pass_data_create_info(project_renderer),
        common::render_final::get_render_pass_data_create_info(project_renderer),
        common::render_gaussian_blur::get_render_pass_data_create_info(project_renderer),
        common::render_motion_blur::get_render_pass_data_create_info(project_renderer),
        common::render_gbuffer::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal),
        common::render_gbuffer::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static),
        common::render_forward::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal),
        common::render_forward::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static, 0),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static, 1),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static, 2),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static, 3),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static, 4),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static, 5),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal, 0),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal, 1),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal, 2),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal, 3),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal, 4),
        common::render_forward_for_light_probe::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal, 5),
        common::render_shadow::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal),
        common::render_shadow::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static),
        common::capture_height_map::get_render_pass_data_create_info(project_renderer, RenderObjectType::Skeletal),
        common::capture_height_map::get_render_pass_data_create_info(project_renderer, RenderObjectType::Static),
        common::render_ssao::get_render_pass_data_create_info(project_renderer),
        common::render_ssao_blur::get_render_pass_data_create_info(project_renderer),
        common::render_ssr::get_render_pass_data_create_info(project_renderer),
        common::render_ssr_resolve::get_render_pass_data_create_info(project_renderer),
        common::render_taa::get_render_pass_data_create_info(project_renderer),
        common::render_ui::get_render_pass_data_create_info(project_renderer),
        effect::process_gpu_particle::get_render_pass_data_create_info(project_renderer),
        effect::render_particle_translucent::get_render_pass_data_create_info(project_renderer, ParticleBlendMode::AlphaBlend, ParticleGeometryType::Quad),
        fft_ocean::render_fft_init::get_render_pass_data_create_info(project_renderer),
        fft_ocean::render_fft_ocean::get_render_pass_data_create_info(project_renderer),
        fft_ocean::render_fft_variance::get_render_pass_data_create_info(project_renderer),
        fft_ocean::render_fft_waves::get_render_pass_data_create_info(project_renderer),
        precomputed_atmosphere::composite_atmosphere::get_render_pass_data_create_info(project_renderer),
        precomputed_atmosphere::compute_transmittance::get_render_pass_data_create_info(project_renderer),
        precomputed_atmosphere::compute_direct_irradiance::get_render_pass_data_create_info(project_renderer),
        precomputed_atmosphere::compute_indirect_irradiance::get_render_pass_data_create_info(project_renderer),
        precomputed_atmosphere::compute_multiple_scattering::get_render_pass_data_create_info(project_renderer),
        precomputed_atmosphere::compute_single_scattering::get_render_pass_data_create_info(project_renderer),
        precomputed_atmosphere::compute_scattering_density::get_render_pass_data_create_info(project_renderer),
        precomputed_atmosphere::render_atmosphere::get_render_pass_data_create_info(project_renderer),
    ]
}