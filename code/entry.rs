struct BucketEntry<K, V> {
    key: K,
    value: V,
}

impl<K, V> BucketEntry<K, V> {
    fn key_matches(&self, key: &K) -> bool;
    fn value_ref(&self) -> &V;
    fn value(self) -> V;
}
