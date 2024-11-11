mod safe;
use safe::ffi;
use cxx::UniquePtr;


struct TaitankSafeNode {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node = node_create();
        set_width(&node, 100.0);
        set_height(&node, 100.0);
        set_direction(&node, Direction::LTR);
        do_layout(&node, std::f64::NAN, std::f64::NAN, Direction::LTR);

        assert_eq!(get_left(&node), 0.0);
        assert_eq!(get_top(&node), 0.0);
        assert_eq!(get_width(&node), 100.0);
        assert_eq!(get_height(&node), 100.0);
    }

    #[test]
    fn flex_basis_flex_grow_column() {
            let root = node_create();
            set_width(&root, 100.0);
            set_height(&root, 100.0);
            let root_child0 = node_create();
            set_flex_grow(&root_child0, 1.0);
            set_flex_basis(&root_child0, 50.0);
            insert_child(&root, &root_child0, 0);

            let root_child1 = node_create();
            set_flex_grow(&root_child1, 1.0);
            insert_child(&root, &root_child1, 1);
            do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::LTR);

            assert_eq!(0.0, get_left(&root));
            assert_eq!(0.0, get_top(&root));
            assert_eq!(100.0, get_width(&root));
            assert_eq!(100.0, get_height(&root));

            assert_eq!(0.0, get_left(&root_child0));
            assert_eq!(0.0, get_top(&root_child0));
            assert_eq!(100.0, get_width(&root_child0));
            assert_eq!(75.0, get_height(&root_child0));

            assert_eq!(0.0, get_left(&root_child1));
            assert_eq!(75.0, get_top(&root_child1));
            assert_eq!(100.0, get_width(&root_child1));
            assert_eq!(25.0, get_height(&root_child1));

            do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::LTR);

            assert_eq!(0.0, get_left(&root));
            assert_eq!(0.0, get_top(&root));
            assert_eq!(100.0, get_width(&root));
            assert_eq!(100.0, get_height(&root));

            assert_eq!(0.0, get_left(&root_child0));
            assert_eq!(0.0, get_top(&root_child0));
            assert_eq!(100.0, get_width(&root_child0));
            assert_eq!(75.0, get_height(&root_child0));

            assert_eq!(0.0, get_left(&root_child1));
            assert_eq!(75.0, get_top(&root_child1));
            assert_eq!(100.0, get_width(&root_child1));
            assert_eq!(25.0, get_height(&root_child1));

    }

    #[test]
    fn flex_basis_flex_grow_row() {
        let root = node_create();
        set_flex_direction(&root, FlexDirection::FlexDirectionRow);
        set_width(&root, 100.0);
        set_height(&root, 100.0);
        let root_child0 = node_create();
        set_flex_grow(&root_child0, 1.0);
        set_flex_basis(&root_child0, 50.0);
        insert_child(&root, &root_child0, 0);

        let root_child1 = node_create();
        set_flex_grow(&root_child1, 1.0);
        insert_child(&root, &root_child1, 1);
        do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::LTR);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(75.0, get_width(&root_child0));
        assert_eq!(100.0, get_height(&root_child0));

        assert_eq!(75.0, get_left(&root_child1));
        assert_eq!(0.0, get_top(&root_child1));
        assert_eq!(25.0, get_width(&root_child1));
        assert_eq!(100.0, get_height(&root_child1));

        do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(25.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(75.0, get_width(&root_child0));
        assert_eq!(100.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(0.0, get_top(&root_child1));
        assert_eq!(25.0, get_width(&root_child1));
        assert_eq!(100.0, get_height(&root_child1));

    }
}