/// A struct used to hold the bucket for as long as the value is needed
pub struct ValueHolder<K, V> {
    bucket: Arc<BucketEntry<K, V>>
}

impl<K, V> Deref for ValueHolder<K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.bucket.value_ref()
    }
}
