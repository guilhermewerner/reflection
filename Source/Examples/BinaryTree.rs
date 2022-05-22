use crate::*;

#[derive(Reflect)]
pub struct BinaryTree<T>
where
    T: Reflect,
{
    inner: T,
}

impl<T> BinaryTree<T> where T: Reflect {}
