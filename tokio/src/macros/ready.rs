macro_rules! ready {
    ($e:expr, resource = $($rest:tt)+ ) => {
        match $e {
            std::task::Poll::Ready(t) => {
                debug!(task.ready = true, resource = $($rest)+ );
                t,
            }
            std::task::Poll::Pending => {
                debug!(task.pending = true, resource = $($rest)+ );
                return std::task::Poll::Pending;
            }
        }
    };
    ($e:expr $(,)?) => {
        match $e {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
