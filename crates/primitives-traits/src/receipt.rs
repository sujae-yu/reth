//! Receipt abstraction

use crate::{InMemorySize, MaybeCompact, MaybeSerde, MaybeSerdeBincodeCompat};
use alloc::vec::Vec;
use alloy_consensus::{
    Eip2718EncodableReceipt, RlpDecodableReceipt, RlpEncodableReceipt, TxReceipt, Typed2718,
};
use alloy_rlp::{Decodable, Encodable};
use core::fmt;

/// Helper trait that unifies all behaviour required by receipt to support full node operations.
pub trait FullReceipt: Receipt + MaybeCompact {}

impl<T> FullReceipt for T where T: Receipt + MaybeCompact {}

/// Abstraction of a receipt.
pub trait Receipt:
    Send
    + Sync
    + Unpin
    + Clone
    + fmt::Debug
    + TxReceipt<Log = alloy_primitives::Log>
    + RlpEncodableReceipt
    + RlpDecodableReceipt
    + Encodable
    + Decodable
    + Eip2718EncodableReceipt
    + Typed2718
    + MaybeSerde
    + InMemorySize
    + MaybeSerdeBincodeCompat
{
}

// Blanket implementation for any type that satisfies all the supertrait bounds
impl<T> Receipt for T where
    T: Send
        + Sync
        + Unpin
        + Clone
        + fmt::Debug
        + TxReceipt<Log = alloy_primitives::Log>
        + RlpEncodableReceipt
        + RlpDecodableReceipt
        + Encodable
        + Decodable
        + Eip2718EncodableReceipt
        + Typed2718
        + MaybeSerde
        + InMemorySize
        + MaybeSerdeBincodeCompat
{
}

/// Retrieves gas spent by transactions as a vector of tuples (transaction index, gas used).
pub fn gas_spent_by_transactions<I, T>(receipts: I) -> Vec<(u64, u64)>
where
    I: IntoIterator<Item = T>,
    T: TxReceipt,
{
    receipts
        .into_iter()
        .enumerate()
        .map(|(id, receipt)| (id as u64, receipt.cumulative_gas_used()))
        .collect()
}
