#include "taitank-safe/include/taitank_safe.h"
#include <functional>
#include <string>

#include "taitank-safe/include/taitank/src/taitank_util.h"
#include "taitank-safe/include/taitank/src/taitank_node.h"
#include "taitank-safe/include/taitank/src/taitank.h"


TaitankSafeNode::TaitankSafeNode() {
  ref = new taitank::TaitankNode();
}

std::unique_ptr<TaitankSafeNode> node_create() {
  return std::unique_ptr<TaitankSafeNode>(new TaitankSafeNode());
}

void TaitankSafeNode::set_width(double width) const {
  taitank::SetWidth(ref, width);
}

void TaitankSafeNode::set_height(double height) const {
  taitank::SetHeight(ref, height);
}

void TaitankSafeNode::do_layout(double parent_width, double parent_height) const {
  taitank::DoLayout(ref, parent_width, parent_height);
}

double TaitankSafeNode::get_top() const {
  return taitank::GetTop(ref);
}

double TaitankSafeNode::get_width() const {
  return taitank::GetWidth(ref);
}
