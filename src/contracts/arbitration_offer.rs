// Prometheus Rust Library
// Designed and written in 2019 by
//     Dr. Maxim Orlovsky <dr.orlovsky@gmail.com>
// as an implementation of 0.1.0 Prometheus standard
//     <https://github.com/pandoracore/prometheus-spec/releases/tag/v0.1.0>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::sync::Arc;

use crate::*;

/// Arbitration offer $\{ \mathsf{O}_i \} \mid \Gamma$: specially-formed contracts from parties
/// that want to act as Arbiters in any future computation contract. These contracts are committed
/// to particular Prometheus contract $\mathsf{P}_i$ and must contain proofs of the locked arbiter's
/// stake of size $a$ (defined by $\mathsf{P}_i$): $\mathsf{O} = \langle \mathsf{P}_i, a_p \rangle$.
/// Parties that have made these commitments constitutes global set $\mathbb{A}$.
pub struct ArbitrationOffer {
    /// Prometheus contract that this arbitration offer is compliant with (in particular, with $a$
    /// parameter — the size of the locked arbiter stake).
    pub prometheus_contract: Arc<contracts::Prometheus>,

    /// Size of the locked arbiter stake (is taken from the commitment transaction, must be equal to
    /// parameter $a$ `arbiter_stake` of the Prometheus contract to which this arbitration offer
    /// is committed to.
    pub locked_stake: Satoshi,

    /// On-chain commitment transaction id, locking the specified Arbiter stake of the
    /// `locked_stake` amount.
    pub commitment_tx: TxID,
}
