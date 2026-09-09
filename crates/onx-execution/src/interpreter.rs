use crate::types::{Builder, ExceptionKind, ExecutionContext, ExecutionResult, Slice, StackValue};
use onx_data_structures::Message;
use onx_primitives::{
    domain_hash,
    hash::{DomainTag, TX_BODY_V1},
    PublicKey, Signature,
};
use onx_state_model::Cell;

pub struct Interpreter {
    pub stack: Vec<StackValue>,
    pub call_stack: Vec<(Cell, usize)>, // (code_cell, bit_offset)
    pub current_code: Cell,
    pub pc_bits: usize,
    pub data: Cell,
    pub out_messages: Vec<Message>,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub context: ExecutionContext,
    pub code_refs: Vec<Cell>,
}

impl Interpreter {
    pub fn new(code: Cell, data: Cell, _message: Message, context: ExecutionContext) -> Self {
        Self {
            stack: Vec::new(),
            call_stack: Vec::new(),
            current_code: code,
            pc_bits: 0,
            data,
            out_messages: Vec::new(),
            gas_limit: context.gas_limit,
            gas_used: 0,
            context,
            code_refs: Vec::new(),
        }
    }

    pub fn consume_gas(&mut self, amount: u64) -> Result<(), ExceptionKind> {
        let new_gas = self.gas_used.saturating_add(amount);
        if new_gas > self.gas_limit {
            self.gas_used = self.gas_limit;
            Err(ExceptionKind::OutOfGas)
        } else {
            self.gas_used = new_gas;
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<StackValue, ExceptionKind> {
        self.stack.pop().ok_or(ExceptionKind::MalformedCell)
    }

    pub fn pop_integer(&mut self) -> Result<[u8; 32], ExceptionKind> {
        match self.pop()? {
            StackValue::Integer(bytes) => Ok(bytes),
            _ => Err(ExceptionKind::TypeMismatch),
        }
    }

    pub fn pop_bytes(&mut self) -> Result<Vec<u8>, ExceptionKind> {
        match self.pop()? {
            StackValue::Bytes(bytes) => Ok(bytes),
            _ => Err(ExceptionKind::TypeMismatch),
        }
    }

    pub fn pop_cell(&mut self) -> Result<Cell, ExceptionKind> {
        match self.pop()? {
            StackValue::Cell(cell) => Ok(cell),
            _ => Err(ExceptionKind::TypeMismatch),
        }
    }

    pub fn pop_slice(&mut self) -> Result<Slice, ExceptionKind> {
        match self.pop()? {
            StackValue::Slice(slice) => Ok(slice),
            _ => Err(ExceptionKind::TypeMismatch),
        }
    }

    pub fn pop_builder(&mut self) -> Result<Builder, ExceptionKind> {
        match self.pop()? {
            StackValue::Builder(builder) => Ok(builder),
            _ => Err(ExceptionKind::TypeMismatch),
        }
    }

    pub fn read_uint8(&mut self) -> Result<u8, ExceptionKind> {
        let total_bits = self.current_code.data_bytes().len() * 8;
        if self.pc_bits + 8 > total_bits {
            return Err(ExceptionKind::MalformedCell);
        }
        let byte_idx = self.pc_bits / 8;
        let bit_rem = self.pc_bits % 8;
        let data = self.current_code.data_bytes();
        let val = if bit_rem == 0 {
            data[byte_idx]
        } else {
            let b1 = data[byte_idx];
            let b2 = data.get(byte_idx + 1).copied().unwrap_or(0);
            (b1 << bit_rem) | (b2 >> (8 - bit_rem))
        };
        self.pc_bits += 8;
        Ok(val)
    }

    pub fn read_uint16(&mut self) -> Result<u16, ExceptionKind> {
        let high = self.read_uint8()? as u16;
        let low = self.read_uint8()? as u16;
        Ok((high << 8) | low)
    }

    pub fn read_bytes_exact(&mut self, len: usize) -> Result<Vec<u8>, ExceptionKind> {
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(self.read_uint8()?);
        }
        Ok(res)
    }

    pub fn step(&mut self) -> Result<bool, ExceptionKind> {
        let total_bits = self.current_code.data_bytes().len() * 8;
        if self.pc_bits >= total_bits {
            if let Some((prev_code, prev_pc)) = self.call_stack.pop() {
                self.current_code = prev_code;
                self.pc_bits = prev_pc;
                return Ok(true);
            } else {
                return Ok(false); // Execution finished successfully
            }
        }

        let opcode = self.read_uint8()?;
        match opcode {
            // 0x00: NOP
            0x00 => {
                self.consume_gas(1)?;
            }
            // 0x01: DROP
            0x01 => {
                self.consume_gas(1)?;
                self.pop()?;
            }
            // 0x02: DUP
            0x02 => {
                self.consume_gas(1)?;
                let top = self
                    .stack
                    .last()
                    .ok_or(ExceptionKind::MalformedCell)?
                    .clone();
                self.stack.push(top);
            }
            // 0x03: SWAP
            0x03 => {
                self.consume_gas(1)?;
                let a = self.pop()?;
                let b = self.pop()?;
                self.stack.push(a);
                self.stack.push(b);
            }
            // 0x04: OVER
            0x04 => {
                self.consume_gas(1)?;
                let len = self.stack.len();
                if len < 2 {
                    return Err(ExceptionKind::MalformedCell);
                }
                let second = self.stack[len - 2].clone();
                self.stack.push(second);
            }
            // 0x05: ROT
            0x05 => {
                self.consume_gas(1)?;
                let c = self.pop()?;
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(b);
                self.stack.push(c);
                self.stack.push(a);
            }
            // 0x06: PICK depth
            0x06 => {
                self.consume_gas(1)?;
                let depth = self.read_uint8()? as usize;
                let len = self.stack.len();
                if depth >= len {
                    return Err(ExceptionKind::MalformedCell);
                }
                let item = self.stack[len - 1 - depth].clone();
                self.stack.push(item);
            }
            // 0x07: ROLL depth
            0x07 => {
                self.consume_gas(1)?;
                let depth = self.read_uint8()? as usize;
                let len = self.stack.len();
                if depth >= len {
                    return Err(ExceptionKind::MalformedCell);
                }
                let item = self.stack.remove(len - 1 - depth);
                self.stack.push(item);
            }
            // 0x08: PUSHINT signed, value[32]
            0x08 => {
                self.consume_gas(1)?;
                let _signed = self.read_uint8()?;
                let bytes = self.read_bytes_exact(32)?;
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&bytes);
                self.stack.push(StackValue::Integer(arr));
            }
            // 0x09: PUSHBYTES len[uint16], bytes
            0x09 => {
                let len = self.read_uint16()? as usize;
                let gas_cost = 1 + len.div_ceil(32);
                self.consume_gas(gas_cost as u64)?;
                let bytes = self.read_bytes_exact(len)?;
                self.stack.push(StackValue::Bytes(bytes));
            }
            // Arithmetic 0x10-0x15
            0x10..=0x15 => {
                self.consume_gas(if opcode == 0x13 || opcode == 0x14 {
                    8
                } else {
                    4
                })?;
                let width = self.read_uint16()?;
                let _flavor = self.read_uint8()?;
                if width == 0 || width > 256 {
                    return Err(ExceptionKind::MalformedCell);
                }
                match opcode {
                    0x10 => {
                        // ADD
                        let b = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        let a = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        let res = a.wrapping_add(b);
                        self.stack.push(StackValue::from_i128(res));
                    }
                    0x11 => {
                        // SUB
                        let b = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        let a = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        let res = a.wrapping_sub(b);
                        self.stack.push(StackValue::from_i128(res));
                    }
                    0x12 => {
                        // NEG
                        let a = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        self.stack.push(StackValue::from_i128(a.wrapping_neg()));
                    }
                    0x13 => {
                        // MUL
                        let b = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        let a = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        self.stack.push(StackValue::from_i128(a.wrapping_mul(b)));
                    }
                    0x14 => {
                        // DIVMOD
                        let b = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        let a = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        if b == 0 {
                            return Err(ExceptionKind::IntegerOverflow);
                        }
                        let q = a / b;
                        let r = a % b;
                        self.stack.push(StackValue::from_i128(q));
                        self.stack.push(StackValue::from_i128(r));
                    }
                    0x15 => {
                        // CMP
                        let b = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        let a = StackValue::Integer(self.pop_integer()?).to_i128()?;
                        let r = if a < b {
                            -1
                        } else if a > b {
                            1
                        } else {
                            0
                        };
                        self.stack.push(StackValue::from_i128(r));
                    }
                    _ => unreachable!(),
                }
            }
            0x16 => {
                // ISZERO
                self.consume_gas(4)?;
                let a = self.pop_integer()?;
                let is_zero = a == [0u8; 32];
                self.stack
                    .push(StackValue::from_i128(if is_zero { 1 } else { 0 }));
            }
            0x20 => {
                // CONV width, signed
                self.consume_gas(4)?;
                let _width = self.read_uint16()?;
                let _signed = self.read_uint8()?;
                let a = self.pop_integer()?;
                self.stack.push(StackValue::Integer(a));
            }
            // Byte/bit string operations 0x30-0x33
            0x30 => {
                // BYTELEN
                self.consume_gas(1)?;
                let bytes = self.pop_bytes()?;
                self.stack.push(StackValue::from_i128(bytes.len() as i128));
            }
            0x31 => {
                // CONCAT
                let b = self.pop_bytes()?;
                let a = self.pop_bytes()?;
                let total_len = a.len() + b.len();
                self.consume_gas(4 + total_len.div_ceil(32) as u64)?;
                let mut res = a;
                res.extend(b);
                self.stack.push(StackValue::Bytes(res));
            }
            0x32 => {
                // SUBBYTES
                self.consume_gas(4)?;
                let len = StackValue::Integer(self.pop_integer()?).to_i128()? as usize;
                let offset = StackValue::Integer(self.pop_integer()?).to_i128()? as usize;
                let bytes = self.pop_bytes()?;
                if offset + len > bytes.len() {
                    return Err(ExceptionKind::MalformedCell);
                }
                self.stack
                    .push(StackValue::Bytes(bytes[offset..offset + len].to_vec()));
            }
            0x33 => {
                // BYTEEQ
                let b = self.pop_bytes()?;
                let a = self.pop_bytes()?;
                let min_len = a.len().min(b.len());
                self.consume_gas(1 + min_len.div_ceil(32) as u64)?;
                self.stack
                    .push(StackValue::from_i128(if a == b { 1 } else { 0 }));
            }
            // Cell access 0x40-0x4C
            0x40 => {
                // NEWC
                self.consume_gas(10)?;
                self.stack.push(StackValue::Builder(Builder::default()));
            }
            0x41 => {
                // ENDC
                self.consume_gas(10)?;
                let builder = self.pop_builder()?;
                let cell_refs = builder.references.iter().map(|c| c.hash()).collect();
                let cell = Cell::new(builder.data_bytes, cell_refs)
                    .map_err(|_| ExceptionKind::MalformedCell)?;
                self.stack.push(StackValue::Cell(cell));
            }
            0x42 => {
                // STBITS width, signed
                self.consume_gas(10)?;
                let width = self.read_uint16()? as usize;
                let _signed = self.read_uint8()?;
                let val_bytes = self.pop_integer()?;
                let mut builder = self.pop_builder()?;
                builder.append_bits(&val_bytes, width)?;
                self.stack.push(StackValue::Builder(builder));
            }
            0x43 => {
                // STREF
                self.consume_gas(10)?;
                let cell = self.pop_cell()?;
                let mut builder = self.pop_builder()?;
                if builder.references.len() >= 4 {
                    return Err(ExceptionKind::MalformedCell);
                }
                builder.references.push(cell);
                self.stack.push(StackValue::Builder(builder));
            }
            0x44 => {
                // STBYTES
                let bytes = self.pop_bytes()?;
                self.consume_gas(10 + bytes.len().div_ceil(32) as u64)?;
                let mut builder = self.pop_builder()?;
                if builder.data_bytes.len() + bytes.len() > 128 {
                    return Err(ExceptionKind::MalformedCell);
                }
                builder.data_bytes.extend(bytes);
                self.stack.push(StackValue::Builder(builder));
            }
            0x45 => {
                // CTOS
                self.consume_gas(10)?;
                let cell = self.pop_cell()?;
                if cell.is_special() {
                    return Err(ExceptionKind::AbsentNode);
                }
                self.stack.push(StackValue::Slice(Slice::new(cell)));
            }
            0x46 => {
                // LDU width
                self.consume_gas(10)?;
                let width = self.read_uint16()? as usize;
                let mut slice = self.pop_slice()?;
                let val_bytes = slice.read_bits(width)?;
                self.stack.push(StackValue::Slice(slice));
                self.stack.push(StackValue::Integer(val_bytes));
            }
            0x47 => {
                // LDI width
                self.consume_gas(10)?;
                let width = self.read_uint16()? as usize;
                let mut slice = self.pop_slice()?;
                let val_bytes = slice.read_bits(width)?;
                self.stack.push(StackValue::Slice(slice));
                self.stack.push(StackValue::Integer(val_bytes));
            }
            0x48 => {
                // LDREF
                self.consume_gas(10)?;
                let mut slice = self.pop_slice()?;
                if slice.remaining_refs() == 0 {
                    return Err(ExceptionKind::MalformedCell);
                }
                let ref_cell = if slice.ref_offset < slice.child_cells.len() {
                    slice.child_cells[slice.ref_offset].clone()
                } else {
                    let ref_hash = slice.cell.cell_refs()[slice.ref_offset];
                    Cell::new(vec![], vec![ref_hash]).unwrap()
                };
                slice.ref_offset += 1;
                self.stack.push(StackValue::Slice(slice));
                self.stack.push(StackValue::Cell(ref_cell));
            }
            0x49 => {
                // ISEXOTIC
                self.consume_gas(10)?;
                let cell = self.pop_cell()?;
                self.stack
                    .push(StackValue::from_i128(if cell.is_special() { 1 } else { 0 }));
            }
            0x4A => {
                // SEMPTY
                self.consume_gas(1)?;
                let slice = self.pop_slice()?;
                let empty = slice.remaining_bits() == 0 && slice.remaining_refs() == 0;
                self.stack
                    .push(StackValue::from_i128(if empty { 1 } else { 0 }));
            }
            0x4B => {
                // SBITS
                self.consume_gas(1)?;
                let slice = self.pop_slice()?;
                self.stack
                    .push(StackValue::from_i128(slice.remaining_bits() as i128));
            }
            0x4C => {
                // SREFS
                self.consume_gas(1)?;
                let slice = self.pop_slice()?;
                self.stack
                    .push(StackValue::from_i128(slice.remaining_refs() as i128));
            }
            // Cryptographic 0x60-0x62
            0x60 => {
                // HASHBYTES
                self.consume_gas(200)?;
                let bytes = self.pop_bytes()?;
                let tag = DomainTag::from_ascii("ONX_EXEC_HASH_V1");
                let hash = domain_hash(&tag, &bytes);
                self.stack.push(StackValue::Integer(hash));
            }
            0x61 => {
                // HASHCELL
                self.consume_gas(200)?;
                let cell = self.pop_cell()?;
                let hash = cell.hash();
                self.stack.push(StackValue::Integer(hash));
            }
            0x62 => {
                // CHKSIGNU
                self.consume_gas(4000)?;
                let hash32 = self.pop_integer()?;
                let sig_bytes = self.pop_bytes()?;
                let pubkey_bytes = self.pop_bytes()?;
                if pubkey_bytes.len() != 32 || sig_bytes.len() != 64 {
                    return Err(ExceptionKind::TypeMismatch);
                }
                let pubkey = PublicKey::decode_exact(&pubkey_bytes);
                let sig = Signature::decode_exact(&sig_bytes);
                let valid = if let (Ok(pk), Ok(s)) = (pubkey, sig) {
                    pk.verify(&TX_BODY_V1, &hash32, &s).is_ok()
                } else {
                    false
                };
                self.stack
                    .push(StackValue::from_i128(if valid { 1 } else { 0 }));
            }
            // Control flow 0x70-0x71, 0x73-0x76
            0x70 | 0x71 | 0x73 | 0x74 | 0x75 | 0x76 => {
                self.consume_gas(4)?;
                let ref_idx = self.read_uint8()? as usize;
                let condition = match opcode {
                    0x70 | 0x71 => true,
                    0x73 | 0x75 => StackValue::Integer(self.pop_integer()?).to_i128()? != 0,
                    0x74 | 0x76 => StackValue::Integer(self.pop_integer()?).to_i128()? == 0,
                    _ => unreachable!(),
                };
                if condition {
                    if ref_idx >= self.code_refs.len() {
                        return Err(ExceptionKind::MalformedCell);
                    }
                    let target_code = self.code_refs[ref_idx].clone();
                    if opcode == 0x71 || opcode == 0x75 || opcode == 0x76 {
                        // CALL variants
                        self.call_stack
                            .push((self.current_code.clone(), self.pc_bits));
                    }
                    self.current_code = target_code;
                    self.pc_bits = 0;
                }
            }
            0x72 => {
                // RET
                self.consume_gas(4)?;
                if let Some((prev_code, prev_pc)) = self.call_stack.pop() {
                    self.current_code = prev_code;
                    self.pc_bits = prev_pc;
                } else {
                    return Ok(false); // Execution finished successfully
                }
            }
            0x77 => {
                // THROW kind
                self.consume_gas(4)?;
                let kind_byte = self.read_uint8()?;
                let kind = match kind_byte {
                    0 => ExceptionKind::IntegerOverflow,
                    1 => ExceptionKind::AbsentNode,
                    2 => ExceptionKind::MalformedCell,
                    3 => ExceptionKind::TypeMismatch,
                    _ => return Err(ExceptionKind::MalformedCell),
                };
                return Err(kind);
            }
            _ => return Err(ExceptionKind::MalformedCell),
        }

        Ok(true)
    }

    pub fn run(&mut self) -> ExecutionResult {
        loop {
            match self.step() {
                Ok(true) => continue,
                Ok(false) => {
                    return ExecutionResult::Success {
                        new_data: self.data.clone(),
                        out_messages: self.out_messages.clone(),
                        gas_used: self.gas_used,
                    };
                }
                Err(kind) => {
                    return ExecutionResult::Exception {
                        kind,
                        gas_used: self.gas_used,
                    };
                }
            }
        }
    }
}
