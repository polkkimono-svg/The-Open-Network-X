//! Tests for transaction/message admission, output-queue ordering, and
//! double-delivery prevention per docs/specification/transactions.md §6.

use onx_data_structures::{AccountId, FullAddress, Message, MessageType, WorkchainIdent};
use onx_primitives::{Uint128, Uint256, Uint32, Uint64};
use onx_state_model::AccountType;
use onx_transactions::{
    admit_external_inbound, admit_external_outbound, admit_internal,
    validate_extra_currencies_sorted, OutputQueue, ProcessedMessageTracker,
    TentativeExecutionOutcome, TransactionsError, MAX_TENTATIVE_GAS,
};

fn addr(byte: u8) -> FullAddress {
    FullAddress::new(WorkchainIdent::BASIC, AccountId::from_bytes([byte; 32]))
}

fn base_message(msg_type: MessageType, created_lt: u64) -> Message {
    Message {
        msg_type,
        src_address: addr(0x01),
        dest_address: addr(0x02),
        amount_nanos: Uint128::from(0u128),
        extra_currencies: vec![],
        created_lt: Uint64::from(created_lt),
        body_cell_hash: Uint256([0u8; 32]),
    }
}

#[test]
fn test_extra_currencies_sorting_validation() {
    let sorted = vec![
        (Uint32::from(1u32), Uint128::from(1u128)),
        (Uint32::from(2u32), Uint128::from(1u128)),
    ];
    assert_eq!(validate_extra_currencies_sorted(&sorted), Ok(()));

    let unsorted = vec![
        (Uint32::from(2u32), Uint128::from(1u128)),
        (Uint32::from(1u32), Uint128::from(1u128)),
    ];
    assert_eq!(
        validate_extra_currencies_sorted(&unsorted),
        Err(TransactionsError::UnsortedExtraCurrencies)
    );

    let duplicate = vec![
        (Uint32::from(5u32), Uint128::from(1u128)),
        (Uint32::from(5u32), Uint128::from(2u128)),
    ];
    assert_eq!(
        validate_extra_currencies_sorted(&duplicate),
        Err(TransactionsError::DuplicateCurrencyId { currency_id: 5 })
    );
}

#[test]
fn test_external_inbound_admission_zero_value_and_tentative_execution() {
    let mut msg = base_message(MessageType::ExternalInbound, 1);

    let ok_outcome = TentativeExecutionOutcome {
        signature_valid: true,
        gas_used: MAX_TENTATIVE_GAS,
    };
    assert_eq!(admit_external_inbound(&msg, ok_outcome), Ok(()));

    // Non-zero amount_nanos is rejected even with a passing tentative outcome.
    msg.amount_nanos = Uint128::from(1u128);
    assert_eq!(
        admit_external_inbound(&msg, ok_outcome),
        Err(TransactionsError::NonZeroExternalValue)
    );
    msg.amount_nanos = Uint128::from(0u128);

    // Non-empty extra_currencies is likewise rejected.
    msg.extra_currencies = vec![(Uint32::from(1u32), Uint128::from(1u128))];
    assert_eq!(
        admit_external_inbound(&msg, ok_outcome),
        Err(TransactionsError::NonZeroExternalValue)
    );
    msg.extra_currencies = vec![];

    // Failed signature verification is rejected regardless of gas used.
    let bad_signature = TentativeExecutionOutcome {
        signature_valid: false,
        gas_used: 0,
    };
    assert_eq!(
        admit_external_inbound(&msg, bad_signature),
        Err(TransactionsError::TentativeSignatureInvalid)
    );

    // Exceeding MAX_TENTATIVE_GAS is rejected.
    let over_gas = TentativeExecutionOutcome {
        signature_valid: true,
        gas_used: MAX_TENTATIVE_GAS + 1,
    };
    assert_eq!(
        admit_external_inbound(&msg, over_gas),
        Err(TransactionsError::TentativeGasLimitExceeded {
            used: MAX_TENTATIVE_GAS + 1,
            limit: MAX_TENTATIVE_GAS,
        })
    );

    // Wrong message type is rejected before any other check.
    let internal_msg = base_message(MessageType::Internal, 1);
    assert_eq!(
        admit_external_inbound(&internal_msg, ok_outcome),
        Err(TransactionsError::WrongMessageType {
            expected: MessageType::ExternalInbound,
            actual: MessageType::Internal,
        })
    );
}

#[test]
fn test_internal_admission_requires_active_source_account() {
    let msg = base_message(MessageType::Internal, 1);

    assert_eq!(admit_internal(&msg, AccountType::Active), Ok(()));
    assert_eq!(
        admit_internal(&msg, AccountType::Frozen),
        Err(TransactionsError::InternalSourceNotActive)
    );
    assert_eq!(
        admit_internal(&msg, AccountType::Uninitialized),
        Err(TransactionsError::InternalSourceNotActive)
    );
    assert_eq!(
        admit_internal(&msg, AccountType::Destroyed),
        Err(TransactionsError::InternalSourceNotActive)
    );

    let external_msg = base_message(MessageType::ExternalInbound, 1);
    assert_eq!(
        admit_internal(&external_msg, AccountType::Active),
        Err(TransactionsError::WrongMessageType {
            expected: MessageType::Internal,
            actual: MessageType::ExternalInbound,
        })
    );
}

#[test]
fn test_external_outbound_admission() {
    let msg = base_message(MessageType::ExternalOutbound, 1);
    assert_eq!(admit_external_outbound(&msg), Ok(()));

    let wrong_type = base_message(MessageType::Internal, 1);
    assert_eq!(
        admit_external_outbound(&wrong_type),
        Err(TransactionsError::WrongMessageType {
            expected: MessageType::ExternalOutbound,
            actual: MessageType::Internal,
        })
    );
}

#[test]
fn test_output_queue_fifo_ordering_and_regression_rejection() {
    let mut queue = OutputQueue::new();

    let msg1 = base_message(MessageType::Internal, 10);
    let msg2 = base_message(MessageType::Internal, 20);
    queue.enqueue(msg1.clone()).unwrap();
    queue.enqueue(msg2.clone()).unwrap();

    // A message with created_lt <= the last one queued for the same pair is rejected.
    let regressed = base_message(MessageType::Internal, 20);
    assert_eq!(
        queue.enqueue(regressed),
        Err(TransactionsError::LogicalTimeRegression {
            previous_lt: 20,
            next_lt: 20,
        })
    );

    assert_eq!(queue.len(), 2);
    assert_eq!(queue.pop_next(), Some(msg1));
    assert_eq!(queue.pop_next(), Some(msg2));
    assert_eq!(queue.pop_next(), None);
    assert!(queue.is_empty());
}

#[test]
fn test_output_queue_global_lt_order_across_lanes() {
    let mut queue = OutputQueue::new();

    // Lane A -> B gets the later message; lane C -> D gets the earlier one.
    let mut msg_ab = base_message(MessageType::Internal, 50);
    msg_ab.src_address = addr(0xA);
    msg_ab.dest_address = addr(0xB);

    let mut msg_cd = base_message(MessageType::Internal, 5);
    msg_cd.src_address = addr(0xC);
    msg_cd.dest_address = addr(0xD);

    queue.enqueue(msg_ab.clone()).unwrap();
    queue.enqueue(msg_cd.clone()).unwrap();

    // The globally lowest created_lt is delivered first, regardless of lane.
    assert_eq!(queue.pop_next(), Some(msg_cd));
    assert_eq!(queue.pop_next(), Some(msg_ab));
}

#[test]
fn test_validate_delivery_order_rejects_fifo_violation() {
    let msg1 = base_message(MessageType::Internal, 10);
    let msg2 = base_message(MessageType::Internal, 20);
    assert_eq!(
        OutputQueue::validate_delivery_order(&[msg1.clone(), msg2.clone()]),
        Ok(())
    );

    // Same (src, dest) pair delivered out of lt order is rejected.
    assert_eq!(
        OutputQueue::validate_delivery_order(&[msg2, msg1]),
        Err(TransactionsError::FifoOrderViolation {
            previous_lt: 20,
            next_lt: 10,
        })
    );
}

#[test]
fn test_double_delivery_prevention() {
    let mut tracker = ProcessedMessageTracker::new();
    let msg = base_message(MessageType::Internal, 1);
    let hash = msg.message_hash();

    assert!(!tracker.contains(&hash));
    assert_eq!(tracker.admit_message(&msg), Ok(()));
    assert!(tracker.contains(&hash));

    // Re-admitting the same message hash is rejected.
    assert_eq!(
        tracker.admit_message(&msg),
        Err(TransactionsError::DuplicateDelivery)
    );

    // Pruning allows the hash to be admitted again (e.g. once the
    // originating shard confirms removal from its output queue).
    tracker.prune(&hash);
    assert!(!tracker.contains(&hash));
    assert_eq!(tracker.admit_message(&msg), Ok(()));
}
