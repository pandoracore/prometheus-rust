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

use bitcoin::Transaction;

use crate::*;

pub struct Computational {
    pub prometheus_contract: contracts::Prometheus,
    pub commitment_transaction: Transaction,
    pub arbitration_transaction: Transaction,
    pub settlement_transactions: [Transaction; 8],
}
