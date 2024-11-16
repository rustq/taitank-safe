use taitank_safe::*;
fn main() {
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

    println!("{:?}", root);
}