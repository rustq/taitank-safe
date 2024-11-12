#[cfg(test)]
mod tests {
    extern crate taitank_safe;
    use taitank_safe::*;

    #[test]
    fn it_works() {
        let mut node = node_create();
        set_width(&mut node, 100.0);
        set_height(&mut node, 100.0);
        set_direction(&mut node, Direction::LTR);
        layout!(&mut node);

        assert_eq!(get_left(&mut node), 0.0);
        assert_eq!(get_top(&mut node), 0.0);
        assert_eq!(get_width(&mut node), 100.0);
        assert_eq!(get_height(&mut node), 100.0);
    }

    #[test]
    fn flex_basis_flex_grow_column() {
        let mut root = node_create();
        set_width(&mut root, 100.0);
        set_height(&mut root, 100.0);
        let mut root_child0 = node_create();
        set_flex_grow(&mut root_child0, 1.0);
        set_flex_basis(&mut root_child0, 50.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_grow(&mut root_child1, 1.0);
        insert_child(&mut root, &mut root_child1, 1);
        do_layout(&mut root, std::f64::NAN, std::f64::NAN, Direction::LTR);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(100.0, get_width(&mut root_child0));
        assert_eq!(75.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(75.0, get_top(&mut root_child1));
        assert_eq!(100.0, get_width(&mut root_child1));
        assert_eq!(25.0, get_height(&mut root_child1));

        layout!(&mut root);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(100.0, get_width(&mut root_child0));
        assert_eq!(75.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(75.0, get_top(&mut root_child1));
        assert_eq!(100.0, get_width(&mut root_child1));
        assert_eq!(25.0, get_height(&mut root_child1));
    }

    #[test]
    fn flex_basis_flex_grow_row() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_width(&mut root, 100.0);
        set_height(&mut root, 100.0);
        let mut root_child0 = node_create();
        set_flex_grow(&mut root_child0, 1.0);
        set_flex_basis(&mut root_child0, 50.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_grow(&mut root_child1, 1.0);
        insert_child(&mut root, &mut root_child1, 1);
        layout!(&mut root);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(75.0, get_width(&mut root_child0));
        assert_eq!(100.0, get_height(&mut root_child0));

        assert_eq!(75.0, get_left(&mut root_child1));
        assert_eq!(0.0, get_top(&mut root_child1));
        assert_eq!(25.0, get_width(&mut root_child1));
        assert_eq!(100.0, get_height(&mut root_child1));

        do_layout(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(25.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(75.0, get_width(&mut root_child0));
        assert_eq!(100.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(0.0, get_top(&mut root_child1));
        assert_eq!(25.0, get_width(&mut root_child1));
        assert_eq!(100.0, get_height(&mut root_child1));
    }

    #[test]
    fn flex_basis_flex_shrink_column() {
        let mut root = node_create();
        set_width(&mut root, 100.0);
        set_height(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_flex_shrink(&mut root_child0, 1.0);
        set_flex_basis(&mut root_child0, 100.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_basis(&mut root_child1, 50.0);
        insert_child(&mut root, &mut root_child1, 1);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(100.0, get_width(&mut root_child0));
        assert_eq!(50.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(50.0, get_top(&mut root_child1));
        assert_eq!(100.0, get_width(&mut root_child1));
        assert_eq!(50.0, get_height(&mut root_child1));

        do_layout(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(100.0, get_width(&mut root_child0));
        assert_eq!(50.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(50.0, get_top(&mut root_child1));
        assert_eq!(100.0, get_width(&mut root_child1));
        assert_eq!(50.0, get_height(&mut root_child1));
    }

    #[test]
    fn flex_basis_flex_shrink_row() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_width(&mut root, 100.0);
        set_height(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_flex_shrink(&mut root_child0, 1.0);
        set_flex_basis(&mut root_child0, 100.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_basis(&mut root_child1, 50.0);
        insert_child(&mut root, &mut root_child1, 1);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(50.0, get_width(&mut root_child0));
        assert_eq!(100.0, get_height(&mut root_child0));

        assert_eq!(50.0, get_left(&mut root_child1));
        assert_eq!(0.0, get_top(&mut root_child1));
        assert_eq!(50.0, get_width(&mut root_child1));
        assert_eq!(100.0, get_height(&mut root_child1));

        do_layout(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(50.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(50.0, get_width(&mut root_child0));
        assert_eq!(100.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(0.0, get_top(&mut root_child1));
        assert_eq!(50.0, get_width(&mut root_child1));
        assert_eq!(100.0, get_height(&mut root_child1));
    }

    #[test]
    fn flex_shrink_to_zero() {
        let mut root = node_create();
        set_height(&mut root, 75.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 50.0);
        set_height(&mut root_child0, 50.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_shrink(&mut root_child1, 1.0);
        set_width(&mut root_child1, 50.0);
        set_height(&mut root_child1, 50.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 50.0);
        set_height(&mut root_child2, 50.0);
        insert_child(&mut root, &mut root_child2, 2);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(50.0, get_width(&mut root));
        assert_eq!(75.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(50.0, get_width(&mut root_child0));
        assert_eq!(50.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(50.0, get_top(&mut root_child1));
        assert_eq!(50.0, get_width(&mut root_child1));
        assert_eq!(0.0, get_height(&mut root_child1));

        assert_eq!(0.0, get_left(&mut root_child2));
        assert_eq!(50.0, get_top(&mut root_child2));
        assert_eq!(50.0, get_width(&mut root_child2));
        assert_eq!(50.0, get_height(&mut root_child2));

        do_layout(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(50.0, get_width(&mut root));
        assert_eq!(75.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(50.0, get_width(&mut root_child0));
        assert_eq!(50.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(50.0, get_top(&mut root_child1));
        assert_eq!(50.0, get_width(&mut root_child1));
        assert_eq!(0.0, get_height(&mut root_child1));

        assert_eq!(0.0, get_left(&mut root_child2));
        assert_eq!(50.0, get_top(&mut root_child2));
        assert_eq!(50.0, get_width(&mut root_child2));
        assert_eq!(50.0, get_height(&mut root_child2));
    }

    #[test]
    fn flex_basis_overrides_main_size() {
        let mut root = node_create();
        set_width(&mut root, 100.0);
        set_height(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_flex_grow(&mut root_child0, 1.0);
        set_flex_basis(&mut root_child0, 50.0);
        set_height(&mut root_child0, 20.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_grow(&mut root_child1, 1.0);
        set_height(&mut root_child1, 10.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_flex_grow(&mut root_child2, 1.0);
        set_height(&mut root_child2, 10.0);
        insert_child(&mut root, &mut root_child2, 2);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(100.0, get_width(&mut root_child0));
        assert_eq!(60.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(60.0, get_top(&mut root_child1));
        assert_eq!(100.0, get_width(&mut root_child1));
        assert_eq!(20.0, get_height(&mut root_child1));

        assert_eq!(0.0, get_left(&mut root_child2));
        assert_eq!(80.0, get_top(&mut root_child2));
        assert_eq!(100.0, get_width(&mut root_child2));
        assert_eq!(20.0, get_height(&mut root_child2));

        do_layout(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(100.0, get_width(&mut root_child0));
        assert_eq!(60.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(60.0, get_top(&mut root_child1));
        assert_eq!(100.0, get_width(&mut root_child1));
        assert_eq!(20.0, get_height(&mut root_child1));

        assert_eq!(0.0, get_left(&mut root_child2));
        assert_eq!(80.0, get_top(&mut root_child2));
        assert_eq!(100.0, get_width(&mut root_child2));
        assert_eq!(20.0, get_height(&mut root_child2));
    }

    #[test]
    fn flex_grow_shrink_at_most() {
        let mut root = node_create();
        set_width(&mut root, 100.0);
        set_height(&mut root, 100.0);

        let mut root_child0 = node_create();
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child0_child0 = node_create();
        set_flex_grow(&mut root_child0_child0, 1.0);
        set_flex_shrink(&mut root_child0_child0, 1.0);
        insert_child(&mut root_child0, &mut root_child0_child0, 0);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(100.0, get_width(&mut root_child0));
        assert_eq!(0.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child0_child0));
        assert_eq!(0.0, get_top(&mut root_child0_child0));
        assert_eq!(100.0, get_width(&mut root_child0_child0));
        assert_eq!(0.0, get_height(&mut root_child0_child0));

        do_layout(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(100.0, get_width(&mut root_child0));
        assert_eq!(0.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child0_child0));
        assert_eq!(0.0, get_top(&mut root_child0_child0));
        assert_eq!(100.0, get_width(&mut root_child0_child0));
        assert_eq!(0.0, get_height(&mut root_child0_child0));
    }

    #[test]
    fn flex_grow_less_than_factor_one() {
        let mut root = node_create();
        set_width(&mut root, 200.0);
        set_height(&mut root, 500.0);

        let mut root_child0 = node_create();
        set_flex_grow(&mut root_child0, 0.2);
        set_flex_basis(&mut root_child0, 40.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_grow(&mut root_child1, 0.2);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_flex_grow(&mut root_child2, 0.4);
        insert_child(&mut root, &mut root_child2, 2);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(200.0, get_width(&mut root));
        assert_eq!(500.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(200.0, get_width(&mut root_child0));
        assert_eq!(132.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(132.0, get_top(&mut root_child1));
        assert_eq!(200.0, get_width(&mut root_child1));
        assert_eq!(92.0, get_height(&mut root_child1));

        assert_eq!(0.0, get_left(&mut root_child2));
        assert_eq!(224.0, get_top(&mut root_child2));
        assert_eq!(200.0, get_width(&mut root_child2));
        assert_eq!(184.0, get_height(&mut root_child2));

        do_layout(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(200.0, get_width(&mut root));
        assert_eq!(500.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(200.0, get_width(&mut root_child0));
        assert_eq!(132.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(132.0, get_top(&mut root_child1));
        assert_eq!(200.0, get_width(&mut root_child1));
        assert_eq!(92.0, get_height(&mut root_child1));

        assert_eq!(0.0, get_left(&mut root_child2));
        assert_eq!(224.0, get_top(&mut root_child2));
        assert_eq!(200.0, get_width(&mut root_child2));
        assert_eq!(184.0, get_height(&mut root_child2));
    }
}
