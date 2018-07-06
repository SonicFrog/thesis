struct AtomicTable<K, V, S>
where
    K: PartialEq + Hash,
    S: BuildHasher,
{
    buckets: Vec<AtomicBucket<K, V>>,
    hash_builder: S,
}

struct AtomicBucket<K, V> {
    value: Arc<AtomicBox<Vec<Arc<BucketEntry<K, V>>>>>,
}
