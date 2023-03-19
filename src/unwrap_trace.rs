use tracing::warn;

pub trait UnwrapTrace {
    type Value;
    type Error: std::fmt::Debug;

    fn unwrap_or_warn<F>(self, message: &str, op: F) -> Self::Value
    where
        F: FnOnce(Self::Error) -> Self::Value;
}

impl<T, U: std::fmt::Debug> UnwrapTrace for Result<T, U> {
    type Value = T;
    type Error = U;

    fn unwrap_or_warn<F>(self, message: &str, op: F) -> Self::Value
    where
        F: FnOnce(Self::Error) -> Self::Value,
    {
        match self {
            Ok(v) => v,
            Err(e) => {
                warn!(?e, message);
                op(e)
            }
        }
    }
}
