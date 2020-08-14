// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ResourceManager - resource manager

mod builtin;
mod index;
mod storage;

pub(crate) mod hash;

use std::collections::HashMap;

use crate::error::Result;
use crate::font::Font;
use crate::image::Framebuffer;
use crate::image::Texture;
use crate::mesh::CoreMesh;
use crate::pipeline::ImageUniform;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use storage::Storage;

pub(crate) use builtin::Builtins;
pub(crate) use index::Index;
pub use storage::Ref;

pub(crate) struct ResourceManager {
    textures: Vec<Storage<Texture>>,
    materials: Vec<Storage<Material>>,
    shaders: Vec<Storage<Shader>>,
    fonts: Vec<Storage<Font>>,
    framebuffers: Vec<Storage<Framebuffer>>,

    meshes: HashMap<Index, CoreMesh>,
    next_index: u32,
}

impl ResourceManager {
    pub(crate) fn new() -> Self {
        Self {
            textures: vec![],
            materials: vec![],
            shaders: vec![],
            fonts: vec![],
            framebuffers: vec![],
            meshes: HashMap::new(),
            next_index: 0,
        }
    }

    pub(crate) fn add_texture(&mut self, texture: Texture) -> Ref<Texture> {
        let storage = Storage::new(texture);
        let reference = storage.as_ref();
        self.textures.push(storage);
        reference
    }

    pub(crate) fn add_material(&mut self, material: Material) -> Ref<Material> {
        let storage = Storage::new(material);
        let reference = storage.as_ref();
        self.materials.push(storage);
        reference
    }

    pub(crate) fn add_mesh(&mut self, mesh: CoreMesh) -> Index {
        let index = Index::new(self.next_index);
        self.next_index += 1;
        self.meshes.insert(index.clone(), mesh);
        index
    }

    pub(crate) fn add_shader(&mut self, shader: Shader) -> Ref<Shader> {
        let storage = Storage::new(shader);
        let reference = storage.as_ref();
        self.shaders.push(storage);
        reference
    }

    pub(crate) fn add_font(&mut self, font: Font) -> Ref<Font> {
        let storage = Storage::new(font);
        let reference = storage.as_ref();
        self.fonts.push(storage);
        reference
    }

    pub(crate) fn add_framebuffer(&mut self, framebuffer: Framebuffer) -> Ref<Framebuffer> {
        let storage = Storage::new(framebuffer);
        let reference = storage.as_ref();
        self.framebuffers.push(storage);
        reference
    }

    pub(crate) fn mesh(&self, index: &Index) -> &CoreMesh {
        self.meshes.get(index).expect("bad index")
    }

    pub(crate) fn mesh_mut(&mut self, index: &Index) -> &mut CoreMesh {
        self.meshes.get_mut(index).expect("bad index")
    }

    pub(crate) fn clean_unused(&mut self, uniform: &mut ImageUniform) {
        self.fonts.retain(|r| r.count() != 0);
        // self.meshes.retain(|r| r.count() != 0);
        self.materials.retain(|r| r.count() != 0);
        self.shaders.retain(|r| r.count() != 0);
        self.framebuffers.retain(|r| r.count() != 0);
        self.textures
            .drain_filter(|r| r.count() == 0)
            .for_each(|r| uniform.remove(r.with(|t| t.image_index())));
    }

    pub(crate) fn update_if_needed(&self) -> Result<()> {
        // for mesh in &self.meshes {
        // mesh.with(|m| m.update_if_needed())?;
        // }
        for material in &self.materials {
            material.with(|m| m.update_if_needed())?;
        }
        Ok(())
    }
}
