use crate::*;

#[derive(Reflect)]
pub struct BinaryHeap<T>
where
    T: Reflect,
{
    inner: T,
}

impl<T> BinaryHeap<T> where T: Reflect {}
