// Prometheus Rust Library
// Designed and written in 2019 by
//     Dr. Maxim Orlovsky <dr.orlovsky@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use bitcoin::Transaction;
use secp256k1::PublicKey;

pub struct PrometheusContract {
    pub worker_stake: u64,
    pub verifier_stake: u64,
    pub arbiters_required: u8,
    pub arbiter_stake: u64,
    pub common_fund_pk: PublicKey,
    pub worker_timeout_penalty: u64,
    pub verifier_timeout_penalty: u64,
    pub worker_to_client_compensation: u64,
    pub worker_failure_penalty: u64,
    pub general_failure_penalty: u64,
    pub arbiter_voting_majority: u64,
    pub verification_requirement: f32,
}

pub struct ComputationalContract {
    pub prometheus_contract: PrometheusContract,
    pub commitment_transaction: Transaction,
    pub arbitration_transaction: Transaction,
    pub settlement_transactions: [Transaction; 8],
}