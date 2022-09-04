use crate::*;
use futures::Future;
use tiny_actor::{Config, Inbox};

/// See [tiny_actor::spawn]
pub fn spawn<P, E, Fun, Fut>(config: Config, fun: Fun) -> (Child<E, P>, Address<P>)
where
    Fun: FnOnce(Inbox<P>) -> Fut + Send + 'static,
    Fut: Future<Output = E> + Send + 'static,
    E: Send + 'static,
    P: Protocol,
{
    let (child, addr) = tiny_actor::spawn(config, fun);
    (Child::from_inner(child), Address::from_inner(addr))
}

/// See [tiny_actor::spawn_one]
pub fn spawn_one<P, E, Fun, Fut>(config: Config, fun: Fun) -> (ChildPool<E, P>, Address<P>)
where
    Fun: FnOnce(Inbox<P>) -> Fut + Send + 'static,
    Fut: Future<Output = E> + Send + 'static,
    E: Send + 'static,
    P: Protocol,
{
    let (child, addr) = tiny_actor::spawn_one(config, fun);
    (ChildPool::from_inner(child), Address::from_inner(addr))
}

/// See [tiny_actor::spawn_many]
pub fn spawn_many<P, E, I, Fun, Fut>(
    iter: impl IntoIterator<Item = I>,
    config: Config,
    fun: Fun,
) -> (ChildPool<E, P>, Address<P>)
where
    Fun: FnOnce(I, Inbox<P>) -> Fut + Send + 'static + Clone,
    Fut: Future<Output = E> + Send + 'static,
    E: Send + 'static,
    P: Protocol,
    I: Send + 'static,
{
    let (child, addr) = tiny_actor::spawn_many(iter, config, fun);
    (ChildPool::from_inner(child), Address::from_inner(addr))
}
