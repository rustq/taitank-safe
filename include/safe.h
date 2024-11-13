#pragma once
#include <memory>
#include "./taitank/src/taitank.h"

class TaitankSafeNode {
public:
  TaitankSafeNode();
  ~TaitankSafeNode();
public:
  taitank::TaitankNodeRef ptr;
};

std::unique_ptr<TaitankSafeNode> node_create();
void set_width(std::unique_ptr<TaitankSafeNode> & node, double width);
void set_height(std::unique_ptr<TaitankSafeNode> & node, double width);
void set_direction(std::unique_ptr<TaitankSafeNode> & node, int direction);
void set_flex(std::unique_ptr<TaitankSafeNode> & node, double flex);
void set_flex_grow(std::unique_ptr<TaitankSafeNode> & node, double flex_grow);
void set_flex_shrink(std::unique_ptr<TaitankSafeNode> & node, double flex_shrink);
void set_flex_basis(std::unique_ptr<TaitankSafeNode> & node, double flex_basis);
void set_flex_direction(std::unique_ptr<TaitankSafeNode> & node, int direction);
void set_flex_wrap(std::unique_ptr<TaitankSafeNode> & node, int flex_wrap_node);
void set_align_items(std::unique_ptr<TaitankSafeNode> & node, int flex_align);
void set_min_width(std::unique_ptr<TaitankSafeNode> & node, double min_width);
void set_max_height(std::unique_ptr<TaitankSafeNode> & node, double max_height);
void set_margin(std::unique_ptr<TaitankSafeNode> & node, int css_direction, double value);
void set_align_content(std::unique_ptr<TaitankSafeNode> & node, int align_content);
void set_justify_content(std::unique_ptr<TaitankSafeNode> & node, int justify_content);
void insert_child(std::unique_ptr<TaitankSafeNode> & node, std::unique_ptr<TaitankSafeNode> & child, int index);
void do_layout(std::unique_ptr<TaitankSafeNode> & node, double parent_width, double parent_height, int direction);
double get_width(std::unique_ptr<TaitankSafeNode> & node);
double get_height(std::unique_ptr<TaitankSafeNode> & node);
double get_left(std::unique_ptr<TaitankSafeNode> & node);
double get_top(std::unique_ptr<TaitankSafeNode> & node);
