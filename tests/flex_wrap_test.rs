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

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(60.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(30.0, get_width(&mut root_child0));
        assert_eq!(30.0, get_height(&mut root_child0));

        assert_eq!(0.0, get_left(&mut root_child1));
        assert_eq!(30.0, get_top(&mut root_child1));
        assert_eq!(30.0, get_width(&mut root_child1));
        assert_eq!(30.0, get_height(&mut root_child1));

        assert_eq!(0.0, get_left(&mut root_child2));
        assert_eq!(60.0, get_top(&mut root_child2));
        assert_eq!(30.0, get_width(&mut root_child2));
        assert_eq!(30.0, get_height(&mut root_child2));

        assert_eq!(30.0, get_left(&mut root_child3));
        assert_eq!(0.0, get_top(&mut root_child3));
        assert_eq!(30.0, get_width(&mut root_child3));
        assert_eq!(30.0, get_height(&mut root_child3));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(60.0, get_width(&mut root));
        assert_eq!(100.0, get_height(&mut root));

        assert_eq!(30.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(30.0, get_width(&mut root_child0));
        assert_eq!(30.0, get_height(&mut root_child0));

        assert_eq!(30.0, get_left(&mut root_child1));
        assert_eq!(30.0, get_top(&mut root_child1));
        assert_eq!(30.0, get_width(&mut root_child1));
        assert_eq!(30.0, get_height(&mut root_child1));

        assert_eq!(30.0, get_left(&mut root_child2));
        assert_eq!(60.0, get_top(&mut root_child2));
        assert_eq!(30.0, get_width(&mut root_child2));
        assert_eq!(30.0, get_height(&mut root_child2));

        assert_eq!(0.0, get_left(&mut root_child3));
        assert_eq!(0.0, get_top(&mut root_child3));
        assert_eq!(30.0, get_width(&mut root_child3));
        assert_eq!(30.0, get_height(&mut root_child3));

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

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(60.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(30.0, get_width(&mut root_child0));
        assert_eq!(30.0, get_height(&mut root_child0));

        assert_eq!(30.0, get_left(&mut root_child1));
        assert_eq!(0.0, get_top(&mut root_child1));
        assert_eq!(30.0, get_width(&mut root_child1));
        assert_eq!(30.0, get_height(&mut root_child1));

        assert_eq!(60.0, get_left(&mut root_child2));
        assert_eq!(0.0, get_top(&mut root_child2));
        assert_eq!(30.0, get_width(&mut root_child2));
        assert_eq!(30.0, get_height(&mut root_child2));

        assert_eq!(0.0, get_left(&mut root_child3));
        assert_eq!(30.0, get_top(&mut root_child3));
        assert_eq!(30.0, get_width(&mut root_child3));
        assert_eq!(30.0, get_height(&mut root_child3));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(60.0, get_height(&mut root));

        assert_eq!(70.0, get_left(&mut root_child0));
        assert_eq!(0.0, get_top(&mut root_child0));
        assert_eq!(30.0, get_width(&mut root_child0));
        assert_eq!(30.0, get_height(&mut root_child0));

        assert_eq!(40.0, get_left(&mut root_child1));
        assert_eq!(0.0, get_top(&mut root_child1));
        assert_eq!(30.0, get_width(&mut root_child1));
        assert_eq!(30.0, get_height(&mut root_child1));

        assert_eq!(10.0, get_left(&mut root_child2));
        assert_eq!(0.0, get_top(&mut root_child2));
        assert_eq!(30.0, get_width(&mut root_child2));
        assert_eq!(30.0, get_height(&mut root_child2));

        assert_eq!(70.0, get_left(&mut root_child3));
        assert_eq!(30.0, get_top(&mut root_child3));
        assert_eq!(30.0, get_width(&mut root_child3));
        assert_eq!(30.0, get_height(&mut root_child3));

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

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(60.0, get_height(&mut root));

        assert_eq!(0.0, get_left(&mut root_child0));
        assert_eq!(20.0, get_top(&mut root_child0));
        assert_eq!(30.0, get_width(&mut root_child0));
        assert_eq!(10.0, get_height(&mut root_child0));

        assert_eq!(30.0, get_left(&mut root_child1));
        assert_eq!(10.0, get_top(&mut root_child1));
        assert_eq!(30.0, get_width(&mut root_child1));
        assert_eq!(20.0, get_height(&mut root_child1));

        assert_eq!(60.0, get_left(&mut root_child2));
        assert_eq!(0.0, get_top(&mut root_child2));
        assert_eq!(30.0, get_width(&mut root_child2));
        assert_eq!(30.0, get_height(&mut root_child2));

        assert_eq!(0.0, get_left(&mut root_child3));
        assert_eq!(30.0, get_top(&mut root_child3));
        assert_eq!(30.0, get_width(&mut root_child3));
        assert_eq!(30.0, get_height(&mut root_child3));

        layout!(&mut root, std::f64::NAN, std::f64::NAN, Direction::RTL);

        assert_eq!(0.0, get_left(&mut root));
        assert_eq!(0.0, get_top(&mut root));
        assert_eq!(100.0, get_width(&mut root));
        assert_eq!(60.0, get_height(&mut root));

        assert_eq!(70.0, get_left(&mut root_child0));
        assert_eq!(20.0, get_top(&mut root_child0));
        assert_eq!(30.0, get_width(&mut root_child0));
        assert_eq!(10.0, get_height(&mut root_child0));

        assert_eq!(40.0, get_left(&mut root_child1));
        assert_eq!(10.0, get_top(&mut root_child1));
        assert_eq!(30.0, get_width(&mut root_child1));
        assert_eq!(20.0, get_height(&mut root_child1));

        assert_eq!(10.0, get_left(&mut root_child2));
        assert_eq!(0.0, get_top(&mut root_child2));
        assert_eq!(30.0, get_width(&mut root_child2));
        assert_eq!(30.0, get_height(&mut root_child2));

        assert_eq!(70.0, get_left(&mut root_child3));
        assert_eq!(30.0, get_top(&mut root_child3));
        assert_eq!(30.0, get_width(&mut root_child3));
        assert_eq!(30.0, get_height(&mut root_child3));

    }
}
