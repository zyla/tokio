#[cfg(all(tokio_unstable, feature = "tracing"))]
pub(crate) use tracing_lib::{span::Entered, Span};

#[cfg(not(all(tokio_unstable, feature = "tracing")))]
pub(crate) struct Span {
    _p: (),
}

#[cfg(not(all(tokio_unstable, feature = "tracing")))]
pub(crate) struct Entered<'a> {
    _p: std::marker::PhantomData<&'a ()>,
}

#[cfg(not(all(tokio_unstable, feature = "tracing")))]
impl Span {
    pub(crate) fn none() -> Self {
        Span { _p: () }
    }

    pub(crate) fn is_none(&self) -> bool {
        false
    }

    pub(crate) fn enter(&self) -> Entered<'_> {
        Entered {
            _p: std::marker::PhantomData,
        }
    }
}
