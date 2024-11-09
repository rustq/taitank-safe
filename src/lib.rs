use cxx::UniquePtr;

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("taitank-safe/include/taitank_safe.h");

        type TaitankSafeNode;
        fn node_create() -> UniquePtr<TaitankSafeNode>;
        fn set_width(node: &UniquePtr<TaitankSafeNode>, width: f64);
        fn set_height(node: &UniquePtr<TaitankSafeNode>, height: f64);
        fn set_direction(node: &UniquePtr<TaitankSafeNode>, direction: i32);
        fn do_layout(node: &UniquePtr<TaitankSafeNode>, parent_width: f64, parent_height: f64);
        fn get_width(node: &UniquePtr<TaitankSafeNode>) -> f64;
        fn get_height(node: &UniquePtr<TaitankSafeNode>) -> f64;
        fn get_left(node: &UniquePtr<TaitankSafeNode>) -> f64;
        fn get_top(node: &UniquePtr<TaitankSafeNode>) -> f64;
    }
}

struct TaitankSafeNode {
    unique_ptr: UniquePtr<ffi::TaitankSafeNode>
}


#[repr(i32)]
pub enum Direction {
    Inherit = 0,
    LTR = 1,
    RTL = 2,
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
pub fn do_layout(node: &TaitankSafeNode, parent_width: f64, parent_height: f64) {
    ffi::do_layout(&node.unique_ptr, parent_width, parent_height);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node = node_create();
        set_width(&node, 100.0);
        set_height(&node, 100.0);
        set_direction(&node, Direction::LTR);
        do_layout(&node, std::f64::NAN, std::f64::NAN);

        assert_eq!(get_left(&node), 0.0);
        assert_eq!(get_top(&node), 0.0);
        assert_eq!(get_width(&node), 100.0);
        assert_eq!(get_height(&node), 100.0);
    }
}