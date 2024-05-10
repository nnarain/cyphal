use crate::{CanError, CanResult};
use cyphal::TransferId;

const MAX_TRANSFER_ID: u8 = 31;

/// Represents the Transfer ID used by the CAN transport.
#[derive(Debug, Default, Copy, Clone)]
pub struct CanTransferId {
    value: u8,
}

impl CanTransferId {
    /// Constructs a new CAN Transfer ID
    pub fn new(value: u8) -> CanResult<Self> {
        if value > MAX_TRANSFER_ID {
            return Err(CanError::InvalidId);
        }

        Ok(CanTransferId { value })
    }
}

impl TransferId<u8> for CanTransferId {
    fn value(&self) -> u8 {
        self.value
    }

    fn next(&self) -> Self {
        if self.value < MAX_TRANSFER_ID {
            CanTransferId {
                value: self.value + 1,
            }
        } else {
            CanTransferId { value: 0 }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::CanTransferId;
    use cyphal::TransferId;
    use std::vec::Vec;

    #[test]
    fn test_new() {
        let value: u8 = 5;
        let id = CanTransferId::new(value).unwrap();

        assert_eq!(id.value, value);
    }

    #[test]
    fn test_new_error() {
        let value: u8 = 32;
        let result = CanTransferId::new(value);

        assert!(result.is_err());
    }

    #[test]
    fn test_default() {
        let id = CanTransferId::default();

        assert_eq!(id.value, 0);
    }

    #[test]
    fn test_next() {
        let mut id = CanTransferId::default();
        let data: Vec<u8> = (0..255).collect();

        for i in data {
            assert_eq!(id.value, i % 32);
            id = id.next();
        }
    }
}
