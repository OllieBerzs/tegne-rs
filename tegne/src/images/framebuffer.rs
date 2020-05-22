use ash::version::DeviceV1_0;
use ash::vk::Filter;
use ash::vk::Framebuffer as VkFramebuffer;
use ash::vk::FramebufferCreateInfo;
use ash::vk::ImageAspectFlags;
use ash::vk::ImageBlit;
use ash::vk::ImageSubresourceLayers;
use ash::vk::Offset3D;
use log::debug;
use log::warn;
use std::cmp;
use std::sync::Arc;

use super::Image;
use super::ImageFormat;
use super::ImageLayout;
use super::ImageOptions;
use super::ImageUsage;
use crate::error::Result;
use crate::instance::Commands;
use crate::instance::Device;
use crate::instance::LayoutChangeOptions;
use crate::instance::Swapchain;
use crate::shaders::ImageUniforms;
use crate::shaders::RenderPass;
use crate::shaders::RenderPasses;
use crate::shaders::ShaderLayout;
use crate::shaders::WorldUniforms;

pub struct Framebuffer {
    vk: VkFramebuffer,
    width: u32,
    height: u32,
    attachment_images: Vec<Image>,
    shader_image: Option<Image>,
    shader_index: Option<i32>,
    world_uniforms: WorldUniforms,
    device: Arc<Device>,
}

impl Framebuffer {
    pub(crate) fn window(
        device: &Arc<Device>,
        swapchain: &Swapchain,
        render_passes: &RenderPasses,
        shader_layout: &ShaderLayout,
    ) -> Result<Vec<Self>> {
        debug!("creating window framebuffers");

        let extent = device.extent();
        let render_pass = render_passes.window();

        swapchain
            .iter_images()?
            .map(|img| {
                let mut images = vec![];

                // depth
                images.push(Image::new(
                    device,
                    ImageOptions {
                        width: extent.width,
                        height: extent.height,
                        format: ImageFormat::Depth,
                        usage: &[ImageUsage::Depth],
                        has_view: true,
                        has_samples: true,
                        ..Default::default()
                    },
                )?);

                // color
                images.push(Image::new(
                    device,
                    ImageOptions {
                        image: Some(img),
                        width: extent.width,
                        height: extent.height,
                        format: ImageFormat::Bgra,
                        has_view: true,
                        ..Default::default()
                    },
                )?);

                // msaa
                if device.is_msaa() {
                    images.push(Image::new(
                        device,
                        ImageOptions {
                            width: extent.width,
                            height: extent.height,
                            format: ImageFormat::Bgra,
                            usage: &[ImageUsage::Color, ImageUsage::Transient],
                            has_view: true,
                            has_samples: true,
                            ..Default::default()
                        },
                    )?);
                }

                let vk =
                    create_framebuffer(device, render_pass, &images, extent.width, extent.height)?;

                let world_uniforms = WorldUniforms::new(device, shader_layout)?;

                Ok(Self {
                    vk,
                    width: extent.width,
                    height: extent.height,
                    shader_image: None,
                    shader_index: None,
                    attachment_images: images,
                    world_uniforms,
                    device: Arc::clone(device),
                })
            })
            .collect::<Result<Vec<_>>>()
    }

    pub(crate) fn color(
        device: &Arc<Device>,
        render_passes: &RenderPasses,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let mut images = vec![];
        let render_pass = render_passes.color();

        // depth
        images.push(Image::new(
            device,
            ImageOptions {
                width,
                height,
                format: ImageFormat::Depth,
                usage: &[ImageUsage::Depth],
                has_view: true,
                has_samples: true,
                ..Default::default()
            },
        )?);

        // color
        images.push(Image::new(
            device,
            ImageOptions {
                width,
                height,
                format: ImageFormat::Bgra,
                usage: &[ImageUsage::Color, ImageUsage::TransferSrc],
                has_view: true,
                ..Default::default()
            },
        )?);

        // msaa
        if device.is_msaa() {
            images.push(Image::new(
                device,
                ImageOptions {
                    width,
                    height,
                    format: ImageFormat::Bgra,
                    usage: &[ImageUsage::Color, ImageUsage::Transient],
                    has_view: true,
                    has_samples: true,
                    ..Default::default()
                },
            )?);
        }

        let (shader_image, shader_index) =
            create_shader_image(device, image_uniforms, width, height, ImageFormat::Bgra)?;

        let vk = create_framebuffer(device, render_pass, &images, width, height)?;

        let world_uniforms = WorldUniforms::new(device, shader_layout)?;

        Ok(Self {
            vk,
            width,
            height,
            shader_image: Some(shader_image),
            shader_index: Some(shader_index),
            attachment_images: images,
            world_uniforms,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn depth(
        device: &Arc<Device>,
        render_passes: &RenderPasses,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let mut images = vec![];
        let render_pass = render_passes.depth();

        // depth
        images.push(Image::new(
            device,
            ImageOptions {
                width,
                height,
                format: ImageFormat::Depth,
                usage: &[ImageUsage::Depth, ImageUsage::TransferSrc],
                has_stencil: true,
                has_view: true,
                ..Default::default()
            },
        )?);

        let (shader_image, shader_index) =
            create_shader_image(device, image_uniforms, width, height, ImageFormat::Depth)?;

        let vk = create_framebuffer(device, render_pass, &images, width, height)?;

        let world_uniforms = WorldUniforms::new(device, shader_layout)?;

        Ok(Self {
            vk,
            width,
            height,
            shader_image: Some(shader_image),
            shader_index: Some(shader_index),
            attachment_images: images,
            world_uniforms,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn blit_to_shader_image(&self, cmd: &Commands) {
        if let Some(shader_image) = &self.shader_image {
            let image = &self.attachment_images[cmp::min(self.attachment_images.len() - 1, 1)];
            let is_depth = image.is_depth_format();

            if is_depth {
                cmd.change_image_layout(
                    image,
                    LayoutChangeOptions {
                        old_layout: ImageLayout::Depth,
                        new_layout: ImageLayout::TransferSrc,
                        ..Default::default()
                    },
                );
            } else {
                cmd.change_image_layout(
                    image,
                    LayoutChangeOptions {
                        old_layout: ImageLayout::Color,
                        new_layout: ImageLayout::TransferSrc,
                        ..Default::default()
                    },
                );
            }
            cmd.change_image_layout(
                shader_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::Shader,
                    new_layout: ImageLayout::TransferDst,
                    ..Default::default()
                },
            );

            let offsets = [
                Offset3D::default(),
                Offset3D {
                    x: self.width as i32,
                    y: self.height as i32,
                    z: 1,
                },
            ];
            let aspect_mask = if is_depth {
                ImageAspectFlags::DEPTH
            } else {
                ImageAspectFlags::COLOR
            };
            let subresource = ImageSubresourceLayers::builder()
                .aspect_mask(aspect_mask)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
                .build();

            let blit = ImageBlit::builder()
                .src_offsets(offsets)
                .src_subresource(subresource)
                .dst_offsets(offsets)
                .dst_subresource(subresource)
                .build();

            let filter = if is_depth {
                Filter::NEAREST
            } else {
                Filter::LINEAR
            };

            cmd.blit_image(image.vk(), shader_image.vk(), blit, filter);

            if is_depth {
                cmd.change_image_layout(
                    image,
                    LayoutChangeOptions {
                        old_layout: ImageLayout::TransferSrc,
                        new_layout: ImageLayout::Depth,
                        ..Default::default()
                    },
                );
            } else {
                cmd.change_image_layout(
                    image,
                    LayoutChangeOptions {
                        old_layout: ImageLayout::TransferSrc,
                        new_layout: ImageLayout::Color,
                        ..Default::default()
                    },
                );
            }
            cmd.change_image_layout(
                shader_image,
                LayoutChangeOptions {
                    old_layout: ImageLayout::TransferDst,
                    new_layout: ImageLayout::Shader,
                    ..Default::default()
                },
            );
        } else {
            warn!("trying to blit framebuffer without a shader image");
        }
    }

    pub(crate) fn vk(&self) -> VkFramebuffer {
        self.vk
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn image_index(&self) -> i32 {
        self.shader_index.unwrap_or(0)
    }

    pub(crate) fn iter_attachments(&self) -> impl Iterator<Item = &Image> {
        self.attachment_images.iter()
    }

    pub(crate) fn world_uniforms(&self) -> &WorldUniforms {
        &self.world_uniforms
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_framebuffer(self.vk, None);
        }
    }
}

impl PartialEq for Framebuffer {
    fn eq(&self, other: &Self) -> bool {
        self.shader_image.as_ref().map(|i| i.vk()) == other.shader_image.as_ref().map(|i| i.vk())
    }
}

fn create_shader_image(
    device: &Arc<Device>,
    uniforms: &ImageUniforms,
    width: u32,
    height: u32,
    format: ImageFormat,
) -> Result<(Image, i32)> {
    let image = Image::new(
        device,
        ImageOptions {
            width,
            height,
            format,
            usage: &[ImageUsage::Sampled, ImageUsage::TransferDst],
            has_view: true,
            ..Default::default()
        },
    )?;
    let cmd = Commands::new(device)?;
    cmd.begin()?;
    cmd.change_image_layout(
        &image,
        LayoutChangeOptions {
            new_layout: ImageLayout::Shader,
            ..Default::default()
        },
    );
    device.submit_and_wait(cmd.end()?)?;
    let index = uniforms.image_count() as i32;
    if let Some(view) = image.view() {
        uniforms.add(view);
    }
    Ok((image, index))
}

fn create_framebuffer(
    device: &Arc<Device>,
    render_pass: &RenderPass,
    images: &[Image],
    width: u32,
    height: u32,
) -> Result<VkFramebuffer> {
    let attachments = images.iter().filter_map(|i| i.view()).collect::<Vec<_>>();

    let info = FramebufferCreateInfo::builder()
        .render_pass(render_pass.vk())
        .attachments(&attachments)
        .width(width)
        .height(height)
        .layers(1);

    Ok(unsafe { device.logical().create_framebuffer(&info, None)? })
}
