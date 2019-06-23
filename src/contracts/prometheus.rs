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
use secp256k1::PublicKey;

use crate::*;

/// **Prometheus contract** $\{ \mathsf{P}_i \} \mid \Gamma$: specially-formed contract encoding
/// pre-defined sets of global parameters. This is *commitment-based agreement*, i.e. agreements
/// that can be referenced by any independent party as a commitment to follow/accept these
/// pre-defined values. Basically, it is a tuple of metaparameters:
/// $\mathsf{P} = \langle w, v, n, a, F, \alpha, \beta,  \gamma, \delta, \varepsilon, \mu, \sigma,
/// t_1, t_2, t_3, t_4 \rangle$ (see struct member definitions below and Table 1 of the Prometheus
/// specification for the meaning of the constants).
pub struct Prometheus {
    /// Stake required to be deposited by Worker, $w$
    pub worker_stake: Satoshi,

    /// Stake required to be deposited by Verifier, $v$
    pub verifier_stake: Satoshi,

    /// Number of required arbiters for the computational contract, $a$
    pub arbiters_required: u8,

    /// Stake required to be deposited by each Arbiter, $a$
    pub arbiter_stake: Satoshi,

    /// Common fund public key which is controlled by external body (can be charity, donations
    /// to development, deterministic proof of burn address or anything else), $F$
    pub common_fund_pk: PublicKey,

    /// Fraction of stake taken from Worker's deposit $w$ in case of its timeout, $\alpha$
    pub worker_timeout_penalty: Satoshi,

    /// Fraction of stake taken from Verifier's deposit $v$ in case of its timeout, $\beta$
    pub verifier_timeout_penalty: Satoshi,

    /// Fraction of Worker's stake $w$ that is awarded to the Client as a compensation payment
    /// in case the Arbitation was not being able to reach a final decision, $\gamma$
    pub worker_to_client_compensation: Satoshi,

    /// Fraction of stake $w$ taken from Worker $W$ if it was found faulty by the Verifier and
    /// didn't appeal, $\delta$
    pub worker_failure_penalty: Satoshi,

    /// Fraction of stakes that all participants ($W, V, \mathcal{A}$) are penalised in case of
    /// failed Arbitration (i.e. when $\mid\mathcal{A^+}\mid\; < \mu \mid\mathcal{A}\mid$) |
    /// $\gamma < \varepsilon \le 1$, $\varepsilon$
    pub general_failure_penalty: Satoshi,

    /// Arbiter voting majority: threshold for Arbiters decision making, $\mu$
    pub arbiter_voting_majority: Satoshi,

    /// Fraction of computation used by Worker in its *computation argument* -- and subsequently
    /// verified by Verifier. It also defines the proportion of Client's payment $c$ that goes to
    /// Worker $W$ in case of successful computing (correspondingly, Verifier $V$ will receive
    /// reward of $1-\sigma c$), $\sigma$
    pub verification_requirement: f32,

    /// Timelock conditions ordered in such a way that smaller-indexed timelock precedes in time
    /// timelocks with larger indexes, $t_1, t_2, t_3, t_4$
    pub timelocks: [u32; 4],
}
