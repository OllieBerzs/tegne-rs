use ash::version::DeviceV1_0;
use ash::vk::AccessFlags;
use ash::vk::PipelineBindPoint;
use ash::vk::PipelineStageFlags;
use ash::vk::RenderPass as VkRenderPass;
use ash::vk::RenderPassCreateInfo;
use ash::vk::SubpassDependency;
use ash::vk::SubpassDescription;
use ash::vk::SUBPASS_EXTERNAL;
use std::rc::Rc;
use std::rc::Weak;

use super::Attachment;
use super::AttachmentOptions;
use crate::images::ImageLayout;
use crate::instance::Device;
use crate::utils::OrError;

pub(crate) struct RenderPass {
    vk: VkRenderPass,
    has_msaa_attachment: bool,
    device: Weak<Device>,
}

#[derive(Default)]
struct RenderPassOptions {
    depth_attachment: Option<Attachment>,
    color_attachment: Option<Attachment>,
    msaa_attachment: Option<Attachment>,
    dependency: Option<SubpassDependency>,
}

impl RenderPass {
    pub(crate) fn window(device: &Rc<Device>) -> Self {
        let mut options = RenderPassOptions::default();

        // depth
        options.depth_attachment = Some(Attachment::new(
            device,
            AttachmentOptions {
                index: 0,
                layout: ImageLayout::Depth,
                has_samples: true,
                has_clear: true,
                ..Default::default()
            },
        ));

        // color
        options.color_attachment = Some(Attachment::new(
            device,
            AttachmentOptions {
                index: 1,
                layout: ImageLayout::Present,
                has_clear: !device.is_msaa(),
                has_store: true,
                ..Default::default()
            },
        ));

        // msaa
        if device.is_msaa() {
            options.msaa_attachment = Some(Attachment::new(
                device,
                AttachmentOptions {
                    index: 2,
                    layout: ImageLayout::Color,
                    has_clear: true,
                    has_samples: true,
                    ..Default::default()
                },
            ));
        }

        options.dependency = Some(
            SubpassDependency::builder()
                .src_subpass(SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(AccessFlags::empty())
                .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(
                    AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
                )
                .build(),
        );

        Self::new(device, options)
    }

    pub(crate) fn color(device: &Rc<Device>) -> Self {
        let mut options = RenderPassOptions::default();

        // depth
        options.depth_attachment = Some(Attachment::new(
            device,
            AttachmentOptions {
                index: 0,
                layout: ImageLayout::Depth,
                has_clear: true,
                has_samples: true,
                ..Default::default()
            },
        ));

        // color
        options.color_attachment = Some(Attachment::new(
            device,
            AttachmentOptions {
                index: 1,
                layout: ImageLayout::Color,
                has_clear: !device.is_msaa(),
                has_store: true,
                ..Default::default()
            },
        ));

        // msaa
        if device.is_msaa() {
            options.msaa_attachment = Some(Attachment::new(
                device,
                AttachmentOptions {
                    index: 2,
                    layout: ImageLayout::Color,
                    has_clear: true,
                    has_samples: true,
                    ..Default::default()
                },
            ));
        }

        options.dependency = Some(
            SubpassDependency::builder()
                .src_subpass(SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(AccessFlags::empty())
                .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(
                    AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
                )
                .build(),
        );

        Self::new(device, options)
    }

    pub(crate) fn depth(device: &Rc<Device>) -> Self {
        let mut options = RenderPassOptions::default();

        // depth
        options.depth_attachment = Some(Attachment::new(
            device,
            AttachmentOptions {
                index: 0,
                layout: ImageLayout::Depth,
                has_clear: true,
                has_store: true,
                ..Default::default()
            },
        ));

        options.dependency = Some(
            SubpassDependency::builder()
                .src_subpass(0)
                .dst_subpass(SUBPASS_EXTERNAL)
                .src_stage_mask(
                    PipelineStageFlags::EARLY_FRAGMENT_TESTS
                        | PipelineStageFlags::LATE_FRAGMENT_TESTS,
                )
                .src_access_mask(AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
                .dst_stage_mask(PipelineStageFlags::TRANSFER)
                .dst_access_mask(AccessFlags::TRANSFER_READ)
                .build(),
        );

        Self::new(device, options)
    }

    fn new(device: &Rc<Device>, options: RenderPassOptions) -> Self {
        let dependencies = [options.dependency.expect("subpass dependency not set")];
        let mut attachments = vec![];
        let mut subpass_builder =
            SubpassDescription::builder().pipeline_bind_point(PipelineBindPoint::GRAPHICS);

        // depth
        let depth_attachment;
        if let Some(attach) = &options.depth_attachment {
            depth_attachment = attach.reference();
            attachments.push(attach.vk());
            subpass_builder = subpass_builder.depth_stencil_attachment(&depth_attachment);
        }

        // color
        let mut color_attachments = vec![];
        if let Some(attach) = &options.color_attachment {
            attachments.push(attach.vk());
            color_attachments.push(attach.reference());
        }

        // resolve
        let mut msaa_attachments = vec![];
        if let Some(attach) = &options.msaa_attachment {
            attachments.push(attach.vk());
            msaa_attachments.push(attach.reference());
        }

        let has_msaa_attachment = options.msaa_attachment.is_some();

        subpass_builder = if has_msaa_attachment {
            subpass_builder
                .color_attachments(&msaa_attachments)
                .resolve_attachments(&color_attachments)
        } else {
            subpass_builder.color_attachments(&color_attachments)
        };

        let subpasses = [subpass_builder.build()];

        let info = RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        let vk = unsafe {
            device
                .logical()
                .create_render_pass(&info, None)
                .or_error("cannot create render pass")
        };

        Self {
            vk,
            has_msaa_attachment,
            device: Rc::downgrade(device),
        }
    }

    pub(crate) fn has_msaa_attachment(&self) -> bool {
        self.has_msaa_attachment
    }

    pub(crate) fn vk(&self) -> VkRenderPass {
        self.vk
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device().logical().destroy_render_pass(self.vk, None);
        }
    }
}
