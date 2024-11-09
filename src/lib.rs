use cxx::UniquePtr;

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("taitank-safe/include/taitank_safe.h");

        type TaitankSafeNode;
        fn node_create() -> UniquePtr<TaitankSafeNode>;
        fn set_width(self: &TaitankSafeNode, width: f64);
        fn set_height(self: &TaitankSafeNode, height: f64);
        fn set_direction(self: &TaitankSafeNode, direction: i32);
        fn do_layout(self: &TaitankSafeNode, parent_width: f64, parent_height: f64);
        fn get_left(self: &TaitankSafeNode) -> f64;
        fn get_top(self: &TaitankSafeNode) -> f64;
        fn get_width(self: &TaitankSafeNode) -> f64;
        fn get_height(self: &TaitankSafeNode) -> f64;
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
pub fn set_direction(node: &mut TaitankSafeNode, direction: Direction) {
    match direction {
         x => {
            node.unique_ptr.set_direction(x as i32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut node = node_create();
        node.unique_ptr.set_width(100.0);
        node.unique_ptr.set_height(100.0);
        set_direction(&mut node, Direction::LTR);
        node.unique_ptr.do_layout(std::f64::NAN, std::f64::NAN);

        assert_eq!(node.unique_ptr.get_left(), 0.0);
        assert_eq!(node.unique_ptr.get_top(), 0.0);
        assert_eq!(node.unique_ptr.get_width(), 100.0);
        assert_eq!(node.unique_ptr.get_height(), 100.0);
    }
}