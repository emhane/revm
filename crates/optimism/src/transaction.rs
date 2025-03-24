pub mod abstraction;
pub mod deposit;
pub mod error;

pub use abstraction::{OpTransaction, OpTxTr};
pub use error::OpTransactionError;

use crate::fast_lz::flz_compress_len;

/// <https://github.com/ethereum-optimism/op-geth/blob/647c346e2bef36219cc7b47d76b1cb87e7ca29e4/core/types/rollup_cost.go#L79>
const L1_COST_FASTLZ_COEF: u64 = 836_500;

/// <https://github.com/ethereum-optimism/op-geth/blob/647c346e2bef36219cc7b47d76b1cb87e7ca29e4/core/types/rollup_cost.go#L78>
/// Inverted to be used with `saturating_sub`.
const L1_COST_INTERCEPT: u64 = 42_585_600;

/// <https://github.com/ethereum-optimism/op-geth/blob/647c346e2bef36219cc7b47d76b1cb87e7ca29e4/core/types/rollup_cost.go#82>
const MIN_TX_SIZE_SCALED: u64 = 100 * 1_000_000;

/// Estimates the compressed size of a transaction.
pub fn estimate_tx_compressed_size(input: &[u8]) -> u64 {
    let fastlz_size = flz_compress_len(input) as u64;

    fastlz_size
        .saturating_mul(L1_COST_FASTLZ_COEF)
        .saturating_sub(L1_COST_INTERCEPT)
        .max(MIN_TX_SIZE_SCALED)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tx_compressed_size() {
        const INPUT_EMPTY: &[u8] = b"";
        const INPUT_LARGE: &[u8] = &[1; 100_000];

        let compressed_size_empty = estimate_tx_compressed_size(INPUT_EMPTY);
        assert_eq!(compressed_size_empty, MIN_TX_SIZE_SCALED);

        let compressed_size_large = estimate_tx_compressed_size(&INPUT_LARGE);
        assert!(compressed_size_large > MIN_TX_SIZE_SCALED);
    }
}
