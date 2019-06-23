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

use std::rc::Weak;
use std::sync::Arc;

use crate::*;

/// Partially-formed arbitration contract $\{ \mathsf{A}_i \} \mid (\Gamma \vee \Lambda)$ before
/// the actual commitment transactions locking the stakes are published on-chain. At this stage
/// this contract is transformed into `contracts::Arbitration` one.
pub struct  ArbitrationFormation {
    /// Computation contract this arbitration will be bound to
    pub computation_contract: Box<contracts::Computation>,

    /// Proposed set of arbitration offers this contract is based on
    pub arbiters: Vec<Arc<contracts::ArbitrationOffer>>,
}

impl Offchain for ArbitrationFormation {

}

impl ChannelScope for ArbitrationFormation {

}

/// Arbitration contract $\{ \mathsf{A}_i \} \mid (\Gamma \vee \Lambda)$:
/// *economically-binding agreements*  commited to some particular contract $\mathsf{C}_i$
/// by Arbiters (denoted as $\mathcal{A}$) that were randomly selected from $\mathbb{A}$
/// to participate in Arbitration stage of the computing contract. The contract contains proofs
/// of the arbiter's stake commitments ($a_p$): $\mathsf{A} = \langle \mathsf{C}_i, \mathcal{A},
/// \{ a_{p_i} \mid \forall A_i \in \mathcal{A} \} \rangle$
pub struct Arbitration {
    /// Computation contract this arbitration is bound to
    pub computation_contract: Weak<contracts::Computation>,

    /// Set of arbitration offers this contract is based on
    pub arbiters: Vec<Arc<contracts::ArbitrationOffer>>,

    pub commitment_tx: TxID,
}

impl Onchain for Arbitration {

}

impl Offchain for Arbitration {

}

impl GlobalScope for Arbitration {

}

impl ChannelScope for Arbitration {

}
