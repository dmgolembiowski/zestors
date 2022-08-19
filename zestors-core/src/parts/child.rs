use crate::*;

pub struct Child<E: Send + 'static, C: DynChannel + ?Sized>(InnerChild<E, C>);

impl<E, C> Child<E, C>
where
    E: Send + 'static,
    C: DynChannel + ?Sized,
{
    pub(crate) fn from_tiny_child(child: InnerChild<E, C>) -> Self {
        Self(child)
    }

    gen::dyn_send_methods!();
}

impl<E, P> Child<E, Channel<P>>
where
    E: Send + 'static,
    P: Send + 'static
{
    gen::send_methods!();
}