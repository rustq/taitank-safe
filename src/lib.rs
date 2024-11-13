pub mod macros;
mod safe;

use cxx::UniquePtr;
use safe::ffi;

pub struct TaitankSafeNode {
    unique_ptr: UniquePtr<ffi::TaitankSafeNode>,
}

#[repr(i32)]
pub enum Direction {
    Inherit = 0,
    LTR = 1,
    RTL = 2,
}

#[repr(i32)]
pub enum FlexDirection {
    FlexDirectionRow = 0,
    FlexDirectionRowReverse = 1,
    FlexDirectionColumn = 2,
    FlexDirectionColumnReverse = 3,
}

#[repr(i32)]
pub enum FlexWrapNode {
    FlexNoWrap = 0,
    FlexWrap = 1,
    FlexWrapReverse = 2,
}

pub fn node_create() -> TaitankSafeNode {
    TaitankSafeNode {
        unique_ptr: ffi::node_create(),
    }
}
pub fn set_width(node: &mut TaitankSafeNode, width: f64) {
    ffi::set_width(&mut node.unique_ptr, width);
}
pub fn set_height(node: &mut TaitankSafeNode, height: f64) {
    ffi::set_height(&mut node.unique_ptr, height);
}
pub fn set_direction(node: &mut TaitankSafeNode, direction: Direction) {
    ffi::set_direction(&mut node.unique_ptr, direction as i32);
}
pub fn set_flex(node: &mut TaitankSafeNode, flex: f64) {
    ffi::set_flex(&mut node.unique_ptr, flex);
}
pub fn set_flex_grow(node: &mut TaitankSafeNode, flex_grow: f64) {
    ffi::set_flex_grow(&mut node.unique_ptr, flex_grow);
}
pub fn set_flex_shrink(node: &mut TaitankSafeNode, flex_shrink: f64) {
    ffi::set_flex_shrink(&mut node.unique_ptr, flex_shrink);
}
pub fn set_flex_basis(node: &mut TaitankSafeNode, flex_basis: f64) {
    ffi::set_flex_basis(&mut node.unique_ptr, flex_basis);
}
pub fn set_flex_direction(node: &mut TaitankSafeNode, flex_direction: FlexDirection) {
    ffi::set_flex_direction(&mut node.unique_ptr, flex_direction as i32);
}

pub fn set_flex_wrap(node: &mut TaitankSafeNode, flex_wrap_node: FlexWrapNode) {
    ffi::set_flex_wrap(&mut node.unique_ptr, flex_wrap_node as i32);
}

pub fn insert_child(node: &mut TaitankSafeNode, child: &mut TaitankSafeNode, index: i32) {
    ffi::insert_child(&mut node.unique_ptr, &mut child.unique_ptr, index);
}
pub fn do_layout(
    node: &mut TaitankSafeNode,
    parent_width: f64,
    parent_height: f64,
    direction: Direction,
) {
    ffi::do_layout(
        &mut node.unique_ptr,
        parent_width,
        parent_height,
        direction as i32,
    );
}
pub fn get_width(node: &mut TaitankSafeNode) -> f64 {
    ffi::get_width(&mut node.unique_ptr)
}
pub fn get_height(node: &mut TaitankSafeNode) -> f64 {
    ffi::get_height(&mut node.unique_ptr)
}
pub fn get_left(node: &mut TaitankSafeNode) -> f64 {
    ffi::get_left(&mut node.unique_ptr)
}
pub fn get_top(node: &mut TaitankSafeNode) -> f64 {
    ffi::get_top(&mut node.unique_ptr)
}
