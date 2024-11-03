#include "taitank-safe/include/taitank_safe.h"
#include "taitank-safe/src/lib.rs.h"
#include <functional>
#include <string>

#include "taitank-safe/include/taitank/src/taitank_util.h"
#include "taitank-safe/include/taitank/src/taitank.h"


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

  // Pretend we did something useful to persist the data.
  auto blobid = std::hash<std::string>{}(contents);
  return blobid;
}