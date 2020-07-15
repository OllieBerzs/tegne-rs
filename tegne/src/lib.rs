// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Tegne is a Vulkan rendering engine

#![feature(proc_macro_hygiene, drain_filter, thread_id_value)]
#![warn(
    rust_2018_idioms,
    unused,
    future_incompatible,
    // missing_docs,
    single_use_lifetimes,
    unused_qualifications,
)]

// should be imported first
mod macros;

mod buffer;
mod context;
mod device;
mod error;
mod font;
mod image;
mod instance;
mod pipeline;
mod renderer;
mod resource;
mod surface;
mod sync;

pub mod camera;
pub mod color;
pub mod math;
pub mod mesh;

#[cfg(feature = "window")]
pub mod window;

#[cfg(feature = "ui")]
pub mod ui;

#[cfg(feature = "profiler")]
pub mod profiler;

pub mod shader {
    pub use super::pipeline::*;
    pub use super::renderer::SamplerOptions;
}

pub mod reference {
    pub type Mesh = super::resource::Ref<super::mesh::Mesh>;
    pub type Material = super::resource::Ref<super::pipeline::Material>;
    pub type Texture = super::resource::Ref<super::image::Texture>;
    pub type Shader = super::resource::Ref<super::pipeline::Shader>;
    pub type Framebuffer = super::resource::Ref<super::image::Framebuffer>;
}

pub use context::Context;
pub use context::ContextOptions;
pub use renderer::Target;
