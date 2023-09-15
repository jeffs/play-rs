//! Entity keys; i.e., Wieldy ID types.

use crate::mapped::{MappedClaim, MappedPayment};

use core::fmt::Debug;
use std::hash::Hash;

pub trait Key: Clone + Debug + Eq + Hash + PartialEq {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ClaimKey(String);

impl Key for ClaimKey {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PaymentKey(String);

impl Key for PaymentKey {}

impl<T: Into<String>> From<T> for PaymentKey {
    fn from(value: T) -> Self {
        PaymentKey(value.into())
    }
}

// Envelopes that pair mapped entities with metadata such as Wieldy IDs.
pub trait Envelope: Debug {
    type Entity;
    type Key: Key;

    fn key(&self) -> &Self::Key;
}

#[derive(Debug)]
pub struct MappedClaimEnvelope {
    pub entity: MappedClaim,
    pub wieldy_id: ClaimKey,
}

impl Envelope for MappedClaimEnvelope {
    type Entity = MappedClaim;
    type Key = ClaimKey;

    fn key(&self) -> &Self::Key {
        &self.wieldy_id
    }
}

#[derive(Debug)]
pub struct MappedPaymentEnvelope {
    pub entity: MappedPayment,
    pub wieldy_id: PaymentKey,
}

impl Envelope for MappedPaymentEnvelope {
    type Entity = MappedPayment;
    type Key = PaymentKey;

    fn key(&self) -> &Self::Key {
        &self.wieldy_id
    }
}
