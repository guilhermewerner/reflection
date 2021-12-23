use crate::*;

#[derive(Reflect)]
pub struct Foo {
    #[property(visible, editable, category = "Default")]
    pub a: u32,

    #[property(visible, editable, category = "Default")]
    pub b: Bar,

    #[property(visible, editable, category = "Default")]
    pub c: Vec<u128>,

    #[property(visible, editable, category = "Default")]
    pub d: Vec<Bar>,
}

impl Foo {
    #[function(callable, category = "Default")]
    pub fn Func(&mut self) {}
}

#[derive(Reflect)]
pub struct Bar {
    #[property(visible, editable, category = "Default")]
    pub value: f32,
}

impl Bar {
    #[function(callable, category = "Default")]
    pub fn Func(&mut self) {}
}
