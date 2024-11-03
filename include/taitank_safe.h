#pragma once
#include <memory>
#include "taitank-safe/include/taitank/src/taitank.h"

struct MultiBuf;

class BlobstoreClient {
public:
  BlobstoreClient();
  uint64_t put(MultiBuf &buf) const;
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();

class TaitankSafeNode {
public:
  TaitankSafeNode(taitank::TaitankNodeRef r);
  bool get_w() const;
  void set_width(double width) const;
private:
  taitank::TaitankNodeRef ref;
  float w;
};

std::unique_ptr<TaitankSafeNode> node_create();
