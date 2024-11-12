mod safe;
use safe::ffi;
use cxx::UniquePtr;


pub struct TaitankSafeNode {
    unique_ptr: UniquePtr<ffi::TaitankSafeNode>
}


#[repr(i32)]
pub enum Direction {
    Inherit = 0,
    LTR = 1,
    RTL = 2,
}

#[repr(i32)]
pub enum FlexDirection {
    FlexDirectionRow,
    FlexDirectionRowReverse,
    FlexDirectionColumn,
    FlexDirectionColumnReverse,
}


pub fn node_create() -> TaitankSafeNode {
    TaitankSafeNode {
        unique_ptr: ffi::node_create()
    }
}
pub fn set_width(node: &TaitankSafeNode, width: f64) {
    ffi::set_width(&node.unique_ptr, width);
}
pub fn set_height(node: &TaitankSafeNode, height: f64) {
    ffi::set_height(&node.unique_ptr, height);
}
pub fn set_direction(node: &TaitankSafeNode, direction: Direction) {
    ffi::set_direction(&node.unique_ptr, direction as i32);
}
pub fn set_flex(node: &TaitankSafeNode, flex: f64) {
    ffi::set_flex(&node.unique_ptr, flex);
}
pub fn set_flex_grow(node: &TaitankSafeNode, flex_grow: f64) {
    ffi::set_flex_grow(&node.unique_ptr, flex_grow);
}
pub fn set_flex_shrink(node: &TaitankSafeNode, flex_shrink: f64) {
    ffi::set_flex_shrink(&node.unique_ptr, flex_shrink);
}
pub fn set_flex_basis(node: &TaitankSafeNode, flex_basis: f64) {
    ffi::set_flex_basis(&node.unique_ptr, flex_basis);
}
pub fn set_flex_direction(node: &TaitankSafeNode, flex_direction: FlexDirection) {
    ffi::set_flex_direction(&node.unique_ptr, flex_direction as i32);
}
pub fn insert_child(node: &TaitankSafeNode, child: &TaitankSafeNode, index: i32) {
    ffi::insert_child(&node.unique_ptr, &child.unique_ptr, index);
}
pub fn do_layout(node: &TaitankSafeNode, parent_width: f64, parent_height: f64, direction: Direction) {
    ffi::do_layout(&node.unique_ptr, parent_width, parent_height, direction as i32);
}
pub fn get_width(node: &TaitankSafeNode) -> f64 {
    ffi::get_width(&node.unique_ptr)
}
pub fn get_height(node: &TaitankSafeNode) -> f64 {
    ffi::get_height(&node.unique_ptr)
}
pub fn get_left(node: &TaitankSafeNode) -> f64 {
    ffi::get_left(&node.unique_ptr)
}
pub fn get_top(node: &TaitankSafeNode) -> f64 {
    ffi::get_top(&node.unique_ptr)
}
