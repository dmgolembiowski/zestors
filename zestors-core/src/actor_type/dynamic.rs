use crate::*;
use std::{any::TypeId, marker::PhantomData};

//------------------------------------------------------------------------------------------------
//  Dyn
//------------------------------------------------------------------------------------------------

/// A dynamic [ActorType], typed by the messages it accepts.
pub struct Dyn<D: ?Sized>(PhantomData<*const D>);

unsafe impl<D: ?Sized> Send for Dyn<D> {}
unsafe impl<D: ?Sized> Sync for Dyn<D> {}

//------------------------------------------------------------------------------------------------
//  IntoDynamic
//------------------------------------------------------------------------------------------------

/// Marker trait that signifies whether an address can be converted to a dynamic [AddressType] `T`.
pub trait IntoDynamic<T> {}

//------------------------------------------------------------------------------------------------
//  IsDynamic
//------------------------------------------------------------------------------------------------

/// Trait implemented for all dynamic [AddressType]s.
pub trait IsDynamic {
    /// Get all message-ids that are accepted by this [AddressType].
    fn message_ids() -> Box<[TypeId]>;
}

//------------------------------------------------------------------------------------------------
//  Dynamic types
//------------------------------------------------------------------------------------------------

macro_rules! dyn_types {
    ($($ident:ident $(<$( $ty:ident ),*>)?),*) => {
        $(
            // Create the trait

            /// A dynamic address-type.
            pub trait $ident< $($($ty: Message,)?)*>: $($( ProtocolMessage<$ty> + )?)* {}

            // Implement `IsDyn` for it
            impl<$($($ty: Message + 'static,)?)*> IsDynamic for Dyn<dyn $ident< $($($ty,)?)*>> {
                fn message_ids() -> Box<[TypeId]> {
                    Box::new([$($(TypeId::of::<$ty>(),)?)*])
                }
            }

            // Implement `IntoDyn` for all dynamic address-types
            impl<T, $($($ty: Message,)?)*> IntoDynamic<Dyn<dyn $ident<$($($ty,)?)*>>> for Dyn<T>
            where
                T: ?Sized $($( + ProtocolMessage<$ty> )?)* {}

            // Implement `IntoDyn` for all static address-types
            impl<T, $($($ty: Message,)?)*> IntoDynamic<Dyn<dyn $ident<$($($ty,)?)*>>> for T
            where
                T: Protocol $($( + ProtocolMessage<$ty> )?)* {}
        )*
    };
}

dyn_types! {
    AcceptsNone,
    AcceptsOne<M1>,
    AcceptsTwo<M1, M2>,
    AcceptsThree<M1, M2, M3>,
    AcceptsFour<M1, M2, M3, M4>,
    AcceptsFive<M1, M2, M3, M4, M5>,
    AcceptsSix<M1, M2, M3, M4, M5, M6>,
    AcceptsSeven<M1, M2, M3, M4, M5, M6, M7>,
    AcceptsEight<M1, M2, M3, M4, M5, M6, M7, M8>,
    AcceptsNine<M1, M2, M3, M4, M5, M6, M7, M8, M9>,
    AcceptsTen<M1, M2, M3, M4, M5, M6, M7, M8, M9, M10>
}

//------------------------------------------------------------------------------------------------
//  IntoAddress
//------------------------------------------------------------------------------------------------

pub trait IntoAddress<T: ActorType> {
    fn into_address(self) -> Address<T>;
}

impl<R: ?Sized, T: ?Sized> IntoAddress<Dyn<T>> for Address<Dyn<R>>
where
    Dyn<R>: ActorType<Channel = dyn BoxChannel> + IntoDynamic<Dyn<T>>,
{
    fn into_address(self) -> Address<Dyn<T>> {
        self.transform()
    }
}

impl<P, T> IntoAddress<Dyn<T>> for Address<P>
where
    P: Protocol + IntoDynamic<Dyn<T>>,
    T: ?Sized,
{
    fn into_address(self) -> Address<Dyn<T>> {
        self.into_dyn()
    }
}

//------------------------------------------------------------------------------------------------
//  DynAddress
//------------------------------------------------------------------------------------------------

/// A macro to easily create dynamic [Address]es.
///
/// See [DynAccepts!] for creating dynamic [ActorType]s.
///
/// # Examples
/// * `DynAddress![]` == `Address<Dyn<dyn AcceptsNone>>`
/// * `DynAddress![u32, u64]` == `Address<Dyn<dyn AcceptsTwo<u32, u64>>>`
#[macro_export]
macro_rules! DynAddress {
    ($($ty:ty),*) => {
        $crate::process::Address<$crate::DynAccepts![$($ty),*]>
    };
}

//------------------------------------------------------------------------------------------------
//  DynAccepts
//------------------------------------------------------------------------------------------------

/// A macro to easily create dynamic [ActorType]s.
///
/// See [DynAddress!] for creating dynamic [Address]es.
///
/// # Examples
/// * `DynAccepts![u32, u64]` == `Dyn<dyn AcceptsTwo<u32, u64>>`
/// * `Address<DynAccepts![]>` == `Address<Dyn<dyn AcceptsNone>>`
/// * `Address<DynAccepts![u32, u64]>` == `Address<Dyn<dyn AcceptsTwo<u32, u64>>>`
#[macro_export]
macro_rules! DynAccepts {
    () => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsNone>
    };
    ($ty1:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsOne<$ty1>>
    };
    ($ty1:ty, $ty2:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsTwo<$ty1, $ty2>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsThree<$ty1, $ty2, $ty3>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsFour<$ty1, $ty2, $ty3, $ty4>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsFive<$ty1, $ty2, $ty3, $ty4, $ty5>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsSix<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsSeven<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, $ty7>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty, $ty8:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsEight<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, $ty7, $ty8>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty, $ty8:ty, $ty9:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsNine<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, $ty7, $ty8, $ty9>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty, $ty8:ty, $ty9:ty, $ty10:ty) => {
        $crate::actor_type::Dyn<dyn $crate::actor_type::AcceptsTen<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, $ty7, $ty8, $ty9, $ty10>>
    };
}

#[cfg(test)]
mod test {
    use std::any::TypeId;

    use crate::IsDynamic;

    #[test]
    fn address_macro_compiles() {
        let _a: DynAddress![];
        let _a: DynAddress![u8];
        let _a: DynAddress![u8, u16];
        let _a: DynAddress![u8, u16, u32];
        let _a: DynAddress![u8, u16, u32, u64];
        let _a: DynAddress![u8, u16, u32, u64, u128];
        let _a: DynAddress![u8, u16, u32, u64, u128, i8];
        let _a: DynAddress![u8, u16, u32, u64, u128, i8, i16];
        let _a: DynAddress![u8, u16, u32, u64, u128, i8, i16, i32];
        let _a: DynAddress![u8, u16, u32, u64, u128, i8, i16, i32, i64];
        let _a: DynAddress![u8, u16, u32, u64, u128, i8, i16, i32, i64, i128];
    }

    #[test]
    fn message_ids() {
        assert_eq!(
            <DynAccepts![] as IsDynamic>::message_ids(),
            Box::new([]) as Box<[TypeId]>
        );

        assert_eq!(
            <DynAccepts![u32, u64] as IsDynamic>::message_ids(),
            Box::new([TypeId::of::<u32>(), TypeId::of::<u64>()]) as Box<[TypeId]>
        );

        assert_eq!(
            <DynAccepts![u32, u64, i8] as IsDynamic>::message_ids(),
            Box::new([TypeId::of::<u32>(), TypeId::of::<u64>(), TypeId::of::<i8>()])
                as Box<[TypeId]>
        );
    }
}
