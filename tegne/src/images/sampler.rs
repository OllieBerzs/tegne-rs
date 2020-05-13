use ash::version::DeviceV1_0;
use ash::vk::BorderColor;
use ash::vk::CompareOp;
use ash::vk::Filter;
use ash::vk::Sampler as VkSampler;
use ash::vk::SamplerAddressMode;
use ash::vk::SamplerCreateInfo;
use ash::vk::SamplerMipmapMode;
use std::rc::Rc;
use std::rc::Weak;

use crate::instance::Device;
use crate::utils::OrError;

pub(crate) struct Sampler {
    vk: VkSampler,
    device: Weak<Device>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct SamplerOptions {
    pub(crate) anisotropy: f32,
    pub(crate) address: SamplerAddress,
    pub(crate) filter: SamplerFilter,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum SamplerFilter {
    Linear,
    Nearest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum SamplerAddress {
    Repeat,
    Clamp,
}

impl Sampler {
    pub(crate) fn new(device: &Rc<Device>, options: SamplerOptions) -> Self {
        let info = SamplerCreateInfo::builder()
            .mag_filter(options.filter.flag())
            .min_filter(options.filter.flag())
            .address_mode_u(options.address.flag())
            .address_mode_v(options.address.flag())
            .address_mode_w(options.address.flag())
            .anisotropy_enable(options.anisotropy != 0.0)
            .max_anisotropy(options.anisotropy)
            .border_color(BorderColor::FLOAT_OPAQUE_WHITE)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(CompareOp::ALWAYS)
            .mipmap_mode(SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(16.0);

        let vk = unsafe {
            device
                .logical()
                .create_sampler(&info, None)
                .or_error("cannot create sampler")
        };

        Self {
            vk,
            device: Rc::downgrade(device),
        }
    }

    pub(crate) fn vk(&self) -> VkSampler {
        self.vk
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            self.device().logical().destroy_sampler(self.vk, None);
        }
    }
}

impl Default for SamplerOptions {
    fn default() -> Self {
        Self {
            anisotropy: 0.0,
            address: SamplerAddress::Repeat,
            filter: SamplerFilter::Linear,
        }
    }
}

impl SamplerAddress {
    pub(crate) fn flag(&self) -> SamplerAddressMode {
        match *self {
            Self::Clamp => SamplerAddressMode::CLAMP_TO_BORDER,
            Self::Repeat => SamplerAddressMode::REPEAT,
        }
    }
}

impl SamplerFilter {
    pub(crate) fn flag(&self) -> Filter {
        match *self {
            Self::Linear => Filter::LINEAR,
            Self::Nearest => Filter::NEAREST,
        }
    }
}
