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

/// Multiparty state channel $\mathsf{M}(\mathsf{C}_i) \mid \Lambda$ originating from each
/// computational contract $\mathsf{C_i}$: $\mathsf{M}(\mathsf{C}_i) = \langle \mathsf{C_i},
/// \Upsilon, \pi, \mathtt{Encrypt}(\Pi, h_p)\rangle$
pub struct StateChannel {

}

impl ChannelScope for StateChannel {

}
