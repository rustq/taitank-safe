#pragma once
#include <memory>
#include "taitank-safe/include/taitank/src/taitank.h"

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
void set_width(const std::unique_ptr<TaitankSafeNode> & node, double width);
void set_height(const std::unique_ptr<TaitankSafeNode> & node, double width);
void set_direction(const std::unique_ptr<TaitankSafeNode> & node, int direction);
void set_flex(const std::unique_ptr<TaitankSafeNode> & node, double flex);
void set_flex_grow(const std::unique_ptr<TaitankSafeNode> & node, double flex_grow);
void set_flex_shrink(const std::unique_ptr<TaitankSafeNode> & node, double flex_shrink);
void set_flex_basis(const std::unique_ptr<TaitankSafeNode> & node, double flex_basis);
void insert_child(const std::unique_ptr<TaitankSafeNode> & node, const std::unique_ptr<TaitankSafeNode> & child, int index);
void do_layout(const std::unique_ptr<TaitankSafeNode> & node, double parent_width, double parent_height);
double get_width(const std::unique_ptr<TaitankSafeNode> & node);
double get_height(const std::unique_ptr<TaitankSafeNode> & node);
double get_left(const std::unique_ptr<TaitankSafeNode> & node);
double get_top(const std::unique_ptr<TaitankSafeNode> & node);
