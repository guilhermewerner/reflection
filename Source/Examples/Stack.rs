use crate::*;

#[derive(Reflect)]
pub struct Stack<T>
where
    T: Reflect,
{
    inner: Vec<T>,
}

impl<T> Default for Stack<T>
where
    T: Reflect,
{
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T> Stack<T>
where
    T: Reflect,
{
    #[function]
    pub fn New() -> Self {
        Self::default()
    }

    #[function]
    pub fn Push(&mut self, element: T) {
        self.inner.push(element);
    }

    #[function]
    pub fn Peek(&mut self) -> Option<&T> {
        self.inner.last()
    }

    #[function]
    pub fn PeekMut(&mut self) -> Option<&mut T> {
        self.inner.last_mut()
    }

    #[function]
    pub fn Pop(&mut self) -> Option<T> {
        self.inner.pop()
    }
}
