#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/src/main.rs.h"
#include <functional>
#include <string>

#include "cxx-demo/include/taitank/src/taitank_util.h"
#include "cxx-demo/include/taitank/src/taitank.h"
#include "cxx-demo/include/mylib/mb.h"


BlobstoreClient::BlobstoreClient() {}

std::unique_ptr<BlobstoreClient> new_blobstore_client() {
  return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}

uint64_t BlobstoreClient::put(MultiBuf &buf) const {
  // Traverse the caller's chunk iterator.
  std::string contents;
  while (true) {
    auto chunk = next_chunk(buf);
    if (chunk.size() == 0) {
      break;
    }
    contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size());
  }

  taitank::TaitankNodeRef node = taitank::NodeCreate();
  taitank::SetWidth(node, 100);
  taitank::SetHeight(node, 100);
  taitank::DoLayout(node, VALUE_UNDEFINED, VALUE_UNDEFINED);
  printf("node->GetTop() = %f\n", taitank::GetTop(node));
  printf("node->GetWidth() = %f\n", taitank::GetWidth(node));

  printf("taitank::FloatIsEqual(1.0, 1.0)) = %d\n", taitank::FloatIsEqual(1.0, 1.0));
  printf("IsFloat2(1.0) = %d\n", IsFloat2(2.0));

  // Pretend we did something useful to persist the data.
  auto blobid = std::hash<std::string>{}(contents);
  return blobid;
}