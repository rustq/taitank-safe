#[cfg(test)]
mod tests {
    extern crate taitank_safe;
    use taitank_safe::*;

    #[test]
    fn it_works() {
        let node = node_create();
        set_width(&node, 100.0);
        set_height(&node, 100.0);
        set_direction(&node, Direction::LTR);
        layout!(&node);

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

            layout!(&root);

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
        layout!(&root);

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

    #[test]
    fn flex_basis_flex_shrink_column() {
        let root = node_create();
        set_width(&root, 100.0);
        set_height(&root, 100.0);

        let root_child0 = node_create();
        set_flex_shrink(&root_child0, 1.0);
        set_flex_basis(&root_child0, 100.0);
        insert_child(&root, &root_child0, 0);

        let root_child1 = node_create();
        set_flex_basis(&root_child1, 50.0);
        insert_child(&root, &root_child1, 1);

        layout!(&root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(100.0, get_width(&root_child0));
        assert_eq!(50.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(50.0, get_top(&root_child1));
        assert_eq!(100.0, get_width(&root_child1));
        assert_eq!(50.0, get_height(&root_child1));

        do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(100.0, get_width(&root_child0));
        assert_eq!(50.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(50.0, get_top(&root_child1));
        assert_eq!(100.0, get_width(&root_child1));
        assert_eq!(50.0, get_height(&root_child1));
    }

    #[test]
    fn flex_basis_flex_shrink_row() {
        let root = node_create();
        set_flex_direction(&root, FlexDirection::FlexDirectionRow);
        set_width(&root, 100.0);
        set_height(&root, 100.0);

        let root_child0 = node_create();
        set_flex_shrink(&root_child0, 1.0);
        set_flex_basis(&root_child0, 100.0);
        insert_child(&root, &root_child0, 0);

        let root_child1 = node_create();
        set_flex_basis(&root_child1, 50.0);
        insert_child(&root, &root_child1, 1);

        layout!(&root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(50.0, get_width(&root_child0));
        assert_eq!(100.0, get_height(&root_child0));

        assert_eq!(50.0, get_left(&root_child1));
        assert_eq!(0.0, get_top(&root_child1));
        assert_eq!(50.0, get_width(&root_child1));
        assert_eq!(100.0, get_height(&root_child1));

        do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(50.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(50.0, get_width(&root_child0));
        assert_eq!(100.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(0.0, get_top(&root_child1));
        assert_eq!(50.0, get_width(&root_child1));
        assert_eq!(100.0, get_height(&root_child1));
    }


    #[test]
    fn flex_shrink_to_zero() {
        let root = node_create();
        set_height(&root, 75.0);

        let root_child0 = node_create();
        set_width(&root_child0, 50.0);
        set_height(&root_child0, 50.0);
        insert_child(&root, &root_child0, 0);

        let root_child1 = node_create();
        set_flex_shrink(&root_child1, 1.0);
        set_width(&root_child1, 50.0);
        set_height(&root_child1, 50.0);
        insert_child(&root, &root_child1, 1);

        let root_child2 = node_create();
        set_width(&root_child2, 50.0);
        set_height(&root_child2, 50.0);
        insert_child(&root, &root_child2, 2);

        layout!(&root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(50.0, get_width(&root));
        assert_eq!(75.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(50.0, get_width(&root_child0));
        assert_eq!(50.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(50.0, get_top(&root_child1));
        assert_eq!(50.0, get_width(&root_child1));
        assert_eq!(0.0, get_height(&root_child1));

        assert_eq!(0.0, get_left(&root_child2));
        assert_eq!(50.0, get_top(&root_child2));
        assert_eq!(50.0, get_width(&root_child2));
        assert_eq!(50.0, get_height(&root_child2));

        do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(50.0, get_width(&root));
        assert_eq!(75.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(50.0, get_width(&root_child0));
        assert_eq!(50.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(50.0, get_top(&root_child1));
        assert_eq!(50.0, get_width(&root_child1));
        assert_eq!(0.0, get_height(&root_child1));

        assert_eq!(0.0, get_left(&root_child2));
        assert_eq!(50.0, get_top(&root_child2));
        assert_eq!(50.0, get_width(&root_child2));
        assert_eq!(50.0, get_height(&root_child2));
    }


    #[test]
    fn flex_basis_overrides_main_size() {
        let root = node_create();
        set_width(&root, 100.0);
        set_height(&root, 100.0);

        let root_child0 = node_create();
        set_flex_grow(&root_child0, 1.0);
        set_flex_basis(&root_child0, 50.0);
        set_height(&root_child0, 20.0);
        insert_child(&root, &root_child0, 0);

        let root_child1 = node_create();
        set_flex_grow(&root_child1, 1.0);
        set_height(&root_child1, 10.0);
        insert_child(&root, &root_child1, 1);

        let root_child2 = node_create();
        set_flex_grow(&root_child2, 1.0);
        set_height(&root_child2, 10.0);
        insert_child(&root, &root_child2, 2);

        layout!(&root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(100.0, get_width(&root_child0));
        assert_eq!(60.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(60.0, get_top(&root_child1));
        assert_eq!(100.0, get_width(&root_child1));
        assert_eq!(20.0, get_height(&root_child1));

        assert_eq!(0.0, get_left(&root_child2));
        assert_eq!(80.0, get_top(&root_child2));
        assert_eq!(100.0, get_width(&root_child2));
        assert_eq!(20.0, get_height(&root_child2));

        do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(100.0, get_width(&root_child0));
        assert_eq!(60.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(60.0, get_top(&root_child1));
        assert_eq!(100.0, get_width(&root_child1));
        assert_eq!(20.0, get_height(&root_child1));

        assert_eq!(0.0, get_left(&root_child2));
        assert_eq!(80.0, get_top(&root_child2));
        assert_eq!(100.0, get_width(&root_child2));
        assert_eq!(20.0, get_height(&root_child2));
    }

    #[test]
    fn flex_grow_shrink_at_most() {
        let root = node_create();
        set_width(&root, 100.0);
        set_height(&root, 100.0);

        let root_child0 = node_create();
        insert_child(&root, &root_child0, 0);

        let root_child0_child0 = node_create();
        set_flex_grow(&root_child0_child0, 1.0);
        set_flex_shrink(&root_child0_child0, 1.0);
        insert_child(&root_child0, &root_child0_child0, 0);

        layout!(&root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(100.0, get_width(&root_child0));
        assert_eq!(0.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child0_child0));
        assert_eq!(0.0, get_top(&root_child0_child0));
        assert_eq!(100.0, get_width(&root_child0_child0));
        assert_eq!(0.0, get_height(&root_child0_child0));

        do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(100.0, get_width(&root_child0));
        assert_eq!(0.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child0_child0));
        assert_eq!(0.0, get_top(&root_child0_child0));
        assert_eq!(100.0, get_width(&root_child0_child0));
        assert_eq!(0.0, get_height(&root_child0_child0));
    }

    #[test]
    fn flex_grow_less_than_factor_one() {
        let root = node_create();
        set_width(&root, 200.0);
        set_height(&root, 500.0);

        let root_child0 = node_create();
        set_flex_grow(&root_child0, 0.2);
        set_flex_basis(&root_child0, 40.0);
        insert_child(&root, &root_child0, 0);

        let root_child1 = node_create();
        set_flex_grow(&root_child1, 0.2);
        insert_child(&root, &root_child1, 1);

        let root_child2 = node_create();
        set_flex_grow(&root_child2, 0.4);
        insert_child(&root, &root_child2, 2);

        layout!(&root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(200.0, get_width(&root));
        assert_eq!(500.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(200.0, get_width(&root_child0));
        assert_eq!(132.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(132.0, get_top(&root_child1));
        assert_eq!(200.0, get_width(&root_child1));
        assert_eq!(92.0, get_height(&root_child1));

        assert_eq!(0.0, get_left(&root_child2));
        assert_eq!(224.0, get_top(&root_child2));
        assert_eq!(200.0, get_width(&root_child2));
        assert_eq!(184.0, get_height(&root_child2));

        do_layout(&root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(200.0, get_width(&root));
        assert_eq!(500.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(200.0, get_width(&root_child0));
        assert_eq!(132.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(132.0, get_top(&root_child1));
        assert_eq!(200.0, get_width(&root_child1));
        assert_eq!(92.0, get_height(&root_child1));

        assert_eq!(0.0, get_left(&root_child2));
        assert_eq!(224.0, get_top(&root_child2));
        assert_eq!(200.0, get_width(&root_child2));
        assert_eq!(184.0, get_height(&root_child2));
    }
}