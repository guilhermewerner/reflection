use crate::{Object, Value};
use anyhow::Result;
use std::sync::Arc;

pub struct Property(Arc<dyn Fn(&Object) -> Result<Value>>);
