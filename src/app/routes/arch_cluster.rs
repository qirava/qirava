//! `GET /architecture/cluster` — scaling, migration & upgrades.
//!
//! One replication mechanism (add a replica → live-sync → FIFO-hold cutover →
//! promote/repoint → drop) sits behind every operation: vertical and horizontal
//! scale, live migration, standalone⇄cluster switching, and zero-downtime CI/CD
//! rollouts. The storage format is identical across modes, so every switch is
//! non-destructive.

use qexec::FunctionResponse;
use qquill_view::{el, Node};

use crate::app::arch_kit::{self, ascii, callout, p, table};
use crate::app::docs_kit::Toc;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Scaling, migration & upgrades — Qirava architecture";
const DESCRIPTION: &str =
    "One replication mechanism behind every operation: vertical + horizontal scale, live \
     migration, standalone⇄cluster, and zero-downtime CI/CD rollouts — all via a FIFO-hold cutover \
     where reads never pause and writes wait milliseconds.";

const LEAD: &str = "Single-leader replication is the one primitive everything reuses. Add a fresh \
replica, stream it live until caught up, then cut over by briefly holding writes in the FIFO lane \
while the location repoints. Reads never pause; writes wait milliseconds. Because the storage \
format is identical across standalone and cluster, every switch is non-destructive.";

const MECHANISM: &str = "\
   ┌─ add a fresh replica (new node / new version) ─┐
   │  stream WAL + change-stream until CAUGHT UP     │  ← ops keep running on the
   │  (the live node serves the whole time)          │     live node, no break
   └───────────────────────┬─────────────────────────┘
                           ▼
        atomically PROMOTE + REDIRECT (epoch-fence the old)  ← writes pause ~sub-second
                           ▼                                    (degraded, not broken)
                    drain + drop the old node
   Enabler: the storage/WAL format is IDENTICAL in standalone & cluster, so every
   switch is non-destructive — add/remove replicas, never a data conversion.";

const CUTOVER: &str = "\
 SILENT cutover  (same storage model · FIFO write-hold · reads never pause)

 t0  OLD node (v1 / node-A) serving reads + writes normally
      │  spin the NEW node (v2 / node-B), stream WAL ──▶ NEW catches up LIVE
      ▼
 t1  CUTOVER (milliseconds):
      reads ───────────── never pause (data is on both nodes) ─────────────▶
      writes → held in the FIFO write lane ──┐  ← the ONLY effect: mutations
      flush last WAL delta  OLD ──▶ NEW       │    queue for the cutover window,
      epoch-fence OLD  +  REPOINT location ───┤    then apply on NEW in order
        (connection / domain / tunnel → B)    │
      release queued writes ──────────────────▶ NEW applies them (FIFO, none lost)
      ▼
 t2  NEW node serving; OLD drained + dropped.   No restart. No break.";

const ROLLOUT: &str = "\
 Qirava CI → build → SIGN release → transparency log → artifact store
      │  control plane schedules a rollout per TENANT POLICY: auto | approve | pinned
      ▼
 node agent: pull + VERIFY signature  →  for each node, rolling:
   ① spin new-version replica → ② sync (ciphertext, live) → ③ DRAIN old:
        router stops new conns → in-flight finish (grace) → FIFO-hold writes → flush WAL
   → ④ epoch-fence + promote new → ⑤ REPOINT route → ⑥ release held writes
   → ⑦ drop old → ⑧ HEALTH/LAG gate: fail → abort + rollback; pass → next node
   Cluster = zero downtime (peers mask each node); standalone = the same silent cutover.";

fn body() -> Node {
    let mut toc = Toc::new();
    let mut c = el("div");

    // --- the one mechanism ---
    c = c
        .child(toc.h2("One mechanism behind everything"))
        .child(p(
            "Migration, scaling, mode-switching, and software upgrades are not four systems — they \
             are one. Each adds a replica, syncs it live while the current node keeps serving, then \
             cuts over.",
        ))
        .child(ascii("The universal primitive", MECHANISM));

    // --- silent cutover ---
    c = c
        .child(toc.h2("The silent cutover"))
        .child(p(
            "The cutover is silent because the storage model is the same for standalone and \
             cluster, and the write path is already FIFO-ordered. At the cutover, reads keep \
             flowing from data present on both nodes; new writes are held briefly in the FIFO lane, \
             the location repoints, and the held writes flush to the new node in order. Nothing is \
             dropped, nothing restarts.",
        ))
        .child(ascii("Standalone migrate / upgrade — the same silent cutover as cluster", CUTOVER))
        .child(callout(
            "note",
            "Degraded, not broken",
            "“Without breaking operations, degraded” means exactly this: reads never pause; writes \
             wait in the FIFO lane for the cutover window (milliseconds), then apply on the new \
             node. No service restart, no dropped operation, no data loss.",
        ));

    // --- every scenario ---
    c = c
        .child(toc.h2("Every scenario, and what it costs"))
        .child(p(
            "The rule is uniform — every operation is silent, with no restart, standalone included.",
        ))
        .child(table(
            &["Scenario", "How", "Reads", "Writes", "Restart"],
            &[
                &["Vertical (cap up, same node)", "control-channel apply-cap → executor re-reads budget live", "ok", "ok", "none"],
                &["Migrate / bigger server / plan", "new node → sync → FIFO-hold cutover → repoint", "never pause", "FIFO-wait ~ms", "none"],
                &["Horizontal (add cluster node)", "add follower → sync → serves reads + failover", "ok", "ok", "none"],
                &["Standalone → cluster", "add follower(s), keep them (same storage format)", "ok", "ok", "none"],
                &["Cluster → standalone", "drop followers (leader keeps all data)", "ok", "ok", "none"],
                &["Upgrade — standalone", "new-version node → sync → FIFO-hold cutover → repoint", "ok", "FIFO-wait ~ms", "none"],
                &["Upgrade — cluster", "roll node-by-node (same primitive per node)", "ok", "ok", "none"],
            ],
        ))
        .child(callout(
            "note",
            "Why a cluster is 3+ nodes",
            "Three nodes give quorum and automatic failover; two work with manual failover; \
             standalone is one. That is also why the cloud's own control plane runs 3+ nodes — it \
             upgrades itself the same rolling way.",
        ));

    // --- CI/CD ---
    c = c
        .child(toc.h2("Managed upgrades: CI/CD with proper drain"))
        .child(p(
            "Qirava builds and signs each release (transparency-logged); the node agent verifies \
             the signature before running it, then rolls it out per the tenant's update policy — \
             automatic, approval-required, or pinned to a version.",
        ))
        .child(ascii("Signed release → rolling, health-gated rollout", ROLLOUT))
        .child(p(
            "Proper drain means the router repoints FIRST (so no new request lands on the dying \
             node), in-flight requests get a grace window, stragglers FIFO-hold and forward to the \
             new leader, the WAL flushes, and only then does the old process exit. A failed health \
             or replication-lag check aborts and rolls back.",
        ));

    // --- primitives & status ---
    c = c
        .child(toc.h2("The primitives this needs"))
        .child(p(
            "Single-leader replication and the change-stream are built. Three pieces turn them \
             into seamless cutovers and are the focused remaining work:",
        ))
        .child(table(
            &["Primitive", "What it does", "Status"],
            &[
                &["Write-forwarding + FIFO-hold", "queue mutations in the write lane at cutover, forward + flush in order — nothing dropped", "Planned"],
                &["Epoch-fencing promotion", "the old leader sees a higher epoch and steps down — no split-brain", "Planned"],
                &["Location repoint", "the connection / domain / tunnel atomically points at the new node", "Planned"],
                &["Single-leader replication", "committed op-frames stream master → follower over a length-prefixed transport", "Built (partial)"],
            ],
        ))
        .child(callout(
            "note",
            "Where this lives",
            "These primitives belong to the DMS itself (so a self-hosted cluster gets them too); \
             the cloud's control plane orchestrates them across nodes. See the Cloud control plane \
             page for the control channel that drives each cutover.",
        ));

    arch_kit::layout("/architecture/cluster", "Scaling, migration & upgrades", LEAD, c, toc)
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    css.push(arch_kit::arch_css().to_string());
    css.push(crate::app::docs_kit::pager_css().to_string());
    let content = body();
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/architecture/cluster" };
    page(&meta, css, content)
}
