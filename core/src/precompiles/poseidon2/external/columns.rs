use core::borrow::Borrow;
use core::borrow::BorrowMut;
use std::mem::size_of;
use std::ops::Add;

use valida_derive::AlignedBorrow;

use crate::air::Word;
use crate::memory::MemoryReadWriteCols;
use crate::operations::Add5Operation;
use crate::operations::AddOperation;
use crate::operations::AndOperation;
use crate::operations::FixedRotateRightOperation;
use crate::operations::NotOperation;
use crate::operations::XorOperation;

pub const NUM_POSEIDON2_EXTERNAL_COLS: usize = size_of::<Poseidon2ExternalCols<u8>>();
pub const POSEIDON2_DEFAULT_ROUNDS_F: usize = 8;
pub const POSEIDON2_DEFAULT_ROUNDS_P: usize = 22;
pub const POSEIDON2_DEFAULT_EXTERNAL_ROUNDS: usize = POSEIDON2_DEFAULT_ROUNDS_F / 2;

#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Poseidon2ExternalCols<T>(
    pub Poseidon2ExternalColsConfigurable<T, POSEIDON2_DEFAULT_EXTERNAL_ROUNDS>,
);
// TODO: I just copied and pasted these from sha compress as a starting point. Carefully examine
// what is necessary and what is not.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Poseidon2ExternalColsConfigurable<T, const ROUNDS: usize> {
    /// Inputs.
    pub segment: T,
    pub clk: T,
    // pub w_and_h_ptr: T,
    pub state_ptr: T,

    //     /// The bits for cycle 8.
    //     pub octet: [T; 8],
    //
    // This will specify which octet we are currently processing.
    // The first octet is for initialize.
    // The next 8 octets are for compress.
    // The last octet is for finalize.
    // pub octet_num: [T; 10],
    pub mem: [MemoryReadWriteCols<T>; ROUNDS],
    pub mem_addr: [T; ROUNDS],

    // pub a: Word<T>,
    // pub b: Word<T>,
    // pub c: Word<T>,
    // pub d: Word<T>,
    // pub e: Word<T>,
    // pub f: Word<T>,
    // pub g: Word<T>,
    // pub h: Word<T>,

    // pub e_rr_6: FixedRotateRightOperation<T>,
    // pub e_rr_11: FixedRotateRightOperation<T>,
    // pub e_rr_25: FixedRotateRightOperation<T>,
    // pub s1_intermediate: XorOperation<T>,
    // pub s1: XorOperation<T>,

    // pub e_and_f: AndOperation<T>,
    // pub e_not: NotOperation<T>,
    // pub e_not_and_g: AndOperation<T>,
    // pub ch: XorOperation<T>,

    // pub temp1: Add5Operation<T>,

    // pub a_rr_2: FixedRotateRightOperation<T>,
    // pub a_rr_13: FixedRotateRightOperation<T>,
    // pub a_rr_22: FixedRotateRightOperation<T>,
    // pub s0_intermediate: XorOperation<T>,
    // pub s0: XorOperation<T>,

    // pub a_and_b: AndOperation<T>,
    // pub a_and_c: AndOperation<T>,
    // pub b_and_c: AndOperation<T>,
    // pub maj_intermediate: XorOperation<T>,
    // pub maj: XorOperation<T>,

    // pub temp2: AddOperation<T>,

    // pub d_add_temp1: AddOperation<T>,
    // pub temp1_add_temp2: AddOperation<T>,

    // This is a materialized column that will have value of a || b || c ... || h depending on
    // the row of the finalized phase.  This column will need to be verified.
    // Note this is needed since the AddOperation gadget can only accept AB::Var types as inputs.
    // TODO: Modify AddOperation to accept AB::Expr types as inputs.
    pub finalized_operand: Word<T>,
    pub finalize_add: AddOperation<T>,

    // We don't have an explicity column for initialize phase.
    // Instead, we can use octet_num[0] for that.
    // pub is_initialize: T,
    // pub is_compression: T,
    pub is_external: T,

    // We don't have an explicity column for finalize phase.
    // Instead, we can use octet_num[9] for that.
    // pub is_finalize: T,
    pub is_real: T,
}

impl<T: Default, const ROUNDS: usize> Default for Poseidon2ExternalColsConfigurable<T, ROUNDS> {
    fn default() -> Self {
        Self {
            segment: T::default(),
            clk: T::default(),
            state_ptr: T::default(),
            mem: core::array::from_fn(|_| MemoryReadWriteCols::<T>::default()),
            mem_addr: core::array::from_fn(|_| T::default()),
            finalized_operand: Word::<T>::default(),
            finalize_add: AddOperation::<T>::default(),
            is_external: T::default(),
            is_real: T::default(),
            // Initialize other fields here...
        }
    }
}