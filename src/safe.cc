#include <iostream>
#include "../include/safe.h"
#include "../include/taitank/src/taitank_util.h"
#include "../include/taitank/src/taitank_node.h"
#include "../include/taitank/src/taitank.h"

TaitankSafeNode::TaitankSafeNode() {
  ptr = new taitank::TaitankNode();
}

TaitankSafeNode::~TaitankSafeNode() {
  //  printf("[cxx]: TaitankSafeNode::~TaitankSafeNode()\n");
  taitank::NodeFreeRecursive(ptr);
}

std::unique_ptr<TaitankSafeNode> node_create() {
  return std::unique_ptr<TaitankSafeNode>(new TaitankSafeNode());
}

void set_width(std::unique_ptr<TaitankSafeNode> & node, double width) {
  taitank::SetWidth(node->ptr, width);
}

void set_height(std::unique_ptr<TaitankSafeNode> & node, double height) {
  taitank::SetHeight(node->ptr, height);
}

void set_direction(std::unique_ptr<TaitankSafeNode> & node, int direction) {
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

void set_flex(std::unique_ptr<TaitankSafeNode> & node, double flex) {
  taitank::SetFlex(node->ptr, flex);
}

void set_flex_grow(std::unique_ptr<TaitankSafeNode> & node, double flex_grow) {
  taitank::SetFlexGrow(node->ptr, flex_grow);
}

void set_flex_shrink(std::unique_ptr<TaitankSafeNode> & node, double flex_shrink) {
  taitank::SetFlexShrink(node->ptr, flex_shrink);
}

void set_flex_basis(std::unique_ptr<TaitankSafeNode> & node, double flex_basis) {
  taitank::SetFlexBasis(node->ptr, flex_basis);
}

void set_flex_direction(std::unique_ptr<TaitankSafeNode> & node, int flex_direction) {
  switch (flex_direction) {
    case 0: {
      taitank::SetFlexDirection(node->ptr, taitank::FlexDirection::FLEX_DIRECTION_ROW);
      break;
    }
    case 1: {
      taitank::SetFlexDirection(node->ptr, taitank::FlexDirection::FLEX_DIRECTION_ROW_REVERSE);
      break;
    }
    case 2: {
      taitank::SetFlexDirection(node->ptr, taitank::FlexDirection::FLEX_DIRECTION_COLUMN);
      break;
    }
    case 3: {
      taitank::SetFlexDirection(node->ptr, taitank::FlexDirection::FLEX_DIRECTION_COLUNM_REVERSE);
      break;
    }
  }
}

void set_flex_wrap(std::unique_ptr<TaitankSafeNode> & node, int flex_wrap_node) {
  switch (flex_wrap_node) {
    case 0: {
      taitank::SetFlexWrap(node->ptr, taitank::FlexWrapMode::FLEX_NO_WRAP);
      break;
    }
    case 1: {
      taitank::SetFlexWrap(node->ptr, taitank::FlexWrapMode::FLEX_WRAP);
      break;
    }
    case 2: {
      taitank::SetFlexWrap(node->ptr, taitank::FlexWrapMode::FLEX_WRAP_REVERSE);
      break;
    }
  }
}


void set_align_items(std::unique_ptr<TaitankSafeNode> & node, int flex_align) {
  switch (flex_align) {
    case 0: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_AUTO);
      break;
    }
    case 1: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_START);
      break;
    }
    case 2: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_CENTER);
      break;
    }
    case 3: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_END);
      break;
    }
    case 4: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_STRETCH);
      break;
    }
    case 5: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_BASE_LINE);
      break;
    }
    case 6: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_BETWEEN);
      break;
    }
    case 7: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_AROUND);
      break;
    }
    case 8: {
      taitank::SetAlignItems(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_EVENLY);
      break;
    }
  }
}


void insert_child(std::unique_ptr<TaitankSafeNode> & node, std::unique_ptr<TaitankSafeNode> & child, int index) {
  taitank::InsertChild(node->ptr, child->ptr, index);
}

void do_layout(std::unique_ptr<TaitankSafeNode> & node, double parent_width, double parent_height, int direction) {
  switch (direction) {
    case 0: {
      taitank::DoLayout(node->ptr, parent_width, parent_height, taitank::TaitankDirection::DIRECTION_INHERIT);
      break;
    }
    case 1: {
      taitank::DoLayout(node->ptr, parent_width, parent_height, taitank::TaitankDirection::DIRECTION_LTR);
      break;
    }
    case 2: {
      taitank::DoLayout(node->ptr, parent_width, parent_height, taitank::TaitankDirection::DIRECTION_RTL);
      break;
    }
  }
}

void set_min_width(std::unique_ptr<TaitankSafeNode> & node, double min_width) {
  taitank::SetMinWidth(node->ptr, min_width);
}

void set_align_content(std::unique_ptr<TaitankSafeNode> & node, int flex_align) {
  switch (flex_align) {
    case 0: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_AUTO);
      break;
    }
    case 1: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_START);
      break;
    }
    case 2: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_CENTER);
      break;
    }
    case 3: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_END);
      break;
    }
    case 4: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_STRETCH);
      break;
    }
    case 5: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_BASE_LINE);
      break;
    }
    case 6: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_BETWEEN);
      break;
    }
    case 7: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_AROUND);
      break;
    }
    case 8: {
      taitank::SetAlignContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_EVENLY);
      break;
    }
  }
}

void set_justify_content(std::unique_ptr<TaitankSafeNode> & node, int justify) {
  switch (justify) {
    case 0: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_AUTO);
      break;
    }
    case 1: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_START);
      break;
    }
    case 2: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_CENTER);
      break;
    }
    case 3: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_END);
      break;
    }
    case 4: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_STRETCH);
      break;
    }
    case 5: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_BASE_LINE);
      break;
    }
    case 6: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_BETWEEN);
      break;
    }
    case 7: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_AROUND);
      break;
    }
    case 8: {
      taitank::SetJustifyContent(node->ptr, taitank::FlexAlign::FLEX_ALIGN_SPACE_EVENLY);
      break;
    }
  }
}

double get_width(std::unique_ptr<TaitankSafeNode> & node) {
  return taitank::GetWidth(node->ptr);
}

double get_height(std::unique_ptr<TaitankSafeNode> & node) {
  return taitank::GetHeight(node->ptr);
}

double get_left(std::unique_ptr<TaitankSafeNode> & node) {
  return taitank::GetLeft(node->ptr);
}

double get_top(std::unique_ptr<TaitankSafeNode> & node) {
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
