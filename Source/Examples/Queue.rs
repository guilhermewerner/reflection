use crate::*;

#[derive(Reflect)]
pub struct Queue<T>
where
    T: Reflect,
{
    inner: T,
}

impl<T> Queue<T> where T: Reflect {}
