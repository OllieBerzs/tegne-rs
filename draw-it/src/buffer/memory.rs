// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// BufferMemory - struct that manages allocated buffer memory

use ash::vk;
use std::ffi::c_void;
use std::ptr;
use std::rc::Rc;

use super::BufferAccess;
use super::BufferUsage;
use crate::device::Device;
use crate::error::Result;

pub(crate) struct BufferMemory {
    handle: vk::Buffer,
    memory: vk::DeviceMemory,
    device: Rc<Device>,
}

impl BufferMemory {
    pub(crate) fn new(
        device: &Rc<Device>,
        usage: &[BufferUsage],
        access: BufferAccess,
        size: usize,
    ) -> Result<Self> {
        // create buffer
        let info = vk::BufferCreateInfo::builder()
            .size((size as u32).into())
            .usage(BufferUsage::combine(usage))
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let (handle, memory) = device.allocate_buffer(&info, access)?;

        Ok(Self {
            handle,
            memory,
            device: Rc::clone(device),
        })
    }

    pub(crate) fn copy_from_data<T: Copy>(&self, data: &[T], size: usize) -> Result<()> {
        self.device.map_memory(self.memory, size, |mem| unsafe {
            ptr::copy_nonoverlapping(data as *const [T] as *const c_void, mem, size);
        })
    }

    pub(crate) fn copy_from_memory(&self, memory: &Self, size: usize) -> Result<()> {
        self.device.do_commands(|cmd| {
            self.device
                .cmd_copy_buffer(cmd, memory.handle(), self.handle, size);
            Ok(())
        })
    }

    pub(crate) fn handle(&self) -> vk::Buffer {
        self.handle
    }
}

impl Drop for BufferMemory {
    fn drop(&mut self) {
        self.device.free_buffer(self.handle, self.memory);
    }
}

impl PartialEq for BufferMemory {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
