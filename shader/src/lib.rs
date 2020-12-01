#![no_std]
#![feature(lang_items)]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::{glam::Vec4, Input, Output};

#[allow(unused_attributes)]
#[spirv(vertex)]
pub fn main(
    #[spirv(vertex_index)] vert_idx: Input<i32>,
    #[spirv(position)] mut builtin_pos: Output<Vec4>,
)  {
    let _ = 5 * 5;
}
