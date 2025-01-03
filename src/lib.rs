pub mod macros;
mod safe;

use cxx::UniquePtr;
use safe::ffi;
use std::fmt::{Debug, Formatter, Result};

pub struct TaitankSafeNode {
    unique_ptr: UniquePtr<ffi::TaitankSafeNode>,
}

impl Debug for TaitankSafeNode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(ffi::print(&self.unique_ptr))
    }
}

unsafe impl Send for TaitankSafeNode {}
unsafe impl Sync for TaitankSafeNode {}


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

#[repr(i32)]
pub enum FlexAlign {
    FlexAlignAuto = 0,
    FlexAlignStart = 1,
    FlexAlignCenter = 2,
    FlexAlignEnd = 3,
    FlexAlignStretch = 4,
    FlexAlignBaseLine = 5,
    FlexAlignSpaceBetween = 6,
    FlexAlignSpaceAround = 7,
    FlexAlignSpaceEvenly = 8,
}

#[repr(i32)]
pub enum CSSDirection {
    CSSLeft = 0,
    CSSTop = 1,
    CSSRight = 2,
    CSSBottom = 3,
    CSSStart = 4,
    CSSEnd = 5,
    CSSHorizontal = 6,
    CSSVertical = 7,
    CSSAll = 8,
    CSSNone = -1,
}


/// Create and get a TaitankSafeNode.
///
/// # Examples
///
/// ```
/// let root = taitank_safe::node_create();
/// println!("{:?}", root);
/// ```
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

pub fn set_align_items(node: &mut TaitankSafeNode, flex_align: FlexAlign) {
    ffi::set_align_items(&mut node.unique_ptr, flex_align as i32);
}

pub fn set_min_width(node: &mut TaitankSafeNode, min_width: f64) {
    ffi::set_min_width(&mut node.unique_ptr, min_width);
}

pub fn set_max_height(node: &mut TaitankSafeNode, max_height: f64) {
    ffi::set_max_height(&mut node.unique_ptr, max_height);
}

pub fn set_margin(node: &mut TaitankSafeNode, css_direction: CSSDirection, value: f64) {
    ffi::set_margin(&mut node.unique_ptr, css_direction as i32, value);
}

pub fn set_align_content(node: &mut TaitankSafeNode, flex_align: FlexAlign) {
    ffi::set_align_content(&mut node.unique_ptr, flex_align as i32);
}

pub fn set_justify_content(node: &mut TaitankSafeNode, flex_align: FlexAlign) {
    ffi::set_justify_content(&mut node.unique_ptr, flex_align as i32);
}

pub fn insert_child(node: &mut TaitankSafeNode, child: &mut TaitankSafeNode, index: i32) {
    ffi::insert_child(&mut node.unique_ptr, &mut child.unique_ptr, index);
}

/// Layout the node.
///
/// # Examples
///
/// ```
/// let mut root = taitank_safe::node_create();
/// taitank_safe::set_width(&mut root, 200.0);
/// taitank_safe::do_layout(&mut root, std::f64::NAN, std::f64::NAN, taitank_safe::Direction::LTR);
/// ```
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
