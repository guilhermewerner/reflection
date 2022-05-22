use crate::*;
use std::collections::HashSet as InnerHashSet;

#[derive(Reflect)]
pub struct HashSet<T>
where
    T: Reflect,
{
    inner: InnerHashSet<T>,
}

impl<T> HashSet<T> where T: Reflect {}
