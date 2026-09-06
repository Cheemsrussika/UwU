use std::collections::HashMap;
use std::sync::RwLock;
use super::chunk::Chunk;

pub struct ConcurrentChunkStorage {
    shards: [RwLock<HashMap<(i32, i32, i32), Chunk>>; 16],
}

impl ConcurrentChunkStorage {
    pub fn new() -> Self {
        Self {
            shards: std::array::from_fn(|_| RwLock::new(HashMap::new())),
        }
    }

    #[inline]
    fn shard_idx(coords: &(i32, i32, i32)) -> usize {
        let (x, y, z) = *coords;
        let mut h = (x as u64).wrapping_mul(0x9E3779B97F4A7C15);
        h ^= (y as u64).wrapping_mul(0xC2B2AE3D27D4EB4F);
        h ^= (z as u64).wrapping_mul(0x165667B19E3779F9);
        (h as usize) % 16
    }

    pub fn get_chunk_read<F, R>(&self, coords: &(i32, i32, i32), f: F) -> Option<R>
    where
        F: FnOnce(&Chunk) -> R,
    {
        let shard = self.shards[Self::shard_idx(coords)].read().unwrap();
        shard.get(coords).map(f)
    }

    pub fn get_chunk_write<F, R>(&self, coords: &(i32, i32, i32), f: F) -> Option<R>
    where
        F: FnOnce(&mut Chunk) -> R,
    {
        let mut shard = self.shards[Self::shard_idx(coords)].write().unwrap();
        shard.get_mut(coords).map(f)
    }

    pub fn insert(&self, coords: (i32, i32, i32), chunk: Chunk) {
        let mut shard = self.shards[Self::shard_idx(&coords)].write().unwrap();
        shard.insert(coords, chunk);
    }

    pub fn get_all_keys(&self) -> Vec<(i32, i32, i32)> {
        let mut keys = Vec::new();
        for shard in &self.shards {
            let s = shard.read().unwrap();
            keys.extend(s.keys().copied());
        }
        keys
    }

    pub fn for_each_chunk<F>(&self, mut f: F)
    where
        F: FnMut(&(i32, i32, i32), &Chunk),
    {
        for shard in &self.shards {
            let s = shard.read().unwrap();
            for (k, v) in s.iter() {
                f(k, v);
            }
        }
    }
}