
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct HeapBuffer<T>{
    inner:Box<[T]>

}

impl HeapBuffer<T>{
    fn new(data:Vec<T>)->Self{
        Self{inner:data.into_boxed_slice()}
    }
}

impl Deref for HeapBuffer<T>{
    type Target = [T];
    fn deref(&self)->&Self::Target{
        &self.inner
    }
}

impl Drop for HeapBuffer<T>{
    fn drop(&mut self){
        DROP_COUNTER.fetch_add(1, Ordering::SeqCst);

        println!("HeapBuffer is being dropped");
    }
}

impl<T> DerefMut for HeapBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


//- Write a test that creates a `HeapBuffer`, mutates an element through `DerefMut`, drops it, and asserts the drop ran.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_buffer_drop() {
        DROP_COUNTER.store(0, Ordering::SeqCst);

        {
            let mut buffer = HeapBuffer::new(vec![1, 2, 3]);

            // DerefMut test
            buffer[0] = 10;
            assert_eq!(buffer[0], 10);
        } // buffer goes out of scope → Drop runs

        assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 1);
    }
}
