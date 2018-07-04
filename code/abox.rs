pub struct AtomicBox<T: Sized>
{
    ptr: AtomicPtr<T>,
}
