#[cfg(all(tokio_unstable, feature = "tracing"))]
macro_rules! trace {
    ($($arg:tt)+) => { tracing_lib::trace!($($arg)+) };
}

#[cfg(all(tokio_unstable, feature = "tracing"))]
macro_rules! trace_span {
    ($($arg:tt)+) => {
        tracing_lib::trace_span!($($arg)+)
    };
}

#[cfg(all(tokio_unstable, feature = "tracing"))]
macro_rules! debug {
    ($($arg:tt)+) => { tracing_lib::debug!($($arg)+) };
}

#[cfg(all(tokio_unstable, feature = "tracing"))]
macro_rules! warn {
    ($($arg:tt)+) => { tracing_lib::warn!($($arg)+) };
}

#[cfg(not(all(tokio_unstable, feature = "tracing")))]
macro_rules! trace {
    ($($arg:tt)+) => {};
}

#[cfg(not(all(tokio_unstable, feature = "tracing")))]
macro_rules! debug {
    ($($arg:tt)+) => {};
}

#[cfg(not(all(tokio_unstable, feature = "tracing")))]
macro_rules! warn {
    ($($arg:tt)+) => {};
}

#[cfg(not(all(tokio_unstable, feature = "tracing")))]
macro_rules! trace_span {
    ($($arg:tt)+) => {
        println!("none");
        crate::util::tracing_lib::Span::none()
    };
}
