use std::future::Future;
use std::pin::Pin;

pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |input| f(g(input))
}

pub fn async_compose<F, G, FutF, FutG, A, B, C>(
    f: F,
    g: G,
) -> impl Fn(A) -> Pin<Box<dyn Future<Output = C>>>
where
    F: Fn(B) -> FutF + Copy + 'static,
    G: Fn(A) -> FutG + Copy + 'static,
    FutF: Future<Output = C> + 'static,
    FutG: Future<Output = B> + 'static,
    A: 'static,
{
    move |input| {
        Box::pin(async move {
            let intermediate = g(input).await;
            f(intermediate).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::{Ready, ready};

    #[test]
    fn composes_sync_functions_right_to_left() {
        let add_one = |value| value + 1;
        let double = |value| value * 2;

        let function = compose(double, add_one);

        assert_eq!(function(5), 12);
    }

    #[test]
    fn composes_async_functions_right_to_left() {
        let add_async = |value| ready(value + 1);
        let double_async = |value| -> Ready<i32> { ready(value * 2) };

        let function = async_compose(double_async, add_async);

        assert_eq!(pollster(function(5)), 12);
    }

    fn pollster<F>(future: F) -> F::Output
    where
        F: Future,
    {
        use std::task::{Context, Poll, Waker};

        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut future = std::pin::pin!(future);

        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => output,
            Poll::Pending => panic!("expected test future to complete immediately"),
        }
    }
}
