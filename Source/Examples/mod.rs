use crate::*;

#[derive(Reflect)]
pub struct Foo {
    #[property(visible, editable, category = "Default")]
    pub a: u32,

    #[property(visible, editable, readwrite, category = "Default")]
    pub b: Bar,

    #[property(visible, readonly, category = "Default")]
    pub c: Vec<u128>,

    #[property(hidden)]
    pub d: Vec<Bar>,
}

impl Foo {
    #[function(callable, multicast, category = "Default")]
    pub fn Func(&mut self) {}
}

#[derive(Reflect)]
pub struct Bar {
    #[property(visible, editable, category = "Default")]
    pub value: f32,
}

impl Bar {
    #[function(event, server, reliable, category = "Default")]
    pub fn Func(&mut self) {}
}
