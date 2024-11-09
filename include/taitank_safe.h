#pragma once
#include <memory>
#include "taitank-safe/include/taitank/src/taitank.h"

class TaitankSafeNode {
public:
  TaitankSafeNode();
  ~TaitankSafeNode();
  void set_width(double width) const;
  void set_height(double height) const;
  void do_layout(double parent_width, double parent_height) const;
  double get_left() const;
  double get_top() const;
  double get_width() const;
  double get_height() const;
private:
  taitank::TaitankNodeRef ptr;
};

std::unique_ptr<TaitankSafeNode> node_create();
