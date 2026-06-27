//! `GET /architecture/security` — Security & governance.
//!
//! The trust root (custodian M-of-N over a RAM-only master seed), the governance
//! hierarchy, the two authentication types, the recovery model (cloud account is
//! recoverable; DMS data is not), confidential computing, and the load-bearing
//! principle: policy-flexible, transparency-mandatory.

use qexec::FunctionResponse;
use qquill_view::Node;

use crate::app::arch_kit::{self, ascii, callout, defs, p, table};
use crate::app::docs_kit::Toc;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Security & governance — Qirava architecture";
const DESCRIPTION: &str =
    "Qirava's trust root: custodian M-of-N over a RAM-only master seed; the governance hierarchy; \
     sessions vs HMAC-signed keys; the recovery split (cloud account recoverable, DMS data not); \
     confidential computing; and the policy-flexible, transparency-mandatory principle.";

const LEAD: &str = "Security is one of Qirava's two pillars. There is one trust root — a master \
seed held by M-of-N custodians — and four jobs ride on it: data encryption, software-update \
signing, cluster membership, and attestation. Nothing here is forced: the DMS supports the full \
spectrum and always shows you the residual risk.";

// --- recovery split diagram -------------------------------------------------

const RECOVERY_DIAGRAM: &str = "\
 CLOUD ACCOUNT  (recoverable — it is a business account)
   sign in : passkey (YubiKey FIDO2)  +  seed phrase  +  COMPULSORY email OTP
   lost passkey AND seed?  → KYC / manual verification → cloud RESTORES your
            ACCOUNT + control of your DMS INSTANCES (the box, billing, mgmt).

 DMS  (the data — NOT recoverable, by design)
   sign in : ONLY passkey + seed phrase (M-of-N custodian). NO email, NO backdoor.
   lost seed + ALL backups?  → the data is GONE. The cloud CANNOT recover it
            (seed-encrypted, zero backdoor). Recovery returns the box, not the data.";

// --- custodian share recovery ----------------------------------------------

const SHARE_DIAGRAM: &str = "\
 Each custodian's share is recoverable 3 independent ways — ANY ONE is enough:
   (a) daily YubiKey   — FIDO2-PRF unwraps a stored wrapped copy of the share
   (b) vault YubiKey   — a 2nd PRF-wrapped copy (each key has its own PRF secret)
   (c) BIP39 steel plate — the share value itself, written down, in a vault

 So one YubiKey is BOTH your passkey AND (via PRF) what unlocks your seed share.
 Custodians engage only at boot / governance — never per query.";

fn body() -> Node {
    let mut toc = Toc::new();
    let mut c = qquill_view::el("div");

    // --- trust root ---
    c = c
        .child(toc.h2("One trust root, four jobs"))
        .child(p(
            "The master seed lives in RAM only, never on disk; all database and KMS data are \
             encrypted with seed-derived keys. The seed is split with Shamir's secret sharing \
             across N custodians, and any M of them can reconstruct it (presets 1/1, 2/3, 3/5, … \
             with M < N). That single trust root governs four things:",
        ))
        .child(defs(&[
            ("Data encryption", "every record and KMS secret is sealed under a seed-derived key."),
            ("Software-update signing", "updates are redeploy-only, M-of-N-signed, and transparency-logged."),
            ("Cluster membership", "which nodes may join and hold (encrypted) data."),
            ("Attestation", "a confidential-VM launch measurement gates seed release + node trust."),
        ]))
        .child(callout(
            "warn",
            "Threat model",
            "A TEE protects RAM from the host/root, not from a bug in our own in-guest code. That \
             is why a tiny, zero-dependency, memory-safe TCB matters most — the smaller the \
             surface, the less there is to get wrong.",
        ));

    // --- governance ---
    c = c
        .child(toc.h2("Governance hierarchy"))
        .child(p(
            "Roles are custodian > admin > user > guest, deny-by-default, enforced in the worker \
             funnel. Authority is re-read live on every request, so a revoked grant takes effect \
             immediately.",
        ))
        .child(defs(&[
            ("Custodians", "the root of trust — the only role that promotes admins, creates custodians, defines RBAC roles, and unlocks the master seed (M-of-N)."),
            ("Admins", "made only by custodians; create users/guests and assign per-db access (read | write | read_write | none). Admins cannot make admins or custodians."),
            ("Users / guests", "reach only the databases they are granted. Onboarding is a custodian-gated, single-use invite that lands as guest."),
        ]))
        .child(callout(
            "note",
            "No hardcoded bootstrap",
            "There is no baked-in custodian on install. Onboarding is invite-only and single-use, \
             tracks who invited whom, and the door closes once the founding ceremony is done.",
        ));

    // --- auth ---
    c = c
        .child(toc.h2("Two ways to authenticate"))
        .child(p(
            "Humans get a session; machines get a signed request — there is no third path, and \
             both are checked at L1 (the worker before-auth).",
        ))
        .child(table(
            &["Surface", "Mechanism", "Replay defense"],
            &[
                &[
                    "Human login → session",
                    "username + PBKDF2 password; a random token stored hashed in _sys_sessions; the after-fn extends the TTL and auto-expires on idle.",
                    "token hashed at rest; idle expiry",
                ],
                &[
                    "API key → stateless request",
                    "HMAC signature over canonical method + path + sorted query + body-hash + timestamp + nonce; verified per request; secret stored hashed.",
                    "timestamp-skew + nonce window",
                ],
            ],
        ))
        .child(p(
            "The cloud adds two things on top for account safety: a passkey + seed phrase plus a \
             COMPULSORY email OTP for communications and recovery. The DMS itself stays \
             email-free — it is governed only by the passkey + seed (M-of-N), with no email \
             dependency and no backdoor.",
        ));

    // --- recovery ---
    c = c
        .child(toc.h2("Recovery: the account is recoverable, the data is not"))
        .child(p(
            "This split is deliberate, and it is the honest center of the model. Your cloud \
             account is a business relationship, so it is recoverable. Your DMS data is sealed \
             under your seed, so only you can recover it.",
        ))
        .child(ascii("Cloud account recovery vs DMS data recovery", RECOVERY_DIAGRAM))
        .child(callout(
            "warn",
            "The trade you are making",
            "A compromised control plane can disrupt availability, but it can never read your \
             data — the seed never leaves your custody. The flip side: lose your seed and every \
             backup, and the data is unrecoverable. Keeping the seed safe is the tenant's \
             responsibility.",
        ))
        .child(ascii("Each custodian share: three independent recoveries, any one enough", SHARE_DIAGRAM))
        .child(p(
            "Re-ceremony (changing M, N, or the custodian set) is authorized by the previous M, \
             runs in the TEE, is transparency-logged, retires the old shares, and preserves the \
             seed — unless M custodians collude, there is no way to move it.",
        ));

    // --- confidential compute ---
    c = c
        .child(toc.h2("Confidential computing"))
        .child(p(
            "On supported silicon the DMS runs inside a confidential VM so the host root cannot \
             read guest RAM, and it attests its launch measurement before it is handed the seed. \
             Hardware tier is one of the independent security axes — the DMS detects it and states \
             the residual risk at boot.",
        ))
        .child(table(
            &["Tier", "What it protects", "Residual risk"],
            &[
                &["Plain", "nothing at the hardware layer", "host root can read RAM"],
                &["TSME", "physical-RAM protection (memory encryption)", "no per-VM isolation / no attestation"],
                &["SEV-SNP / TDX", "per-VM isolation + attestation; host root cannot read RAM", "trust in the CPU vendor's root"],
            ],
        ))
        .child(callout(
            "note",
            "Status",
            "The auth, governance, sessions/HMAC, and invite model are BUILT. The M-of-N custodian \
             seed ceremony, at-rest KMS encryption, and SEV-SNP/TDX attestation are designed and \
             on the roadmap.",
        ));

    // --- principle ---
    c = c
        .child(toc.h2("Policy-flexible, transparency-mandatory"))
        .child(p(
            "The DMS never forces a security configuration. It supports the full spectrum on each \
             independent axis — hardware tier, key custody, authenticator, backup, topology — and \
             for every choice it computes and SHOWS the residual risk, so you give informed \
             consent. The only thing forced is honesty.",
        ))
        .child(p(
            "That surfaces four ways: a setup chooser (option + risk), a startup banner (the \
             auto-detected tier, which cannot lie), an on-demand security-posture report (a \
             generated threat model: who you trust, your single points of failure, your recovery \
             story), and a pre-ceremony confirmation. “Secure by informed choice, not false \
             promise” is the differentiator — most databases hide the trade-off or make \
             custody mean trust-one-party.",
        ));

    arch_kit::layout(
        "/architecture/security",
        "Security & governance",
        LEAD,
        c,
        toc,
    )
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    css.push(arch_kit::arch_css().to_string());
    css.push(crate::app::docs_kit::pager_css().to_string());
    let content = body();
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/architecture/security" };
    page(&meta, css, content)
}
