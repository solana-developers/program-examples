use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    pub const PREFIX: &[u8] = b"counter";
    pub const SIZE: usize = core::mem::size_of::<Self>();
}
