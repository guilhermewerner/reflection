use crate::{Class, Reflect};
use anyhow::Result;
use std::sync::Arc;

pub struct Object {
    inner: Arc<dyn Reflect>,
}

impl Object {
    fn New(obj: impl Reflect) -> Self {
        Self {
            inner: Arc::new(obj),
        }
    }

    fn InstanceOf(&self, class: &Class) -> bool {
        self.inner.as_ref().type_id() == class.GetId()
    }

    fn GetClass(&self) -> Result<()> {
        Ok(())
    }

    pub fn GetProperty(&self) -> Result<()> {
        Ok(())
    }
}
