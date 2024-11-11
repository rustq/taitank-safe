#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("taitank-safe/include/safe.h");

        type TaitankSafeNode;
        fn node_create() -> UniquePtr<TaitankSafeNode>;
        fn set_width(node: &UniquePtr<TaitankSafeNode>, width: f64);
        fn set_height(node: &UniquePtr<TaitankSafeNode>, height: f64);
        fn set_direction(node: &UniquePtr<TaitankSafeNode>, direction: i32);
        fn set_flex(node: &UniquePtr<TaitankSafeNode>, flex: f64);
        fn set_flex_grow(node: &UniquePtr<TaitankSafeNode>, flex_grow: f64);
        fn set_flex_shrink(node: &UniquePtr<TaitankSafeNode>, flex_shrink: f64);
        fn set_flex_basis(node: &UniquePtr<TaitankSafeNode>, flex_basis: f64);
        fn insert_child(node: &UniquePtr<TaitankSafeNode>, child: &UniquePtr<TaitankSafeNode>, index: i32);
        fn do_layout(node: &UniquePtr<TaitankSafeNode>, parent_width: f64, parent_height: f64);
        fn get_width(node: &UniquePtr<TaitankSafeNode>) -> f64;
        fn get_height(node: &UniquePtr<TaitankSafeNode>) -> f64;
        fn get_left(node: &UniquePtr<TaitankSafeNode>) -> f64;
        fn get_top(node: &UniquePtr<TaitankSafeNode>) -> f64;
    }
}

