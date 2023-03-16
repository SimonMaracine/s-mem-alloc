mod allocator;
mod allocators;

#[cfg(test)]
mod tests {
    use super::allocator::Alloc;
    use super::allocators::naive::NaiveAllocator;

    #[test]
    fn it_works() {
        let mut naive_allocator = NaiveAllocator::new(1024);

        let _addr = naive_allocator.malloc(128);
    }
}
