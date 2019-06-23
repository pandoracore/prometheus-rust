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

use crate::*;
use std::sync::Arc;
use bitcoin::util::psbt::PartiallySignedTransaction;

/// Partially-formed computational contract $\{ \mathsf{C}_i \} \mid (\Lambda)$ before
/// the actual commitment transactions locking the stakes are published on-chain. At this stage
/// this contract is transformed into `contracts::Computation` one.
pub struct ComputationFormation {
    pub prometheus_contract: Arc<contracts::Prometheus>,
    pub commitment_tx: PartiallySignedTransaction,
    pub arbitration_tx: PartiallySignedTransaction,
    pub settlement_txs: [PartiallySignedTransaction; 8],
}

impl Offchain for ComputationFormation {

}

/// Computational contract $\{ \mathsf{C}_i \} \mid (\Gamma \vee \Lambda)$:
/// *economically-binding agreements*  committed to some particular contract $\mathsf{P}_i$ between
/// parties $C, W, V$ containing proofs of their locked stakes $w_p,v_p$ (corresponding to
/// the amounts defined by $\mathsf{P}_i$), computational reward $c$ and computation-related
/// $\mathtt{payload}$: $\mathsf{C} = \langle \mathsf{P}_i, C, W, V, c, w_p, v_p, \mathtt{payload}
/// \rangle$. Computational contract locks deposits and includes all possible schemes
/// for payouts (see Table 2 in the original Prometheus specification).
pub struct Computation {
    /// Prometheus contract defining the values of the main parameters for the computation contract
    pub prometheus_contract: Arc<contracts::Prometheus>,

    /// ID of the commitment transaction this contract is bound to
    pub commitment_txid: TxID,

    /// Pre-agreed and on-chain committed arbitration contract used by this computation contract
    pub arbitration_contract: Box<contracts::Arbitration>,

    /// Set of the partically-signed settlement transaction for each of the possible settlement
    /// cases
    pub settlement_txs: [PartiallySignedTransaction; 8],
}

impl Offchain for Computation {

}

impl GlobalScope for Computation {

}
