use crate::{
    cpu::{MemoryReadRecord, MemoryWriteRecord},
    precompiles::{poseidon2::Poseidon2ExternalEvent, PrecompileRuntime},
    runtime::Register,
};

use super::{columns::POSEIDON2_DEFAULT_EXTERNAL_ROUNDS, Poseidon2ExternalChip};

// TODO: I just copied and pasted these from sha as a starting point, so a lot will likely has to
// change.
impl<const N: usize> Poseidon2ExternalChip<N> {
    // TODO: How do I calculate this? I just copied and pasted these from sha as a starting point.
    pub const NUM_CYCLES: u32 = (8 * POSEIDON2_DEFAULT_EXTERNAL_ROUNDS * N) as u32;

    pub fn execute(rt: &mut PrecompileRuntime) -> (u32, Poseidon2ExternalEvent<N>) {
        // Read `w_ptr` from register a0.
        let state_ptr = rt.register_unsafe(Register::X10);

        // Set the clock back to the original value and begin executing the
        // precompile.
        let saved_clk = rt.clk;
        let saved_state_ptr = state_ptr;
        let mut state_read_records =
            [[MemoryReadRecord::default(); N]; POSEIDON2_DEFAULT_EXTERNAL_ROUNDS];
        let mut state_write_records =
            [[MemoryWriteRecord::default(); N]; POSEIDON2_DEFAULT_EXTERNAL_ROUNDS];

        // Execute the "initialize" phase.
        // const H_START_IDX: u32 = 64;
        // let mut hx = [0u32; 8];

        // Read?
        for round in 0..POSEIDON2_DEFAULT_EXTERNAL_ROUNDS {
            let mut input_state = Vec::new();
            for i in 0..N {
                let (record, value) = rt.mr(state_ptr + (i as u32) * 4);
                state_read_records[round][i] = record;
                input_state.push(value);
                // TODO: Remove this debugging statement.
                println!("clk: {} value: {}", rt.clk, value);
                // hx[i] = value;
                rt.clk += 4;
            }

            // TODO: This is where we'll do some operations and calculate the next value.

            for i in 0..N {
                // Adding back 100 + i as specified in the test program.
                let record = rt.mw(state_ptr.wrapping_add((i as u32) * 4), 100 + i as u32);
                state_write_records[round][i] = record;
                rt.clk += 4;
            }
        }

        (
            state_ptr,
            Poseidon2ExternalEvent {
                clk: saved_clk,
                state_ptr: saved_state_ptr,
                state_reads: state_read_records,
                state_writes: state_write_records,
            },
        )
    }
}