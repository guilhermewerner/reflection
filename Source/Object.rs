use crate::{Class, Reflect};
use anyhow::Result;
use std::sync::Arc;

#[derive(Reflect, Clone)]
pub struct Object {
    inner: Arc<dyn Reflect>,
}

impl Object {
    pub fn New(obj: impl Reflect) -> Self {
        Self {
            inner: Arc::new(obj),
        }
    }

    pub fn InstanceOf(&self, class: &Class) -> bool {
        self.inner.as_ref().type_id() == class.GetId()
    }

    pub fn GetClass(&self) -> Result<()> {
        Ok(())
    }

    pub fn GetProperty(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod Test {
    use super::*;
    use crate::{Class, Reflect};

    struct Foo {}
    struct Bar {}

    unsafe impl Reflect for Foo {
        fn TypeName(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
    }

    unsafe impl Reflect for Bar {
        fn TypeName(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
    }

    #[test]
    fn InstanceOf() {
        let foo_class: Class = Class::New::<Foo>();
        let bar_class: Class = Class::New::<Bar>();
        let foo_instance: Object = Object::New(Foo {});

        assert!(foo_instance.InstanceOf(&foo_class));
        assert!(!foo_instance.InstanceOf(&bar_class));
    }
}
