#include <iostream>
#include "taitank-safe/include/safe.h"
#include "taitank-safe/include/taitank/src/taitank_util.h"
#include "taitank-safe/include/taitank/src/taitank_node.h"
#include "taitank-safe/include/taitank/src/taitank.h"

TaitankSafeNode::TaitankSafeNode() {
  ptr = new taitank::TaitankNode();
}

TaitankSafeNode::~TaitankSafeNode() {
  taitank::NodeFreeRecursive(ptr);
}

std::unique_ptr<TaitankSafeNode> node_create() {
  return std::unique_ptr<TaitankSafeNode>(new TaitankSafeNode());
}

void set_width(const std::unique_ptr<TaitankSafeNode> & node, double width) {
  taitank::SetWidth(node->ptr, width);
}

void set_height(const std::unique_ptr<TaitankSafeNode> & node, double height) {
  taitank::SetHeight(node->ptr, height);
}

void set_direction(const std::unique_ptr<TaitankSafeNode> & node, int direction) {
  switch (direction) {
    case 0: {
      taitank::SetDirection(node->ptr, taitank::TaitankDirection::DIRECTION_INHERIT);
      break;
    }
    case 1: {
      taitank::SetDirection(node->ptr, taitank::TaitankDirection::DIRECTION_LTR);
      break;
    }
    case 2: {
      taitank::SetDirection(node->ptr, taitank::TaitankDirection::DIRECTION_RTL);
      break;
    }
  }
}

void set_flex(const std::unique_ptr<TaitankSafeNode> & node, double flex) {
  taitank::SetFlex(node->ptr, flex);
}

void set_flex_grow(const std::unique_ptr<TaitankSafeNode> & node, double flex_grow) {
  taitank::SetFlexGrow(node->ptr, flex_grow);
}

void set_flex_shrink(const std::unique_ptr<TaitankSafeNode> & node, double flex_shrink) {
  taitank::SetFlexShrink(node->ptr, flex_shrink);
}

void set_flex_basis(const std::unique_ptr<TaitankSafeNode> & node, double flex_basis) {
  taitank::SetFlexBasis(node->ptr, flex_basis);
}

void insert_child(const std::unique_ptr<TaitankSafeNode> & node, const std::unique_ptr<TaitankSafeNode> & child, int index) {
  taitank::InsertChild(node->ptr, child->ptr, index);
}

void do_layout(const std::unique_ptr<TaitankSafeNode> & node, double parent_width, double parent_height) {
  taitank::DoLayout(node->ptr, parent_width, parent_height);
}

double get_width(const std::unique_ptr<TaitankSafeNode> & node) {
  return taitank::GetWidth(node->ptr);
}

double get_height(const std::unique_ptr<TaitankSafeNode> & node) {
  return taitank::GetHeight(node->ptr);
}

double get_left(const std::unique_ptr<TaitankSafeNode> & node) {
  return taitank::GetLeft(node->ptr);
}

double get_top(const std::unique_ptr<TaitankSafeNode> & node) {
  return taitank::GetTop(node->ptr);
}
//
//void TaitankSafeNode::set_width(double width) const {
//  taitank::SetWidth(ptr, width);
//}
//
//void TaitankSafeNode::set_height(double height) const {
//  taitank::SetHeight(ptr, height);
//}
//
//void TaitankSafeNode::set_direction(int direction) const {
//  switch (direction) {
//    case 0: {
//      taitank::SetDirection(ptr, taitank::TaitankDirection::DIRECTION_INHERIT);
//      break;
//    }
//    case 1: {
//      taitank::SetDirection(ptr, taitank::TaitankDirection::DIRECTION_LTR);
//      break;
//    }
//    case 2: {
//      taitank::SetDirection(ptr, taitank::TaitankDirection::DIRECTION_RTL);
//      break;
//    }
//  }
//}
//
//void TaitankSafeNode::do_layout(double parent_width, double parent_height) const {
//  taitank::DoLayout(ptr, parent_width, parent_height);
//}
//
//double TaitankSafeNode::get_left() const {
//  return taitank::GetLeft(ptr);
//}
//
//double TaitankSafeNode::get_top() const {
//  return taitank::GetTop(ptr);
//}
//
//double TaitankSafeNode::get_width() const {
//  return taitank::GetWidth(ptr);
//}
//
//double TaitankSafeNode::get_height() const {
//  return taitank::GetHeight(ptr);
//}
