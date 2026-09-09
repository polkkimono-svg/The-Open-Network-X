//! ONX transaction and messaging semantics.
//!
//! Implements `docs/specification/transactions.md` (ADR-0005): external and
//! internal message admission rules including tentative execution of
//! External Inbound messages, `extra_currencies` value-model validation,
//! the output-queue-only delivery architecture with per-account FIFO
//! ordering, and double-delivery prevention via `processed_msg_hashes`.
//!
//! Hypercube routing (§3.4) is explicitly out of scope here: the baseline
//! protocol defers even its "slow path" transit-fee accounting to the
//! execution layer, and Instant Hypercube Routing ("fast path") is deferred
//! to ONX-ARCH-011.

pub mod admission;
pub mod error;
pub mod output_queue;
pub mod processed;

pub use admission::{
    admit_external_inbound, admit_external_outbound, admit_internal,
    validate_extra_currencies_sorted, validate_message_shape, TentativeExecutionOutcome,
    MAX_TENTATIVE_GAS,
};
pub use error::TransactionsError;
pub use output_queue::OutputQueue;
pub use processed::ProcessedMessageTracker;
