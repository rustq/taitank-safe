#[macro_export]
macro_rules! layout {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        do_layout($a, $b, $c, $d)
    };

    ($a:expr, $b:expr, $c:expr) => {
        do_layout($a, $b, $c, Direction::LTR)
    };

    ($a:expr, $b:expr) => {
        do_layout($a, $b, std::f64::NAN, Direction::LTR)
    };

    ($a:expr) => {
        do_layout($a, std::f64::NAN, std::f64::NAN, Direction::LTR)
    };
}