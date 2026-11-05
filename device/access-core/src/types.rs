//! Basic value types shared by the whole access core.

/// Monotonic time in milliseconds, e.g. since device boot.
///
/// The core never reads a clock itself: the device agent passes the current
/// `Millis` with every input. This keeps the core deterministic and testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Millis(pub u64);

impl Millis {
    /// Returns `self + ms`, clamped to `u64::MAX` instead of overflowing.
    pub const fn saturating_add(self, ms: u64) -> Millis {
        Millis(self.0.saturating_add(ms))
    }
}

/// Direction of passage through the turnstile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Into the gym.
    Entry,
    /// Out of the gym.
    Exit,
}

/// Maximum RFID card UID length in bytes (ISO/IEC 14443: 4, 7 or 10 bytes).
pub const MAX_UID_LEN: usize = 10;

/// An RFID card UID.
///
/// Stored in a fixed-size array so the core needs no heap allocation.
/// A UID identifies a card; it is **not** proof of authenticity (UIDs can be cloned).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardUid {
    // Invariant: `len` is 4, 7 or 10, and bytes after `len` are zero.
    // The fields are private so this invariant can only be created through `CardUid::new`.
    bytes: [u8; MAX_UID_LEN],
    len: u8,
}

/// Why a byte slice could not be turned into a [`CardUid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardUidError {
    /// The UID was not 4, 7 or 10 bytes long. Contains the rejected length.
    InvalidLength(usize),
}

impl CardUid {
    /// Creates a UID from raw bytes read from the card.
    pub fn new(raw: &[u8]) -> Result<CardUid, CardUidError> {
        match raw.len() {
            4 | 7 | 10 => {
                let mut bytes = [0u8; MAX_UID_LEN];
                bytes[..raw.len()].copy_from_slice(raw);
                Ok(CardUid {
                    bytes,
                    len: raw.len() as u8,
                })
            }
            other => Err(CardUidError::InvalidLength(other)),
        }
    }

    /// The UID bytes, without padding.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }
}

/// Why access was denied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenyReason {
    /// The card is not registered.
    UnknownCredential,
    /// The card was revoked by an administrator.
    CredentialRevoked,
    /// The member account is suspended.
    MemberSuspended,
    /// The member has no membership valid right now.
    NoValidMembership,
    /// The member is already inside (entry) — see ADR-0012.
    AntiPassback,
    /// This device is disabled in the backend.
    DeviceDisabled,
    /// Offline and the cached credential snapshot is too old — see ADR-0009.
    OfflineCacheStale,
    /// No decision arrived in time.
    AuthorizationTimeout,
    /// The turnstile is in a fault state.
    DeviceFault,
}

/// Who made an authorization decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionSource {
    /// The backend answered.
    Online,
    /// The device's cached credential snapshot was used.
    OfflineCache,
    /// The core decided by itself (timeout, fault).
    Local,
}

/// The result of an authorization request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthDecision {
    /// Access allowed: one rotation in the requested direction.
    Granted {
        /// Who granted access.
        source: DecisionSource,
    },
    /// Access refused.
    Denied {
        /// Why access was refused.
        reason: DenyReason,
        /// Who refused access.
        source: DecisionSource,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_uid_accepts_iso_14443_lengths() {
        for len in [4, 7, 10] {
            let raw = [0xAB; MAX_UID_LEN];
            let uid = CardUid::new(&raw[..len]).expect("valid length");
            assert_eq!(uid.as_bytes(), &raw[..len]);
        }
    }

    #[test]
    fn card_uid_rejects_other_lengths() {
        for len in [0, 1, 5, 8, 11] {
            let raw = [0u8; 11];
            assert_eq!(
                CardUid::new(&raw[..len]),
                Err(CardUidError::InvalidLength(len))
            );
        }
    }

    #[test]
    fn card_uids_with_same_prefix_but_different_length_differ() {
        let short = CardUid::new(&[1, 2, 3, 4]).unwrap();
        let long = CardUid::new(&[1, 2, 3, 4, 0, 0, 0]).unwrap();
        assert_ne!(short, long);
    }

    #[test]
    fn millis_saturating_add_does_not_overflow() {
        assert_eq!(Millis(1_000).saturating_add(30_000), Millis(31_000));
        assert_eq!(Millis(u64::MAX).saturating_add(1), Millis(u64::MAX));
    }
}
