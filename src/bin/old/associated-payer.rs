#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Timestamp(String);

// Mapped entities
struct Claim {}
struct Payment {
    payer: String,
}

// Entity keys; i.e., Wieldy ID types.
trait Key: Clone + Eq + Hash + PartialEq {}

#[derive(Clone, Eq, Hash, PartialEq)]
struct ClaimKey(String);

impl Key for ClaimKey {}

#[derive(Clone, Eq, Hash, PartialEq)]
struct PaymentKey(String);

impl Key for PaymentKey {}

// Envelopes that pair mapped entities with metadata such as Wieldy IDs.
trait Envelope {
    type Entity;
    type Key: Key;

    fn key(&self) -> &Self::Key;

    fn payer(&self) -> &str;
}

struct ClaimEnvelope {
    entity: Claim,
    wieldy_id: ClaimKey,
    payer: String,
}

impl Envelope for ClaimEnvelope {
    type Entity = Claim;
    type Key = ClaimKey;

    fn key(&self) -> &Self::Key {
        &self.wieldy_id
    }

    fn payer(&self) -> &str {
        &self.payer
    }
}

struct PaymentEnvelope {
    entity: Payment,
    wieldy_id: PaymentKey,
}

impl Envelope for PaymentEnvelope {
    type Entity = Payment;
    type Key = PaymentKey;

    fn key(&self) -> &Self::Key {
        &self.wieldy_id
    }

    fn payer(&self) -> &str {
        &self.entity.payer
    }
}

/// Returns the latest timestamped envelope for each key.
fn latest_by_key<E: Envelope>(envelopes: &[(Timestamp, E)]) -> HashMap<E::Key, &E> {
    let mut result = HashMap::new();
    for (timestamp, envelope) in envelopes {
        result.insert(envelope.key().clone(), envelope);
    }
    result
}

fn main() {}
