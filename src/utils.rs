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

///! File defining custom types and generic-use functions

use bitcoin_hashes::sha256d;

/// It would be more convenient for this type to be defined in the bitcoin crate, however we need
/// to wait until the merged PR <https://github.com/rust-bitcoin/rust-bitcoin/pull/270> will
/// be released as a part of `bitcoin` crate version 0.19.0. After that we can change this type and
/// re-define it with the `Amount` type provided by the rust-bitcoin library.
pub type Satoshi = u64;
// TODO: Change to Satoshi = bitcoin::util::Amount once bitcoin crate version 0.19.0  will be released

/// It would be more convenient for this type to be defined in the bitcoin crate, however we need
/// to wait until this issue will be solved: <https://github.com/rust-bitcoin/rust-bitcoin/issues/284>
pub type TxID = sha256d::Hash;
// TODO: Remove once <https://github.com/rust-bitcoin/rust-bitcoin/issues/284> is accepted and implemented
