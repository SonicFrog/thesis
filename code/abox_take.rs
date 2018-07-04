pub fn replace_with<F>(&self, f: F) where F: Fn(Arc<T>) -> T;

pub fn get(&self) -> Arc<T>;
