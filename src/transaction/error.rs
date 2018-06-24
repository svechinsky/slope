use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionError {
	/// Error returned when signer doesn't match sender
	InvalidSigner,
	/// Error returned when the transaction casues an invalid state transition
	BadStateTransition,
}

impl fmt::Display for TransactionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::TransactionError::*;
		match *self {
			InvalidSigner => write!(f, "Transaction signer doesn't match from"),
			BadStateTransition => write!(f, "State transition is bad"),
		}
	}
}
