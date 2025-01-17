#[macro_use]
extern crate zestors;

use std::any::TypeId;
use zestors::{
    messaging::{BoxedMessage, Protocol, ProtocolMessage},
    Message,
};
use zestors_request::{new_request, Rx};

#[test]
fn basic_macros() {
    #[derive(Message, Debug, Clone)]
    #[msg(Rx<()>)]
    pub struct Msg1;

    #[derive(Message)]
    pub struct Msg2;

    #[derive(Message)]
    #[msg(Rx<u32>)]
    pub enum Msg3 {
        Variant,
    }

    #[derive(Message)]
    pub enum Msg4 {
        Variant,
    }

    #[protocol]
    enum Prot {
        One(Msg1),
        Two(Msg2),
        Three(Msg3),
        Four(Msg4),
    }

    Prot::One((Msg1, new_request().0));
    Prot::Two(Msg2);
    Prot::Three((Msg3::Variant, new_request().0));
    Prot::Four(Msg4::Variant);

    <Prot as Protocol>::try_unbox(BoxedMessage::new::<Msg1>((Msg1, new_request().0))).unwrap();
    <Prot as Protocol>::try_unbox(BoxedMessage::new::<Msg2>(Msg2)).unwrap();
    <Prot as Protocol>::try_unbox(BoxedMessage::new::<Msg3>((Msg3::Variant, new_request().0)))
        .unwrap();
    <Prot as Protocol>::try_unbox(BoxedMessage::new::<Msg4>(Msg4::Variant)).unwrap();

    assert!(<Prot as Protocol>::accepts(&TypeId::of::<Msg1>()));
    assert!(<Prot as Protocol>::accepts(&TypeId::of::<Msg2>()));
    assert!(<Prot as Protocol>::accepts(&TypeId::of::<Msg3>()));
    assert!(<Prot as Protocol>::accepts(&TypeId::of::<Msg4>()));
    assert!(!<Prot as Protocol>::accepts(&TypeId::of::<u32>()));

    <Prot as ProtocolMessage<Msg1>>::from_sends((Msg1, new_request().0));
    <Prot as ProtocolMessage<Msg2>>::from_sends(Msg2);
    <Prot as ProtocolMessage<Msg3>>::from_sends((Msg3::Variant, new_request().0));
    <Prot as ProtocolMessage<Msg4>>::from_sends(Msg4::Variant);
}
