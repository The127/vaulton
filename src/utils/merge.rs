/// Defines behavior for merging one instance into another.
///
/// This trait allows an instance to be updated with values from another instance
/// of the same type, following type-specific merge rules.
pub trait Merge {
    /// Merges another instance into this one, potentially modifying the current instance.
    fn merge(&mut self, other: Self);
}

/// Implementation of Merge for Option<T> that only updates if the other value is Some.
///
/// This implementation will:
/// - Replace the current value if `other` is `Some`
/// - Keep the current value if `other` is `None`
impl<T: Clone> Merge for Option<T> {
    fn merge(&mut self, other: Self) {
        if other.is_some() {
            *self = other;
        }
    }
}