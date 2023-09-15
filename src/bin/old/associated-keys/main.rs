mod envelopes;
mod mapped;
mod timestamp;

use std::collections::HashMap;

use envelopes::Envelope;
use timestamp::Timestamp;

/// Returns the latest timestamped envelope for each key.
#[allow(dead_code)]
fn latest_by_key<'a, E: Envelope>(
    mut envelopes: Vec<(Timestamp, &'a E)>,
) -> HashMap<E::Key, &'a E> {
    let mut result = HashMap::new();
    envelopes.sort_unstable_by(|(ref ts1, _), (ref ts2, _)| ts1.cmp(&ts2));
    for (_timestamp, envelope) in envelopes {
        result.insert(envelope.key().clone(), envelope);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::envelopes::{MappedPaymentEnvelope, PaymentKey};

    use super::*;

    #[test]
    fn test_latest_by_key() {
        let key: PaymentKey = "some-id".into();

        let later_payment = MappedPaymentEnvelope {
            entity: mapped::MappedPayment {},
            wieldy_id: key.clone(),
        };

        let early_payment = MappedPaymentEnvelope {
            entity: mapped::MappedPayment {},
            wieldy_id: key.clone(),
        };

        let arg = vec![
            ("2023-09-14T07:54:39".into(), &later_payment),
            ("2023-08-01T00:00:00".into(), &early_payment),
        ];

        let mut want = HashMap::new();
        want.insert(key, &later_payment);

        let got = latest_by_key(arg);
        assert_eq!(got.len(), 1);
        for (&got, &want) in got.values().zip(want.values()) {
            assert_eq!(got as *const _, want as *const _);
        }
    }
}

fn main() {}
