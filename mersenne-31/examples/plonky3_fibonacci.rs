use std::fmt::Debug;
use std::marker::PhantomData;

use itertools::Itertools;
use num_traits::{One, Zero};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_challenger::{DuplexChallenger, HashChallenger, SerializingChallenger32};
use p3_circle::CirclePcs;
use p3_commit::ExtensionMmcs;
use p3_field::extension::{BinomialExtensionField, Complex};
use p3_field::{FieldAlgebra, Field};
use p3_fri::FriConfig;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_mersenne_31::{Mersenne31, Poseidon2Mersenne31};
use p3_poseidon2::{Poseidon2};

use p3_symmetric::{
    CompressionFunctionFromHasher, PaddingFreeSponge, SerializingHasher32, TruncatedPermutation,
};
use p3_uni_stark::{prove, verify, StarkConfig};
use rand::thread_rng;
use tracing::{info, span, Level};
use tracing_forest::util::LevelFilter;
use tracing_subscriber::EnvFilter;

fn main() {}
pub struct FibonacciAir {
    pub num_steps: usize,
    pub final_value: u32,
}

impl<F: Field> BaseAir<F> for FibonacciAir {
    fn width(&self) -> usize {
        2 // For current and next Fibonacci number
    }
}

impl<AB: AirBuilder> Air<AB> for FibonacciAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);

        // Enforce starting values
        builder.when_first_row().assert_eq(local[0], AB::Expr::ZERO);
        builder.when_first_row().assert_eq(local[1], AB::Expr::ONE);

        // Enforce state transition constraints
        builder.when_transition().assert_eq(next[0], local[1]);
        builder
            .when_transition()
            .assert_eq(next[1], local[0] * local[0] + local[1] * local[1]);

        // Constrain the final value - no hace falta final value
        let final_value = AB::Expr::from_canonical_u32(self.final_value);
        builder.when_last_row().assert_eq(local[1], final_value);
    }
}

pub fn generate_fibonacci_trace<F: Field>(num_steps: usize) -> RowMajorMatrix<F> {
    let mut values = Vec::with_capacity(num_steps * 2);
    let mut a = F::ZERO;
    let mut b = F::ONE;
    for _ in 0..num_steps {
        values.push(a);
        values.push(b);
        let c = a * a + b * b;
        a = b;
        b = c;
    }
    RowMajorMatrix::new(values, 2)
}

pub struct WideFibonacciAir {
    pub num_steps: usize,
    #[allow(unused)]
    pub number_of_instances: usize,
}

impl<F: Field> BaseAir<F> for WideFibonacciAir {
    fn width(&self) -> usize {
        self.num_steps // the width of the trace is the number of steps.
    }
}

impl<AB: AirBuilder> Air<AB> for WideFibonacciAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        let current_row = main.row_slice(0);

        builder.assert_eq(current_row[0], AB::Expr::ZERO);
        builder.assert_eq(current_row[1], AB::Expr::ONE);

        for i in 2..self.num_steps {
            builder.when_transition().assert_eq(
                current_row[i - 2] * current_row[i - 2] + current_row[i - 1] * current_row[i - 1],
                current_row[i],
            );
        }
    }
}

#[allow(unused)]
pub fn generate_wide_fibonacci_trace<F: Field>(
    number_of_instances: usize,
    num_steps: usize,
) -> RowMajorMatrix<F> {
    // let mut values = vec![vec![F::zero();(num_steps * 2)]; number_of_instances];
    let mut values: Vec<F> = Vec::with_capacity(number_of_instances * num_steps);
    let a = F::ZERO;
    let b = F::ONE;

    for _i in (0..number_of_instances) {
        values.push(a);
        values.push(b);

        for _ in 2..num_steps {
            let a = *values.get(values.len() - 2).unwrap();
            let b = *values.get(values.len() - 1).unwrap();
            values.push(a * a + b * b);
        }
    }
    RowMajorMatrix::new(values, num_steps)
}


fn wide_fibonacci_example_with_poseidon(
    num_steps: usize,
    number_of_instances: usize,
)  -> Result<(), impl Debug> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    type Val = Mersenne31;
    type Challenge = BinomialExtensionField<Val, 3>;

    type Perm16 = Poseidon2Mersenne31<16>;
    let perm16 = Perm16::new_from_rng_128(&mut thread_rng());

    type Perm24 = Poseidon2Mersenne31<24>;
    let perm24 = Perm24::new_from_rng_128(&mut thread_rng());

    type MyHash = PaddingFreeSponge<Perm24, 24, 16, 8>;
    let hash = MyHash::new(perm24.clone());

    type MyCompress = TruncatedPermutation<Perm16, 2, 8, 16>;
    let compress = MyCompress::new(perm16.clone());

    type ValMmcs =
        MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 8>;
    let val_mmcs = ValMmcs::new(hash, compress);

    type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

    type Challenger = DuplexChallenger<Val, Perm24, 24, 16>;

    let air = WideFibonacciAir {
        num_steps,
        number_of_instances,
    };
    let trace = generate_wide_fibonacci_trace::<Val>(number_of_instances, num_steps);


    let fri_config = FriConfig {
        log_blowup: 1,
        num_queries: 100,
        proof_of_work_bits: 16,
        mmcs: challenge_mmcs,
    };
    type Pcs = CirclePcs<Val, ValMmcs, ChallengeMmcs>;
    let pcs = Pcs {
        mmcs: val_mmcs,
        fri_config,
        _phantom: PhantomData,
    };

    type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;
    let config = MyConfig::new(pcs);

    let mut challenger = Challenger::new(perm24.clone());
    let proof = prove(&config, &air, &mut challenger, trace, &vec![]);

    let mut challenger = Challenger::new(perm24.clone());
    verify(&config, &air, &mut challenger, &proof, &vec![])
}

fn wide_fibonacci_example_with_poseidon_QM31(
    num_steps: usize,
    number_of_instances: usize,
)  -> Result<(), impl Debug> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    type Val = Mersenne31;
    type CM31 = Complex<Val>;
    type QM31 = BinomialExtensionField<CM31, 2>;

    type Challenge = QM31;
    // type Challenge = BinomialExtensionField<Val, 3>;


    type Perm16 = Poseidon2Mersenne31<16>;
    let perm16 = Perm16::new_from_rng_128(&mut thread_rng());

    type Perm24 = Poseidon2Mersenne31<24>;
    let perm24 = Perm24::new_from_rng_128(&mut thread_rng());

    type MyHash = PaddingFreeSponge<Perm24, 24, 16, 8>;
    let hash = MyHash::new(perm24.clone());

    type MyCompress = TruncatedPermutation<Perm16, 2, 8, 16>;
    let compress = MyCompress::new(perm16.clone());

    type ValMmcs =
        MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 8>;
    let val_mmcs = ValMmcs::new(hash, compress);

    type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

    type Challenger = DuplexChallenger<Val, Perm24, 24, 16>;

    let air = WideFibonacciAir {
        num_steps,
        number_of_instances,
    };
    let trace = generate_wide_fibonacci_trace::<Val>(number_of_instances, num_steps);


    let fri_config = FriConfig {
        log_blowup: 1,
        num_queries: 100,
        proof_of_work_bits: 16,
        mmcs: challenge_mmcs,
    };
    type Pcs = CirclePcs<Val, ValMmcs, ChallengeMmcs>;
    let pcs = Pcs {
        mmcs: val_mmcs,
        fri_config,
        _phantom: PhantomData,
    };

    type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;
    let config = MyConfig::new(pcs);

    let mut challenger = Challenger::new(perm24.clone());
    let proof = prove(&config, &air, &mut challenger, trace, &vec![]);

    let mut challenger = Challenger::new(perm24.clone());
    verify(&config, &air, &mut challenger, &proof, &vec![])
}


#[cfg(test)]
mod tests {
    //use crate::fibonacci_air::generate_wide_fibonacci_trace;

    use super::*;

    #[test]
    fn test_generate_fibonacci_trace() {
        generate_wide_fibonacci_trace::<Mersenne31>(10, 10);
    }

    #[test]
    fn test_wide_fibonacci_example_with_poseidon() {
        wide_fibonacci_example_with_poseidon(1024, 16).unwrap();
    }

    #[test]
    fn test_wide_fibonacci_example_with_poseidon_QM31() {
        wide_fibonacci_example_with_poseidon_QM31(1024, 16).unwrap();
    }
}
