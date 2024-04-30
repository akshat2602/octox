use crate::ticketlock::TicketLock;
use alloc::vec::Vec;
use core::hash::Hasher;

// Custom implementation of a simple hash map entry.
struct Entry<K, V> {
    key: K,
    value: V,
}

// Concurrent hash map implementation.
pub struct ConcurrentHashMap<K, V> {
    buckets: Vec<Option<TicketLock<Entry<K, V>>>>,
    size: usize,
}

impl<K, V> ConcurrentHashMap<K, V>
where
    K: Eq + core::hash::Hash,
{
    // Create a new concurrent hash map with specified initial capacity.
    pub fn new(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(None);
        }
        ConcurrentHashMap { buckets, size: 0 }
    }

    // Helper function to get the bucket index based on the hash of the key.
    fn get_bucket_index(&self, key: &K) -> usize {
        let hash = self.hash(key);
        hash % self.buckets.len()
    }

    // Insert a key-value pair into the hash map.
    pub fn insert(&mut self, key: K, value: V) {
        let index = self.get_bucket_index(&key);

        // Initialize the TicketLock if it's not already initialized.
        if self.buckets[index].is_none() {
            self.buckets[index] =
                Some(TicketLock::new(Entry { key, value }, "concurrent_hash_map"));
        }
        // Call the lock method to acquire the lock.
        let ticket_lock = self.buckets[index].as_ref().unwrap();
        let mut _guard = ticket_lock.lock();
        self.size += 1;
    }

    // Retrieve a value associated with the given key from the hash map.
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_bucket_index(key);

        if let Some(ticket_lock) = &self.buckets[index] {
            let guard = ticket_lock.lock();
            let entry = unsafe { &*guard.lock().get_mut() };
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

// Example FNV-1a hash function implementation (compatible with `no_std` environment).
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
