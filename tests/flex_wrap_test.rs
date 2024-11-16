#[cfg(test)]
mod tests {
    extern crate taitank_safe;
    use taitank_safe::*;

    #[test]
    fn wrap_column() {
        let mut root = node_create();
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrap);
        set_height(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 30.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 30.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 30.0);
        insert_child(&mut root, &mut root_child3, 3);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(60.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(30.0, get_width(&root_child0));
        assert_eq!(30.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(30.0, get_top(&root_child1));
        assert_eq!(30.0, get_width(&root_child1));
        assert_eq!(30.0, get_height(&root_child1));

        assert_eq!(0.0, get_left(&root_child2));
        assert_eq!(60.0, get_top(&root_child2));
        assert_eq!(30.0, get_width(&root_child2));
        assert_eq!(30.0, get_height(&root_child2));

        assert_eq!(30.0, get_left(&root_child3));
        assert_eq!(0.0, get_top(&root_child3));
        assert_eq!(30.0, get_width(&root_child3));
        assert_eq!(30.0, get_height(&root_child3));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(60.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(30.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(30.0, get_width(&root_child0));
        assert_eq!(30.0, get_height(&root_child0));

        assert_eq!(30.0, get_left(&root_child1));
        assert_eq!(30.0, get_top(&root_child1));
        assert_eq!(30.0, get_width(&root_child1));
        assert_eq!(30.0, get_height(&root_child1));

        assert_eq!(30.0, get_left(&root_child2));
        assert_eq!(60.0, get_top(&root_child2));
        assert_eq!(30.0, get_width(&root_child2));
        assert_eq!(30.0, get_height(&root_child2));

        assert_eq!(0.0, get_left(&root_child3));
        assert_eq!(0.0, get_top(&root_child3));
        assert_eq!(30.0, get_width(&root_child3));
        assert_eq!(30.0, get_height(&root_child3));

    }

    #[test]
    fn wrap_row() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrap);
        set_width(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 30.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 30.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 30.0);
        insert_child(&mut root, &mut root_child3, 3);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(60.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(30.0, get_width(&root_child0));
        assert_eq!(30.0, get_height(&root_child0));

        assert_eq!(30.0, get_left(&root_child1));
        assert_eq!(0.0, get_top(&root_child1));
        assert_eq!(30.0, get_width(&root_child1));
        assert_eq!(30.0, get_height(&root_child1));

        assert_eq!(60.0, get_left(&root_child2));
        assert_eq!(0.0, get_top(&root_child2));
        assert_eq!(30.0, get_width(&root_child2));
        assert_eq!(30.0, get_height(&root_child2));

        assert_eq!(0.0, get_left(&root_child3));
        assert_eq!(30.0, get_top(&root_child3));
        assert_eq!(30.0, get_width(&root_child3));
        assert_eq!(30.0, get_height(&root_child3));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(60.0, get_height(&root));

        assert_eq!(70.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(30.0, get_width(&root_child0));
        assert_eq!(30.0, get_height(&root_child0));

        assert_eq!(40.0, get_left(&root_child1));
        assert_eq!(0.0, get_top(&root_child1));
        assert_eq!(30.0, get_width(&root_child1));
        assert_eq!(30.0, get_height(&root_child1));

        assert_eq!(10.0, get_left(&root_child2));
        assert_eq!(0.0, get_top(&root_child2));
        assert_eq!(30.0, get_width(&root_child2));
        assert_eq!(30.0, get_height(&root_child2));

        assert_eq!(70.0, get_left(&root_child3));
        assert_eq!(30.0, get_top(&root_child3));
        assert_eq!(30.0, get_width(&root_child3));
        assert_eq!(30.0, get_height(&root_child3));

    }

    #[test]
    fn wrap_row_align_items_flex_end() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_align_items(&mut root, FlexAlign::FlexAlignEnd);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrap);
        set_width(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 10.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 20.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 30.0);
        insert_child(&mut root, &mut root_child3, 3);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(60.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(20.0, get_top(&root_child0));
        assert_eq!(30.0, get_width(&root_child0));
        assert_eq!(10.0, get_height(&root_child0));

        assert_eq!(30.0, get_left(&root_child1));
        assert_eq!(10.0, get_top(&root_child1));
        assert_eq!(30.0, get_width(&root_child1));
        assert_eq!(20.0, get_height(&root_child1));

        assert_eq!(60.0, get_left(&root_child2));
        assert_eq!(0.0, get_top(&root_child2));
        assert_eq!(30.0, get_width(&root_child2));
        assert_eq!(30.0, get_height(&root_child2));

        assert_eq!(0.0, get_left(&root_child3));
        assert_eq!(30.0, get_top(&root_child3));
        assert_eq!(30.0, get_width(&root_child3));
        assert_eq!(30.0, get_height(&root_child3));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(60.0, get_height(&root));

        assert_eq!(70.0, get_left(&root_child0));
        assert_eq!(20.0, get_top(&root_child0));
        assert_eq!(30.0, get_width(&root_child0));
        assert_eq!(10.0, get_height(&root_child0));

        assert_eq!(40.0, get_left(&root_child1));
        assert_eq!(10.0, get_top(&root_child1));
        assert_eq!(30.0, get_width(&root_child1));
        assert_eq!(20.0, get_height(&root_child1));

        assert_eq!(10.0, get_left(&root_child2));
        assert_eq!(0.0, get_top(&root_child2));
        assert_eq!(30.0, get_width(&root_child2));
        assert_eq!(30.0, get_height(&root_child2));

        assert_eq!(70.0, get_left(&root_child3));
        assert_eq!(30.0, get_top(&root_child3));
        assert_eq!(30.0, get_width(&root_child3));
        assert_eq!(30.0, get_height(&root_child3));

    }

    #[test]
    fn wrap_row_align_items_center() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_align_items(&mut root, FlexAlign::FlexAlignCenter);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrap);
        set_width(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 10.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 20.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 30.0);
        insert_child(&mut root, &mut root_child3, 3);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(60.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(10.0, get_top(&root_child0));
        assert_eq!(30.0, get_width(&root_child0));
        assert_eq!(10.0, get_height(&root_child0));

        assert_eq!(30.0, get_left(&root_child1));
        assert_eq!(5.0, get_top(&root_child1));
        assert_eq!(30.0, get_width(&root_child1));
        assert_eq!(20.0, get_height(&root_child1));

        assert_eq!(60.0, get_left(&root_child2));
        assert_eq!(0.0, get_top(&root_child2));
        assert_eq!(30.0, get_width(&root_child2));
        assert_eq!(30.0, get_height(&root_child2));

        assert_eq!(0.0, get_left(&root_child3));
        assert_eq!(30.0, get_top(&root_child3));
        assert_eq!(30.0, get_width(&root_child3));
        assert_eq!(30.0, get_height(&root_child3));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(60.0, get_height(&root));

        assert_eq!(70.0, get_left(&root_child0));
        assert_eq!(10.0, get_top(&root_child0));
        assert_eq!(30.0, get_width(&root_child0));
        assert_eq!(10.0, get_height(&root_child0));

        assert_eq!(40.0, get_left(&root_child1));
        assert_eq!(5.0, get_top(&root_child1));
        assert_eq!(30.0, get_width(&root_child1));
        assert_eq!(20.0, get_height(&root_child1));

        assert_eq!(10.0, get_left(&root_child2));
        assert_eq!(0.0, get_top(&root_child2));
        assert_eq!(30.0, get_width(&root_child2));
        assert_eq!(30.0, get_height(&root_child2));

        assert_eq!(70.0, get_left(&root_child3));
        assert_eq!(30.0, get_top(&root_child3));
        assert_eq!(30.0, get_width(&root_child3));
        assert_eq!(30.0, get_height(&root_child3));

    }

    #[test]
    fn flex_wrap_children_with_min_main_overriding_flex_basis() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrap);
        set_width(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_flex_basis(&mut root_child0, 50.0);
        set_min_width(&mut root_child0, 55.0);
        set_height(&mut root_child0, 50.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_basis(&mut root_child0, 50.0);
        set_min_width(&mut root_child1, 55.0);
        set_height(&mut root_child1, 50.0);
        insert_child(&mut root, &mut root_child1, 1);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(55.0, get_width(&root_child0));
        assert_eq!(50.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(50.0, get_top(&root_child1));
        assert_eq!(55.0, get_width(&root_child1));
        assert_eq!(50.0, get_height(&root_child1));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(100.0, get_height(&root));

        assert_eq!(45.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(55.0, get_width(&root_child0));
        assert_eq!(50.0, get_height(&root_child0));

        assert_eq!(45.0, get_left(&root_child1));
        assert_eq!(50.0, get_top(&root_child1));
        assert_eq!(55.0, get_width(&root_child1));
        assert_eq!(50.0, get_height(&root_child1));

    }

    #[test]
    fn flex_wrap_wrap_to_child_height() {
        let mut root = node_create();

        let mut root_child0 = node_create();
        set_flex_direction(&mut root_child0, FlexDirection::FlexDirectionRow);
        set_align_items(&mut root_child0, FlexAlign::FlexAlignStart);
        set_flex_wrap(&mut root_child0, FlexWrapNode::FlexWrap);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child0_child0 = node_create();
        set_width(&mut root_child0_child0, 100.0);
        insert_child(&mut root_child0, &mut root_child0_child0, 0);

        let mut root_child0_child0_child0 = node_create();
        set_width(&mut root_child0_child0_child0, 100.0);
        set_height(&mut root_child0_child0_child0, 100.0);
        insert_child(&mut root_child0_child0, &mut root_child0_child0_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 100.0);
        set_height(&mut root_child1, 100.0);
        insert_child(&mut root, &mut root_child1, 1);

        layout!(&mut root);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(200.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(100.0, get_width(&root_child0));
        assert_eq!(100.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child0_child0));
        assert_eq!(0.0, get_top(&root_child0_child0));
        assert_eq!(100.0, get_width(&root_child0_child0));
        assert_eq!(100.0, get_height(&root_child0_child0));

        assert_eq!(0.0, get_left(&root_child0_child0_child0));
        assert_eq!(0.0, get_top(&root_child0_child0_child0));
        assert_eq!(100.0, get_width(&root_child0_child0_child0));
        assert_eq!(100.0, get_height(&root_child0_child0_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(100.0, get_top(&root_child1));
        assert_eq!(100.0, get_width(&root_child1));
        assert_eq!(100.0, get_height(&root_child1));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&root));
        assert_eq!(0.0, get_top(&root));
        assert_eq!(100.0, get_width(&root));
        assert_eq!(200.0, get_height(&root));

        assert_eq!(0.0, get_left(&root_child0));
        assert_eq!(0.0, get_top(&root_child0));
        assert_eq!(100.0, get_width(&root_child0));
        assert_eq!(100.0, get_height(&root_child0));

        assert_eq!(0.0, get_left(&root_child0_child0));
        assert_eq!(0.0, get_top(&root_child0_child0));
        assert_eq!(100.0, get_width(&root_child0_child0));
        assert_eq!(100.0, get_height(&root_child0_child0));

        assert_eq!(0.0, get_left(&root_child0_child0_child0));
        assert_eq!(0.0, get_top(&root_child0_child0_child0));
        assert_eq!(100.0, get_width(&root_child0_child0_child0));
        assert_eq!(100.0, get_height(&root_child0_child0_child0));

        assert_eq!(0.0, get_left(&root_child1));
        assert_eq!(100.0, get_top(&root_child1));
        assert_eq!(100.0, get_width(&root_child1));
        assert_eq!(100.0, get_height(&root_child1));

    }

    #[test]
    fn flex_wrap_align_stretch_fits_one_row() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrap);
        set_width(&mut root, 150.0);
        set_height(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 50.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 50.0);
        insert_child(&mut root, &mut root_child1, 1);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 150.0);
        assert_eq!(get_height(&root), 100.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 50.0);
        assert_eq!(get_height(&root_child0), 100.0);

        assert_eq!(get_left(&root_child1), 50.0);
        assert_eq!(get_top(&root_child1), 0.0);
        assert_eq!(get_width(&root_child1), 50.0);
        assert_eq!(get_height(&root_child1), 100.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 150.0);
        assert_eq!(get_height(&root), 100.0);

        assert_eq!(get_left(&root_child0), 100.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 50.0);
        assert_eq!(get_height(&root_child0), 100.0);

        assert_eq!(get_left(&root_child1), 50.0);
        assert_eq!(get_top(&root_child1), 0.0);
        assert_eq!(get_width(&root_child1), 50.0);
        assert_eq!(get_height(&root_child1), 100.0);
    }


    #[test]
    fn wrap_reverse_row_align_content_flex_start() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrapReverse);
        set_width(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 10.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 20.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 40.0);
        insert_child(&mut root, &mut root_child3, 3);

        let mut root_child4 = node_create();
        set_width(&mut root_child4, 30.0);
        set_height(&mut root_child4, 50.0);
        insert_child(&mut root, &mut root_child4, 4);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 100.0);
        assert_eq!(get_height(&root), 80.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 70.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 30.0);
        assert_eq!(get_top(&root_child1), 60.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 60.0);
        assert_eq!(get_top(&root_child2), 50.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 0.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 30.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 100.0);
        assert_eq!(get_height(&root), 80.0);

        assert_eq!(get_left(&root_child0), 70.0);
        assert_eq!(get_top(&root_child0), 70.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 40.0);
        assert_eq!(get_top(&root_child1), 60.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 10.0);
        assert_eq!(get_top(&root_child2), 50.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 70.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 40.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);
    }

    #[test]
    fn wrap_reverse_row_align_content_center() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_align_content(&mut root, FlexAlign::FlexAlignCenter);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrapReverse);
        set_width(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 10.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 20.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 40.0);
        insert_child(&mut root, &mut root_child3, 3);

        let mut root_child4 = node_create();
        set_width(&mut root_child4, 30.0);
        set_height(&mut root_child4, 50.0);
        insert_child(&mut root, &mut root_child4, 4);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 100.0);
        assert_eq!(get_height(&root), 80.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 70.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 30.0);
        assert_eq!(get_top(&root_child1), 60.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 60.0);
        assert_eq!(get_top(&root_child2), 50.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 0.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 30.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 100.0);
        assert_eq!(get_height(&root), 80.0);

        assert_eq!(get_left(&root_child0), 70.0);
        assert_eq!(get_top(&root_child0), 70.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 40.0);
        assert_eq!(get_top(&root_child1), 60.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 10.0);
        assert_eq!(get_top(&root_child2), 50.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 70.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 40.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);
    }

    #[test]
    fn wrap_reverse_row_single_line_different_size() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrapReverse);
        set_width(&mut root, 300.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 10.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 20.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 40.0);
        insert_child(&mut root, &mut root_child3, 3);

        let mut root_child4 = node_create();
        set_width(&mut root_child4, 30.0);
        set_height(&mut root_child4, 50.0);
        insert_child(&mut root, &mut root_child4, 4);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 300.0);
        assert_eq!(get_height(&root), 50.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 40.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 30.0);
        assert_eq!(get_top(&root_child1), 30.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 60.0);
        assert_eq!(get_top(&root_child2), 20.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 90.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 120.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 300.0);
        assert_eq!(get_height(&root), 50.0);

        assert_eq!(get_left(&root_child0), 270.0);
        assert_eq!(get_top(&root_child0), 40.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 240.0);
        assert_eq!(get_top(&root_child1), 30.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 210.0);
        assert_eq!(get_top(&root_child2), 20.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 180.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 150.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);
    }

    #[test]
    fn wrap_reverse_row_align_content_stretch() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_align_content(&mut root, FlexAlign::FlexAlignStretch);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrapReverse);
        set_width(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 10.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 20.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 40.0);
        insert_child(&mut root, &mut root_child3, 3);

        let mut root_child4 = node_create();
        set_width(&mut root_child4, 30.0);
        set_height(&mut root_child4, 50.0);
        insert_child(&mut root, &mut root_child4, 4);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 100.0);
        assert_eq!(get_height(&root), 80.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 70.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 30.0);
        assert_eq!(get_top(&root_child1), 60.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 60.0);
        assert_eq!(get_top(&root_child2), 50.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 0.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 30.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 100.0);
        assert_eq!(get_height(&root), 80.0);

        assert_eq!(get_left(&root_child0), 70.0);
        assert_eq!(get_top(&root_child0), 70.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 40.0);
        assert_eq!(get_top(&root_child1), 60.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 10.0);
        assert_eq!(get_top(&root_child2), 50.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 70.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 40.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);
    }

    #[test]
    fn wrap_reverse_row_align_content_space_around() {
        let mut root = node_create();
        set_flex_direction(&mut root, FlexDirection::FlexDirectionRow);
        set_align_content(&mut root, FlexAlign::FlexAlignSpaceAround);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrapReverse);
        set_width(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 10.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 20.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 40.0);
        insert_child(&mut root, &mut root_child3, 3);

        let mut root_child4 = node_create();
        set_width(&mut root_child4, 30.0);
        set_height(&mut root_child4, 50.0);
        insert_child(&mut root, &mut root_child4, 4);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 100.0);
        assert_eq!(get_height(&root), 80.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 70.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 30.0);
        assert_eq!(get_top(&root_child1), 60.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 60.0);
        assert_eq!(get_top(&root_child2), 50.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 0.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 30.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 100.0);
        assert_eq!(get_height(&root), 80.0);

        assert_eq!(get_left(&root_child0), 70.0);
        assert_eq!(get_top(&root_child0), 70.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 40.0);
        assert_eq!(get_top(&root_child1), 60.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 10.0);
        assert_eq!(get_top(&root_child2), 50.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 70.0);
        assert_eq!(get_top(&root_child3), 10.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 40.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);
    }

    #[test]
    fn wrap_reverse_column_fixed_size() {
        let mut root = node_create();
        set_align_items(&mut root, FlexAlign::FlexAlignCenter);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrapReverse);
        set_width(&mut root, 200.0);
        set_height(&mut root, 100.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 30.0);
        set_height(&mut root_child0, 10.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_width(&mut root_child1, 30.0);
        set_height(&mut root_child1, 20.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 30.0);
        set_height(&mut root_child2, 30.0);
        insert_child(&mut root, &mut root_child2, 2);

        let mut root_child3 = node_create();
        set_width(&mut root_child3, 30.0);
        set_height(&mut root_child3, 40.0);
        insert_child(&mut root, &mut root_child3, 3);

        let mut root_child4 = node_create();
        set_width(&mut root_child4, 30.0);
        set_height(&mut root_child4, 50.0);
        insert_child(&mut root, &mut root_child4, 4);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 200.0);
        assert_eq!(get_height(&root), 100.0);

        assert_eq!(get_left(&root_child0), 170.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 170.0);
        assert_eq!(get_top(&root_child1), 10.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 170.0);
        assert_eq!(get_top(&root_child2), 30.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 170.0);
        assert_eq!(get_top(&root_child3), 60.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 140.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 200.0);
        assert_eq!(get_height(&root), 100.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 30.0);
        assert_eq!(get_height(&root_child0), 10.0);

        assert_eq!(get_left(&root_child1), 0.0);
        assert_eq!(get_top(&root_child1), 10.0);
        assert_eq!(get_width(&root_child1), 30.0);
        assert_eq!(get_height(&root_child1), 20.0);

        assert_eq!(get_left(&root_child2), 0.0);
        assert_eq!(get_top(&root_child2), 30.0);
        assert_eq!(get_width(&root_child2), 30.0);
        assert_eq!(get_height(&root_child2), 30.0);

        assert_eq!(get_left(&root_child3), 0.0);
        assert_eq!(get_top(&root_child3), 60.0);
        assert_eq!(get_width(&root_child3), 30.0);
        assert_eq!(get_height(&root_child3), 40.0);

        assert_eq!(get_left(&root_child4), 30.0);
        assert_eq!(get_top(&root_child4), 0.0);
        assert_eq!(get_width(&root_child4), 30.0);
        assert_eq!(get_height(&root_child4), 50.0);
    }
    #[test]
    fn wrapped_row_within_align_items_center() {
        let mut root = node_create();
        set_align_items(&mut root, FlexAlign::FlexAlignCenter);
        set_width(&mut root, 200.0);
        set_height(&mut root, 200.0);

        let mut root_child0 = node_create();
        set_flex_direction(&mut root_child0, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root_child0, FlexWrapNode::FlexWrap);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child0_child0 = node_create();
        set_width(&mut root_child0_child0, 150.0);
        set_height(&mut root_child0_child0, 80.0);
        insert_child(&mut root_child0, &mut root_child0_child0, 0);

        let mut root_child0_child1 = node_create();
        set_width(&mut root_child0_child1, 80.0);
        set_height(&mut root_child0_child1, 80.0);
        insert_child(&mut root_child0, &mut root_child0_child1, 1);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 200.0);
        assert_eq!(get_height(&root), 200.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 200.0);
        assert_eq!(get_height(&root_child0), 160.0);

        assert_eq!(get_left(&root_child0_child0), 0.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 150.0);
        assert_eq!(get_height(&root_child0_child0), 80.0);

        assert_eq!(get_left(&root_child0_child1), 0.0);
        assert_eq!(get_top(&root_child0_child1), 80.0);
        assert_eq!(get_width(&root_child0_child1), 80.0);
        assert_eq!(get_height(&root_child0_child1), 80.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 200.0);
        assert_eq!(get_height(&root), 200.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 200.0);
        assert_eq!(get_height(&root_child0), 160.0);

        assert_eq!(get_left(&root_child0_child0), 50.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 150.0);
        assert_eq!(get_height(&root_child0_child0), 80.0);

        assert_eq!(get_left(&root_child0_child1), 120.0);
        assert_eq!(get_top(&root_child0_child1), 80.0);
        assert_eq!(get_width(&root_child0_child1), 80.0);
        assert_eq!(get_height(&root_child0_child1), 80.0);
    }

    #[test]
    fn wrapped_row_within_align_items_flex_start() {
        let mut root = node_create();
        set_align_items(&mut root, FlexAlign::FlexAlignStart);
        set_width(&mut root, 200.0);
        set_height(&mut root, 200.0);

        let mut root_child0 = node_create();
        set_flex_direction(&mut root_child0, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root_child0, FlexWrapNode::FlexWrap);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child0_child0 = node_create();
        set_width(&mut root_child0_child0, 150.0);
        set_height(&mut root_child0_child0, 80.0);
        insert_child(&mut root_child0, &mut root_child0_child0, 0);

        let mut root_child0_child1 = node_create();
        set_width(&mut root_child0_child1, 80.0);
        set_height(&mut root_child0_child1, 80.0);
        insert_child(&mut root_child0, &mut root_child0_child1, 1);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 200.0);
        assert_eq!(get_height(&root), 200.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 200.0);
        assert_eq!(get_height(&root_child0), 160.0);

        assert_eq!(get_left(&root_child0_child0), 0.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 150.0);
        assert_eq!(get_height(&root_child0_child0), 80.0);

        assert_eq!(get_left(&root_child0_child1), 0.0);
        assert_eq!(get_top(&root_child0_child1), 80.0);
        assert_eq!(get_width(&root_child0_child1), 80.0);
        assert_eq!(get_height(&root_child0_child1), 80.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 200.0);
        assert_eq!(get_height(&root), 200.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 200.0);
        assert_eq!(get_height(&root_child0), 160.0);

        assert_eq!(get_left(&root_child0_child0), 50.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 150.0);
        assert_eq!(get_height(&root_child0_child0), 80.0);

        assert_eq!(get_left(&root_child0_child1), 120.0);
        assert_eq!(get_top(&root_child0_child1), 80.0);
        assert_eq!(get_width(&root_child0_child1), 80.0);
        assert_eq!(get_height(&root_child0_child1), 80.0);
    }

    #[test]
    fn wrapped_row_within_align_items_flex_end() {
        let mut root = node_create();
        set_align_items(&mut root, FlexAlign::FlexAlignEnd);
        set_width(&mut root, 200.0);
        set_height(&mut root, 200.0);

        let mut root_child0 = node_create();
        set_flex_direction(&mut root_child0, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root_child0, FlexWrapNode::FlexWrap);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child0_child0 = node_create();
        set_width(&mut root_child0_child0, 150.0);
        set_height(&mut root_child0_child0, 80.0);
        insert_child(&mut root_child0, &mut root_child0_child0, 0);

        let mut root_child0_child1 = node_create();
        set_width(&mut root_child0_child1, 80.0);
        set_height(&mut root_child0_child1, 80.0);
        insert_child(&mut root_child0, &mut root_child0_child1, 1);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 200.0);
        assert_eq!(get_height(&root), 200.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 200.0);
        assert_eq!(get_height(&root_child0), 160.0);

        assert_eq!(get_left(&root_child0_child0), 0.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 150.0);
        assert_eq!(get_height(&root_child0_child0), 80.0);

        assert_eq!(get_left(&root_child0_child1), 0.0);
        assert_eq!(get_top(&root_child0_child1), 80.0);
        assert_eq!(get_width(&root_child0_child1), 80.0);
        assert_eq!(get_height(&root_child0_child1), 80.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 200.0);
        assert_eq!(get_height(&root), 200.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 200.0);
        assert_eq!(get_height(&root_child0), 160.0);

        assert_eq!(get_left(&root_child0_child0), 50.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 150.0);
        assert_eq!(get_height(&root_child0_child0), 80.0);

        assert_eq!(get_left(&root_child0_child1), 120.0);
        assert_eq!(get_top(&root_child0_child1), 80.0);
        assert_eq!(get_width(&root_child0_child1), 80.0);
        assert_eq!(get_height(&root_child0_child1), 80.0);
    }

    #[test]
    fn wrapped_column_max_height() {
        let mut root = node_create();
        set_justify_content(&mut root, FlexAlign::FlexAlignCenter);
        set_align_content(&mut root, FlexAlign::FlexAlignCenter);
        set_align_items(&mut root, FlexAlign::FlexAlignCenter);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrap);
        set_width(&mut root, 700.0);
        set_height(&mut root, 500.0);

        let mut root_child0 = node_create();
        set_width(&mut root_child0, 100.0);
        set_height(&mut root_child0, 500.0);
        set_max_height(&mut root_child0, 200.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_margin(&mut root_child1, CSSDirection::CSSLeft, 20.0);
        set_margin(&mut root_child1, CSSDirection::CSSTop, 20.0);
        set_margin(&mut root_child1, CSSDirection::CSSRight, 20.0);
        set_margin(&mut root_child1, CSSDirection::CSSBottom, 20.0);
        set_width(&mut root_child1, 200.0);
        set_height(&mut root_child1, 200.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 100.0);
        set_height(&mut root_child2, 100.0);
        insert_child(&mut root, &mut root_child2, 2);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 700.0);
        assert_eq!(get_height(&root), 500.0);

        assert_eq!(get_left(&root_child0), 250.0);
        assert_eq!(get_top(&root_child0), 30.0);
        assert_eq!(get_width(&root_child0), 100.0);
        assert_eq!(get_height(&root_child0), 200.0);

        assert_eq!(get_left(&root_child1), 200.0);
        assert_eq!(get_top(&root_child1), 250.0);
        assert_eq!(get_width(&root_child1), 200.0);
        assert_eq!(get_height(&root_child1), 200.0);

        assert_eq!(get_left(&root_child2), 420.0);
        assert_eq!(get_top(&root_child2), 200.0);
        assert_eq!(get_width(&root_child2), 100.0);
        assert_eq!(get_height(&root_child2), 100.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 700.0);
        assert_eq!(get_height(&root), 500.0);

        assert_eq!(get_left(&root_child0), 350.0);
        assert_eq!(get_top(&root_child0), 30.0);
        assert_eq!(get_width(&root_child0), 100.0);
        assert_eq!(get_height(&root_child0), 200.0);

        assert_eq!(get_left(&root_child1), 300.0);
        assert_eq!(get_top(&root_child1), 250.0);
        assert_eq!(get_width(&root_child1), 200.0);
        assert_eq!(get_height(&root_child1), 200.0);

        assert_eq!(get_left(&root_child2), 180.0);
        assert_eq!(get_top(&root_child2), 200.0);
        assert_eq!(get_width(&root_child2), 100.0);
        assert_eq!(get_height(&root_child2), 100.0);
    }

    #[test]
    fn wrapped_column_max_height_flex() {
        let mut root = node_create();
        set_justify_content(&mut root, FlexAlign::FlexAlignCenter);
        set_align_content(&mut root, FlexAlign::FlexAlignCenter);
        set_align_items(&mut root, FlexAlign::FlexAlignCenter);
        set_flex_wrap(&mut root, FlexWrapNode::FlexWrap);
        set_width(&mut root, 700.0);
        set_height(&mut root, 500.0);

        let mut root_child0 = node_create();
        set_flex_grow(&mut root_child0, 1.0);
        set_flex_shrink(&mut root_child0, 1.0);
        set_flex_basis(&mut root_child0, 0.0);
        set_width(&mut root_child0, 100.0);
        set_height(&mut root_child0, 500.0);
        set_max_height(&mut root_child0, 200.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child1 = node_create();
        set_flex_grow(&mut root_child1, 1.0);
        set_flex_shrink(&mut root_child1, 1.0);
        set_flex_basis(&mut root_child1, 0.0);
        set_margin(&mut root_child1, CSSDirection::CSSLeft, 20.0);
        set_margin(&mut root_child1, CSSDirection::CSSTop, 20.0);
        set_margin(&mut root_child1, CSSDirection::CSSRight, 20.0);
        set_margin(&mut root_child1, CSSDirection::CSSBottom, 20.0);
        set_width(&mut root_child1, 200.0);
        set_height(&mut root_child1, 200.0);
        insert_child(&mut root, &mut root_child1, 1);

        let mut root_child2 = node_create();
        set_width(&mut root_child2, 100.0);
        set_height(&mut root_child2, 100.0);
        insert_child(&mut root, &mut root_child2, 2);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 700.0);
        assert_eq!(get_height(&root), 500.0);

        assert_eq!(get_left(&root_child0), 300.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 100.0);
        assert_eq!(get_height(&root_child0), 180.0);

        assert_eq!(get_left(&root_child1), 250.0);
        assert_eq!(get_top(&root_child1), 200.0);
        assert_eq!(get_width(&root_child1), 200.0);
        assert_eq!(get_height(&root_child1), 180.0);

        assert_eq!(get_left(&root_child2), 300.0);
        assert_eq!(get_top(&root_child2), 400.0);
        assert_eq!(get_width(&root_child2), 100.0);
        assert_eq!(get_height(&root_child2), 100.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 700.0);
        assert_eq!(get_height(&root), 500.0);

        assert_eq!(get_left(&root_child0), 300.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 100.0);
        assert_eq!(get_height(&root_child0), 180.0);

        assert_eq!(get_left(&root_child1), 250.0);
        assert_eq!(get_top(&root_child1), 200.0);
        assert_eq!(get_width(&root_child1), 200.0);
        assert_eq!(get_height(&root_child1), 180.0);

        assert_eq!(get_left(&root_child2), 300.0);
        assert_eq!(get_top(&root_child2), 400.0);
        assert_eq!(get_width(&root_child2), 100.0);
        assert_eq!(get_height(&root_child2), 100.0);
    }

    #[test]
    fn wrap_nodes_with_content_sizing_overflowing_margin() {
        let mut root = node_create();
        set_width(&mut root, 500.0);
        set_height(&mut root, 500.0);

        let mut root_child0 = node_create();
        set_flex_direction(&mut root_child0, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root_child0, FlexWrapNode::FlexWrap);
        set_width(&mut root_child0, 85.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child0_child0 = node_create();
        insert_child(&mut root_child0, &mut root_child0_child0, 0);

        let mut root_child0_child0_child0 = node_create();
        set_width(&mut root_child0_child0_child0, 40.0);
        set_height(&mut root_child0_child0_child0, 40.0);
        insert_child(&mut root_child0_child0, &mut root_child0_child0_child0, 0);

        let mut root_child0_child1 = node_create();
        set_margin(&mut root_child0_child1, CSSDirection::CSSRight, 10.0);
        insert_child(&mut root_child0, &mut root_child0_child1, 1);

        let mut root_child0_child1_child0 = node_create();
        set_width(&mut root_child0_child1_child0, 40.0);
        set_height(&mut root_child0_child1_child0, 40.0);
        insert_child(&mut root_child0_child1, &mut root_child0_child1_child0, 0);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 500.0);
        assert_eq!(get_height(&root), 500.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 85.0);
        assert_eq!(get_height(&root_child0), 80.0);

        assert_eq!(get_left(&root_child0_child0), 0.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 40.0);
        assert_eq!(get_height(&root_child0_child0), 40.0);

        assert_eq!(get_left(&root_child0_child0_child0), 0.0);
        assert_eq!(get_top(&root_child0_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0_child0), 40.0);
        assert_eq!(get_height(&root_child0_child0_child0), 40.0);

        assert_eq!(get_left(&root_child0_child1), 0.0);
        assert_eq!(get_top(&root_child0_child1), 40.0);
        assert_eq!(get_width(&root_child0_child1), 40.0);
        assert_eq!(get_height(&root_child0_child1), 40.0);

        assert_eq!(get_left(&root_child0_child1_child0), 0.0);
        assert_eq!(get_top(&root_child0_child1_child0), 0.0);
        assert_eq!(get_width(&root_child0_child1_child0), 40.0);
        assert_eq!(get_height(&root_child0_child1_child0), 40.0);
    }

    #[test]
    fn wrap_nodes_with_content_sizing_margin_cross() {
        let mut root = node_create();
        set_width(&mut root, 500.0);
        set_height(&mut root, 500.0);

        let mut root_child0 = node_create();
        set_flex_direction(&mut root_child0, FlexDirection::FlexDirectionRow);
        set_flex_wrap(&mut root_child0, FlexWrapNode::FlexWrap);
        set_width(&mut root_child0, 70.0);
        insert_child(&mut root, &mut root_child0, 0);

        let mut root_child0_child0 = node_create();
        insert_child(&mut root_child0, &mut root_child0_child0, 0);

        let mut root_child0_child0_child0 = node_create();
        set_width(&mut root_child0_child0_child0, 40.0);
        set_height(&mut root_child0_child0_child0, 40.0);
        insert_child(&mut root_child0_child0, &mut root_child0_child0_child0, 0);

        let mut root_child0_child1 = node_create();
        set_margin(&mut root_child0_child1, CSSDirection::CSSTop, 10.0);
        insert_child(&mut root_child0, &mut root_child0_child1, 1);

        let mut root_child0_child1_child0 = node_create();
        set_width(&mut root_child0_child1_child0, 40.0);
        set_height(&mut root_child0_child1_child0, 40.0);
        insert_child(&mut root_child0_child1, &mut root_child0_child1_child0, 0);

        layout!(&mut root);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 500.0);
        assert_eq!(get_height(&root), 500.0);

        assert_eq!(get_left(&root_child0), 0.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 70.0);
        assert_eq!(get_height(&root_child0), 90.0);

        assert_eq!(get_left(&root_child0_child0), 0.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 40.0);
        assert_eq!(get_height(&root_child0_child0), 40.0);

        assert_eq!(get_left(&root_child0_child0_child0), 0.0);
        assert_eq!(get_top(&root_child0_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0_child0), 40.0);
        assert_eq!(get_height(&root_child0_child0_child0), 40.0);

        assert_eq!(get_left(&root_child0_child1), 0.0);
        assert_eq!(get_top(&root_child0_child1), 50.0);
        assert_eq!(get_width(&root_child0_child1), 40.0);
        assert_eq!(get_height(&root_child0_child1), 40.0);

        assert_eq!(get_left(&root_child0_child1_child0), 0.0);
        assert_eq!(get_top(&root_child0_child1_child0), 0.0);
        assert_eq!(get_width(&root_child0_child1_child0), 40.0);
        assert_eq!(get_height(&root_child0_child1_child0), 40.0);

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(get_left(&root), 0.0);
        assert_eq!(get_top(&root), 0.0);
        assert_eq!(get_width(&root), 500.0);
        assert_eq!(get_height(&root), 500.0);

        assert_eq!(get_left(&root_child0), 430.0);
        assert_eq!(get_top(&root_child0), 0.0);
        assert_eq!(get_width(&root_child0), 70.0);
        assert_eq!(get_height(&root_child0), 90.0);

        assert_eq!(get_left(&root_child0_child0), 30.0);
        assert_eq!(get_top(&root_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0), 40.0);
        assert_eq!(get_height(&root_child0_child0), 40.0);

        assert_eq!(get_left(&root_child0_child0_child0), 0.0);
        assert_eq!(get_top(&root_child0_child0_child0), 0.0);
        assert_eq!(get_width(&root_child0_child0_child0), 40.0);
        assert_eq!(get_height(&root_child0_child0_child0), 40.0);

        assert_eq!(get_left(&root_child0_child1), 30.0);
        assert_eq!(get_top(&root_child0_child1), 50.0);
        assert_eq!(get_width(&root_child0_child1), 40.0);
        assert_eq!(get_height(&root_child0_child1), 40.0);

        assert_eq!(get_left(&root_child0_child1_child0), 0.0);
        assert_eq!(get_top(&root_child0_child1_child0), 0.0);
        assert_eq!(get_width(&root_child0_child1_child0), 40.0);
        assert_eq!(get_height(&root_child0_child1_child0), 40.0);
    }

}
