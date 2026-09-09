//! Tests for ONX Execution Engine and TVM Instruction Set per docs/specification/tvm-instruction-set.md.

use onx_data_structures::{AccountId, FullAddress, Message, MessageType, WorkchainIdent};
use onx_execution::{
    execute, Builder, ExceptionKind, ExecutionContext, ExecutionResult, Interpreter, Slice,
    StackValue,
};
use onx_primitives::{Uint128, Uint256, Uint64};
use onx_state_model::Cell;

fn dummy_message() -> Message {
    let addr = FullAddress::new(WorkchainIdent::BASIC, AccountId::from_bytes([0x01; 32]));
    Message {
        msg_type: MessageType::Internal,
        src_address: addr,
        dest_address: addr,
        amount_nanos: Uint128::from(1000u128),
        extra_currencies: vec![],
        created_lt: Uint64::from(100u64),
        body_cell_hash: Uint256([0xAA; 32]),
    }
}

fn dummy_context(gas_limit: u64) -> ExecutionContext {
    ExecutionContext {
        gen_utime: 1700000000,
        start_lt: 100,
        end_lt: 200,
        gas_limit,
    }
}

#[test]
fn test_nop_and_pushint_execution() {
    // Code: NOP (0x00), RET (0x72)
    let code = Cell::new(vec![0x00, 0x72], vec![]).unwrap();
    let data = Cell::new(vec![], vec![]).unwrap();
    let res = execute(code, data.clone(), dummy_message(), dummy_context(100));

    match res {
        ExecutionResult::Success {
            new_data, gas_used, ..
        } => {
            assert_eq!(new_data, data);
            assert_eq!(gas_used, 5); // 1 for NOP + 4 for RET
        }
        _ => panic!("Expected successful execution"),
    }
}

#[test]
fn test_absent_node_exception_on_pruned_cell_ctos() {
    // Code: CTOS (0x45)
    let code = Cell::new(vec![0x45], vec![]).unwrap();
    let pruned_cell = Cell::new_with_special(vec![0x00], vec![], true).unwrap();

    let mut interpreter = Interpreter::new(
        code,
        Cell::new(vec![], vec![]).unwrap(),
        dummy_message(),
        dummy_context(100),
    );
    interpreter.stack.push(StackValue::Cell(pruned_cell));
    let res = interpreter.run();

    match res {
        ExecutionResult::Exception { kind, .. } => {
            assert_eq!(kind, ExceptionKind::AbsentNode);
        }
        _ => panic!("Expected AbsentNode exception"),
    }
}

#[test]
fn test_out_of_gas_exception() {
    // Code: NOP, NOP, NOP...
    let code = Cell::new(vec![0x00; 10], vec![]).unwrap();
    let res = execute(
        code,
        Cell::new(vec![], vec![]).unwrap(),
        dummy_message(),
        dummy_context(3),
    );

    match res {
        ExecutionResult::Exception { kind, gas_used } => {
            assert_eq!(kind, ExceptionKind::OutOfGas);
            assert_eq!(gas_used, 3);
        }
        _ => panic!("Expected OutOfGas exception"),
    }
}

#[test]
fn test_slice_ldu_bit_reading() {
    let cell = Cell::new(vec![0xA5], vec![]).unwrap(); // 0xA5 = 1010 0101
    let mut slice = Slice::new(cell);

    // Read 4 bits: should be 1010 = 10
    let bits1 = slice.read_bits(4).unwrap();
    let val1 = bits1[31] & 0x0F;
    assert_eq!(val1, 10);

    // Read remaining 4 bits: should be 0101 = 5
    let bits2 = slice.read_bits(4).unwrap();
    let val2 = bits2[31] & 0x0F;
    assert_eq!(val2, 5);
}

#[test]
fn test_builder_stbits_packing() {
    let mut builder = Builder::default();
    let val_bytes = StackValue::from_i128(0xA5).to_i128().unwrap();
    let mut arr = [0u8; 32];
    arr[31] = val_bytes as u8;

    builder.append_bits(&arr, 8).unwrap();
    assert_eq!(builder.data_bytes, vec![0xA5]);
}

fn pushint(value: i128) -> Vec<u8> {
    let mut code = vec![0x08, 1];
    let mut bytes = [0u8; 32];
    if value < 0 {
        bytes[..16].fill(0xff);
    }
    bytes[16..].copy_from_slice(&value.to_be_bytes());
    code.extend(bytes);
    code
}

#[test]
fn expanded_arithmetic_opcodes_charge_spec_gas() {
    for (opcode, expected) in [(0x17, 14), (0x18, 10), (0x19, 10)] {
        let mut program = pushint(8);
        program.extend(pushint(2));
        program.extend([opcode, 0, 64, 1, 0x72]);
        let result = execute(
            Cell::new(program, vec![]).unwrap(),
            Cell::new(vec![], vec![]).unwrap(),
            dummy_message(),
            dummy_context(100),
        );
        assert!(
            matches!(result, ExecutionResult::Success { gas_used, .. } if gas_used == expected)
        );
    }
}

#[test]
fn expanded_stack_opcodes_charge_one_gas_each() {
    for (opcode, operands, values, expected_depth) in [
        (0x03, vec![], vec![1, 2], 2),
        (0x0a, vec![], vec![1, 2], 1),
        (0x0b, vec![], vec![1, 2], 3),
        (0x0c, vec![1, 1], vec![1, 2], 2),
        (0x07, vec![1], vec![1, 2], 2),
    ] {
        let mut program = Vec::new();
        for value in values {
            program.extend(pushint(value));
        }
        program.push(opcode);
        program.extend(operands);
        program.push(0x72);
        let mut interpreter = Interpreter::new(
            Cell::new(program, vec![]).unwrap(),
            Cell::new(vec![], vec![]).unwrap(),
            dummy_message(),
            dummy_context(100),
        );
        assert!(matches!(
            interpreter.run(),
            ExecutionResult::Success { gas_used: 7, .. }
        ));
        assert_eq!(interpreter.stack.len(), expected_depth);
    }
}

#[test]
fn expanded_conditional_opcodes_charge_four_gas() {
    // IFELSE uses a zero offset so it simply chooses the following RET.
    for program in [
        {
            let mut p = pushint(1);
            p.extend([0x78, 0, 0, 0x72]);
            p
        },
        {
            let mut p = pushint(1);
            p.extend([0x79, 0x72]);
            p
        },
        {
            let mut p = pushint(1);
            p.extend([0x7b, 0, 0x72]);
            p
        },
        vec![0x7a, 0, 0, 0x72],
    ] {
        let expected_gas = match program[0] {
            0x7a => 8,
            0x08 if program[34] == 0x79 => 5,
            _ => 9,
        };
        assert!(
            matches!(execute(Cell::new(program, vec![]).unwrap(), Cell::new(vec![], vec![]).unwrap(), dummy_message(), dummy_context(100)), ExecutionResult::Success { gas_used, .. } if gas_used == expected_gas)
        );
    }
}
