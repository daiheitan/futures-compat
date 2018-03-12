use std::sync::Arc;

use futures_compat_01::{Async as Async01, Future as Future01, Poll as Poll01};
use futures_compat_01::task::{self as task01, Task as Task01};

use futures_compat_02::{Async as Async02, Future as Future02};
use futures_compat_02::task::{Context, LocalMap, Wake, Waker};
use futures_compat_02::executor::{Executor as Executor02};

pub struct Future02As01<E, F> {
    exec: E,
    v02: F,
}

pub trait FutureInto01: Future02 {
    fn into_01_compat<E>(self, exec: E) -> Future02As01<E, Self>
    where
        Self: Sized,
        E: Executor02;
}

impl<F> FutureInto01 for F
where
    F: Future02,
{
    fn into_01_compat<E>(self, exec: E) -> Future02As01<E, Self>
    where
        Self: Sized,
        E: Executor02,
    {
        Future02As01 {
            exec,
            v02: self,
        }
    }
}

impl<E, F> Future01 for Future02As01<E, F>
where
    F: Future02,
    E: Executor02,
{
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll01<Self::Item, Self::Error> {
        let mut locals = LocalMap::new();
        let waker = current_as_waker();
        let mut cx = Context::new(&mut locals, &waker, &mut self.exec);

        match self.v02.poll(&mut cx) {
            Ok(Async02::Ready(val)) => Ok(Async01::Ready(val)),
            Ok(Async02::Pending) => Ok(Async01::NotReady),
            Err(err) => Err(err),
        }
    }
}

// Maybe it's possible to do all this without cloning and allocating,
// but I just wanted to get this working now. Optimzations welcome.

fn current_as_waker() -> Waker {
    Waker::from(Arc::new(Current(task01::current())))
}

struct Current(Task01);

impl Wake for Current {
    fn wake(arc_self: &Arc<Self>) {
        arc_self.0.notify();
    }
}