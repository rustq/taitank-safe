
#[cxx::bridge]
mod ffi {

    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }
    
    unsafe extern "C++" {
        include!("taitank-safe/include/taitank_safe.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(self: &BlobstoreClient, parts: &mut MultiBuf) -> u64;

        type TaitankSafeNode;
        fn node_create() -> UniquePtr<TaitankSafeNode>;
        fn get_w(self: &TaitankSafeNode) -> bool;
    }
}

pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

fn f() {
    let client = ffi::new_blobstore_client();
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node = ffi::node_create();
        assert!(node.get_w() == true);
    }
}