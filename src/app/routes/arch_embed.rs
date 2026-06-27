//! `GET /architecture/embed` — embedded & sync.
//!
//! The same engine as an in-process library (Tauri / mobile), and bidirectional
//! WebSocket sync — the same op-frame as cluster replication — for offline-first,
//! backup, and restore, authenticated by API key.

use qexec::FunctionResponse;
use qquill_view::{el, Node};

use crate::app::arch_kit::{self, ascii, callout, defs, p};
use crate::app::docs_kit::Toc;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Embedded & sync — Qirava architecture";
const DESCRIPTION: &str =
    "The Qirava engine as an in-process library for Tauri and mobile, with bidirectional WebSocket \
     sync — the same op-frame as cluster replication — for offline-first use, backup, and restore \
     by API key.";

const LEAD: &str = "The same engine that runs a server runs in-process inside a desktop or mobile \
app — no socket, RBAC intact. And because an embedded instance speaks the same op-frame as a \
cluster, it syncs both ways: offline-first locally, continuously backed up to the cloud, and \
restorable from it with an API key.";

const SYNC: &str = "\
 embedded DMS (in-process, offline-first) ──WebSocket, API-KEY auth──▶ cloud / cluster DMS
   DUAL = bidirectional op-frame sync (origin-id tags prevent echo loops):
     push local ops up   → continuous BACKUP into the cloud
     pull remote ops down → offline-first, multi-device
   RESTORE: a fresh embed authenticates with the API key → replays the full
            stream down → fully rehydrated. An embed is just a follower/forwarder over WS.";

fn body() -> Node {
    let mut toc = Toc::new();
    let mut c = el("div");

    // --- engine as a library ---
    c = c
        .child(toc.h2("The engine as a library"))
        .child(p(
            "On desktop (Tauri) and mobile, the engine links directly into the app and runs \
             execute() in-process. The untrusted webview dispatches to an in-process worker — no \
             socket — which fits Tauri's function-calls-only model and keeps the full RBAC \
             pipeline intact. The at-rest key is scoped to the app id through the OS keystore.",
        ))
        .child(defs(&[
            ("In-process execute", "the same one execute() + function registry, no network layer."),
            ("RBAC intact", "L1/L2/L3 still apply; the webview is just another untrusted caller."),
            ("OS-scoped key", "at-rest encryption keyed per app id via the platform keystore."),
        ]));

    // --- dual sync ---
    c = c
        .child(toc.h2("Dual WebSocket sync"))
        .child(p(
            "Sync is bidirectional over a WebSocket, authenticated by an API key. The embed pushes \
             its local operations up and pulls remote operations down, with origin-id tags to \
             prevent echo loops. It is offline-first: the app works fully offline and reconciles \
             when it reconnects.",
        ))
        .child(ascii("Embedded ⇄ cloud: dual op-frame sync over WebSocket", SYNC));

    // --- backup & restore ---
    c = c
        .child(toc.h2("Backup & restore"))
        .child(p(
            "The push direction is a continuous backup — your local data streams into the cloud as \
             it changes. Restore is the pull direction from empty: a fresh embed authenticates with \
             the API key and replays the full stream down until it is fully rehydrated. Same \
             op-frame, same path — backup and restore are just the two directions of the one sync.",
        ))
        .child(callout(
            "note",
            "Status",
            "The in-process embed model and the shared op-frame are designed on top of the built \
             change-stream; the bidirectional (dual) WebSocket sync, origin-id loop prevention, and \
             API-key-authenticated restore are on the roadmap alongside write-forwarding.",
        ));

    arch_kit::layout("/architecture/embed", "Embedded & sync", LEAD, c, toc)
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    css.push(arch_kit::arch_css().to_string());
    css.push(crate::app::docs_kit::pager_css().to_string());
    let content = body();
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/architecture/embed" };
    page(&meta, css, content)
}
