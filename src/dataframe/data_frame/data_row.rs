use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use super::DataValue;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DataRow(Vec<DataValue>);

impl Deref for DataRow {
    type Target = Vec<DataValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for DataRow
where
    T: ?Sized,
    <DataRow as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl DataRow {
    pub fn new(len: usize) -> Self {
        Self(vec![DataValue::Null; len])
    }
}
