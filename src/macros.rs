
/// Layout the node using macro.
///
/// # Examples
///
/// ```
/// let mut root = taitank_safe::node_create();
/// taitank_safe::set_width(&mut root);
/// taitank_safe::layout!(&mut root);
/// ```
#[macro_export]
macro_rules! layout {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::do_layout($a, $b, $c, $d)
    };

    ($a:expr, $b:expr, $c:expr) => {
        $crate::do_layout($a, $b, $c, $crate::Direction::LTR)
    };

    ($a:expr, $b:expr) => {
        $crate::do_layout($a, $b, std::f64::NAN, $crate::Direction::LTR)
    };

    ($a:expr) => {
        $crate::do_layout($a, std::f64::NAN, std::f64::NAN, $crate::Direction::LTR)
    };
}
