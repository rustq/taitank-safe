#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("taitank-safe/include/safe.h");

        type TaitankSafeNode;
        fn node_create() -> UniquePtr<TaitankSafeNode>;
        fn set_width(node: &mut UniquePtr<TaitankSafeNode>, width: f64);
        fn set_height(node: &mut UniquePtr<TaitankSafeNode>, height: f64);
        fn set_direction(node: &mut UniquePtr<TaitankSafeNode>, direction: i32);
        fn set_flex(node: &mut UniquePtr<TaitankSafeNode>, flex: f64);
        fn set_flex_grow(node: &mut UniquePtr<TaitankSafeNode>, flex_grow: f64);
        fn set_flex_shrink(node: &mut UniquePtr<TaitankSafeNode>, flex_shrink: f64);
        fn set_flex_basis(node: &mut UniquePtr<TaitankSafeNode>, flex_basis: f64);
        fn set_flex_direction(node: &mut UniquePtr<TaitankSafeNode>, flex_direction: i32);
        fn set_flex_wrap(node: &mut UniquePtr<TaitankSafeNode>, flex_wrap_node: i32);
        fn set_align_items(node: &mut UniquePtr<TaitankSafeNode>, flex_align: i32);
        fn set_min_width(node: &mut UniquePtr<TaitankSafeNode>, min_width: f64);
        fn set_align_content(node: &mut UniquePtr<TaitankSafeNode>, flex_align: i32);
        fn set_justify_content(node: &mut UniquePtr<TaitankSafeNode>, flex_align: i32);
        fn insert_child(
            node: &mut UniquePtr<TaitankSafeNode>,
            child: &mut UniquePtr<TaitankSafeNode>,
            index: i32,
        );
        fn do_layout(
            node: &mut UniquePtr<TaitankSafeNode>,
            parent_width: f64,
            parent_height: f64,
            direction: i32,
        );
        fn get_width(node: &mut UniquePtr<TaitankSafeNode>) -> f64;
        fn get_height(node: &mut UniquePtr<TaitankSafeNode>) -> f64;
        fn get_left(node: &mut UniquePtr<TaitankSafeNode>) -> f64;
        fn get_top(node: &mut UniquePtr<TaitankSafeNode>) -> f64;
    }
}
