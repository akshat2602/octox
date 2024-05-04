use alloc::vec::Vec;
use core::hash::Hasher;

use crate::ticketlock::TicketLock;

struct Entry<K, V> {
    key: K,
    value: V,
}

pub static CONCURRENTHASHMAP: TicketLock<HashMap<u32, u32>> =
    TicketLock::new(HashMap::new(), "CONCURRENTHASHMAP");

// Hash map implementation.
pub struct HashMap<K, V> {
    buckets: Vec<Option<Entry<K, V>>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + core::hash::Hash,
{
    pub const fn new() -> Self {
        Self {
            buckets: Vec::new(),
            size: 0,
        }
    }

    pub fn init(&mut self, capacity: usize) {
        self.buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            self.buckets.push(None);
        }
    }

    fn get_bucket_index(&self, key: &K) -> usize {
        self.hash(key) % self.buckets.len()
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.get_bucket_index(&key);

        if self.buckets[index].is_none() {
            self.buckets[index] = Some(Entry { key, value });
            self.size += 1;
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_bucket_index(key);

        if let Some(entry) = &self.buckets[index] {
            if &entry.key == key {
                return Some(&entry.value);
            }
        }

        None
    }

    // Helper function to calculate the hash of a key.
    fn hash<Q: ?Sized + core::hash::Hash>(&self, key: &Q) -> usize {
        let mut hasher = FnvHasher::default();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}

#[derive(Default)]
struct FnvHasher {
    state: u64,
}

impl core::hash::Hasher for FnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        const FNV_PRIME: u64 = 1099511628211;
        // const OFFSET_BASIS: u64 = 14695981039346656037;

        for &byte in bytes {
            self.state ^= u64::from(byte);
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}
