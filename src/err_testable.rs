/// Trait for checking function return types for errors.
/// Generalizes [`is_err`] method of [`Result<T, E>`]
/// and [`is_none`] method of [`Option<T>`].
///
/// [`is_err`]: ::std::result::Result::is_err
/// [`is_none`]: ::std::option::Option::is_none
pub trait ErrTestable {
    /// Returns `true` if [`self`] indicates error
    /// or absence of value
    fn is_err(&self) -> bool {
        false
    }
}

impl<T, E: ::core::fmt::Debug> ErrTestable for Result<T, E> {
    fn is_err(&self) -> bool {
        self.is_err()
    }
}

impl<T> ErrTestable for Option<T> {
    fn is_err(&self) -> bool {
        self.is_none()
    }
}

impl ErrTestable for () {}
