#pragma once
#include <memory>
#include "./taitank/src/taitank.h"

class TaitankSafeNode {
public:
  TaitankSafeNode();
  ~TaitankSafeNode();
//  void set_width(double width) const;
//  void set_height(double height) const;
//
//  void set_direction(int direction) const;
//
//  void do_layout(double parent_width, double parent_height) const;
//  double get_left() const;
//  double get_top() const;
//  double get_width() const;
//  double get_height() const;
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
void insert_child(std::unique_ptr<TaitankSafeNode> & node, std::unique_ptr<TaitankSafeNode> & child, int index);
void do_layout(std::unique_ptr<TaitankSafeNode> & node, double parent_width, double parent_height, int direction);
double get_width(std::unique_ptr<TaitankSafeNode> & node);
double get_height(std::unique_ptr<TaitankSafeNode> & node);
double get_left(std::unique_ptr<TaitankSafeNode> & node);
double get_top(std::unique_ptr<TaitankSafeNode> & node);
