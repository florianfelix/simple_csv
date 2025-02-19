use std::{
    cmp::Ordering,
    hash::Hasher,
    ops::{Deref, DerefMut},
};

#[derive(Copy, Clone, Debug)]
pub struct Float(f64);
impl Float {
    /// Construct a new [`Float`].
    pub fn new(v: f64) -> Self {
        Float(v)
    }

    /// Returns the wrapped float.
    pub fn get(self) -> f64 {
        self.0
    }
}

impl<T> AsRef<T> for Float
where
    T: ?Sized,
    <Float as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl Deref for Float {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self::new(value as f64)
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_nan() && other.0.is_nan() || self.0 == other.0
    }
}

impl Eq for Float {}

impl std::hash::Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.to_bits());
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            _ => self.0.partial_cmp(&other.0),
        }
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Bug: Contract violation")
    }
}
