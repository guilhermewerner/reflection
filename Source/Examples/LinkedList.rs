use crate::*;

#[derive(Reflect)]
pub struct LinkedList<T>
where
    T: Reflect,
{
    inner: T,
}

impl<T> LinkedList<T> where T: Reflect {}
