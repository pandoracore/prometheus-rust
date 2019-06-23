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

/// Routines used for the on-chain commitments for the global consensus state-related data
/// $\Gamma^\Omega$
pub trait Onchain {

}


/// Routines used to encode and operate both global consensus state off-chain data $\Gamma^\Theta$
/// and multiparty state channel-related data $\mathsf{M}(\mathsf{C}_i) \mid \Lambda$
/// which are not stored on-chain and are exchanged in peer-to-peer mannel + stored locally
pub trait Offchain {

}

/// Data present in the global scope $\Gamma$
pub trait GlobalScope {

}

/// Data present in the state channel scope $\Lambda$
pub trait ChannelScope {

}
