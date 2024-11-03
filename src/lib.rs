#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("taitank-safe/include/taitank_safe.h");

        type TaitankSafeNode;
        fn node_create() -> UniquePtr<TaitankSafeNode>;
        fn set_width(self: &TaitankSafeNode, width: f64);
        fn set_height(self: &TaitankSafeNode, height: f64);
        fn do_layout(self: &TaitankSafeNode, parent_width: f64, parent_height: f64);
        fn get_top(self: &TaitankSafeNode) -> f64;
        fn get_width(self: &TaitankSafeNode) -> f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node = ffi::node_create();
        node.set_width(200.0);
        node.set_height(200.0);
        node.do_layout(std::f64::NAN, std::f64::NAN);

        assert_eq!(node.get_top(), 0.0);
        assert_eq!(node.get_width(), 200.0);
    }
}