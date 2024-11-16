#[cfg(test)]
mod tests {
    extern crate taitank_safe;

    #[test]
    fn it_works() {
        let mut node = taitank_safe::node_create();
        taitank_safe::set_width(&mut node, 100.0);
        taitank_safe::set_height(&mut node, 100.0);
        taitank_safe::set_direction(&mut node, taitank_safe::Direction::LTR);
        taitank_safe::layout!(&mut node);

        assert_eq!(taitank_safe::get_left(&node), 0.0);
        assert_eq!(taitank_safe::get_top(&node), 0.0);
        assert_eq!(taitank_safe::get_width(&node), 100.0);
        assert_eq!(taitank_safe::get_height(&node), 100.0);
    }
}
