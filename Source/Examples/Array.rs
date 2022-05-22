use crate::*;

#[derive(Reflect)]
pub struct Array<T>
where
    T: Reflect,
{
    inner: Vec<T>,
}

impl<T> Default for Array<T>
where
    T: Reflect,
{
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T> Array<T>
where
    T: Reflect,
{
    #[function]
    pub fn New() -> Self {
        Self::default()
    }

    #[function]
    pub fn Get(&mut self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    #[function]
    pub fn GetMut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }
}
