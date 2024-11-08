#include "taitank-safe/include/taitank_safe.h"
#include <functional>
#include <string>

#include "taitank-safe/include/taitank/src/taitank_util.h"
#include "taitank-safe/include/taitank/src/taitank_node.h"
#include "taitank-safe/include/taitank/src/taitank.h"


TaitankSafeNode::TaitankSafeNode() {
  ptr = new taitank::TaitankNode();
}

std::unique_ptr<TaitankSafeNode> node_create() {
  return std::unique_ptr<TaitankSafeNode>(new TaitankSafeNode());
}

void TaitankSafeNode::set_width(double width) const {
  taitank::SetWidth(ptr, width);
}

void TaitankSafeNode::set_height(double height) const {
  taitank::SetHeight(ptr, height);
}

void TaitankSafeNode::do_layout(double parent_width, double parent_height) const {
  taitank::DoLayout(ptr, parent_width, parent_height);
}

double TaitankSafeNode::get_left() const {
  return taitank::GetLeft(ptr);
}

double TaitankSafeNode::get_top() const {
  return taitank::GetTop(ptr);
}

double TaitankSafeNode::get_width() const {
  return taitank::GetWidth(ptr);
}

double TaitankSafeNode::get_height() const {
  return taitank::GetHeight(ptr);
}
