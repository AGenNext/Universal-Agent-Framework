# UAF Architecture

This document describes how the Universal Agent Framework realizes the
[Universal Agent Protocol](../spec/PROTOCOL.md) as a **kernel-native**,
**multi-platform**, **multi-language** runtime. The protocol is normative; this
document is design rationale.

## 1. The kernel is the runtime core

UAF is organized around a single small **kernel** — the protocol's reference
host. It is not a service you deploy; it is a **library you embed** (think
SQLite, not Postgres). The kernel owns exactly four responsibilities:

1. **Workspace** — the append-only, totally-ordered entry log (the blackboard),
   plus the artifact store that backs `uaf://` handles.
2. **Phase state machine** — enforces the Cognition → Deliberation lifecycle and
   the cognition round limit.
3. **Router + scheduler** — delivers each new entry to the participants whose
   role/subscription matches, and orders concurrent submissions by `seq`.
4. **Tool dispatch** — capability-gated routing of `tool_call`s to tools that
   advertised matching capabilities.

Everything else — perception, planning, reflection, generation — lives in
**participants** outside the kernel. The kernel never contains model logic. This
keeps the trusted core tiny and auditable.

## 2. Kernel-native, multi-platform

The kernel is written in a systems language and compiled to a native artifact
with **no managed runtime dependency**, exposing a stable **C ABI**. From one
codebase it targets:

| Platform | Embedding |
| --- | --- |
| Server / desktop (Linux, macOS, Windows) | Native shared library via C ABI. |
| Edge / embedded | Static link; no GC, bounded memory. |
| Web | Compiled to **WASM**; participants connect over a host-provided channel. |
| Mobile | Native library via the platform FFI. |

Because the workspace is a deterministic, replayable log, the *same* session can
be recorded on one platform and replayed/audited on another.

## 3. Multi-language participation

A participant is "any process that speaks UAP." Two integration modes:

- **In-process** — a binding calls the kernel over its C ABI; the agent runs in
  the host process. Lowest latency; best for the kernel-local executor and the
  MLLM bridge.
- **Out-of-process** — a participant connects over stdio, a local socket, or
  (on web) a message channel, exchanging newline-delimited JSON entries. This is
  how heterogeneous languages and remote tools join.

Both modes carry the *same* entries, so an agent can move between them without
changing its logic. See [`../bindings/README.md`](../bindings/README.md).

## 4. Where the MAGUS roles live

The MAGUS contribution (arXiv:2508.10494) maps onto participants:

| MAGUS element | UAF participant | Mode |
| --- | --- | --- |
| Perceiver / Planner / Reflector | role-conditioned MLLM agents | in- or out-of-process |
| Executor (deliberation driver) | kernel-local participant | in-process |
| Diffusion / TTS / video generators | `tool` participants | out-of-process |
| Shared textual workspace | the kernel's workspace | kernel |

Agents are *role-conditioned variants of one MLLM*: the same model backend,
prompted differently per role. This is what makes the system training-free.

## 5. Canonical Real-World Agents

UAF ships a **catalog of canonical agents** — reference participants that bind
the abstract roles to real-world tasks (document understanding, UI generation,
audio captioning, video summarization, …). They are the "standard library":
copy-able, conformance-tested implementations that demonstrate each role and
each conformance level against realistic inputs.

→ [`CANONICAL_AGENTS.md`](./CANONICAL_AGENTS.md)

## 6. Data flow (one task)

```
client.task ─▶ kernel(workspace) ─▶ perceiver.observation ─▶ planner.plan ─▶ reflector.critique
                                                                                   │
                                  ┌────────────────────────────────────────────────┘
                                  ▼ accept
                          orchestrator.phase_transition ─▶ executor walks plan
                                  │
                                  ├─ understand step ─▶ MLLM tool_result
                                  └─ generate step  ─▶ generator tool_result ─▶ artifact
                                  ▼ all steps done
                          executor.result ─▶ client
```

## 7. Non-goals (at this layer)

- The kernel does not host or run models; it routes to participants that do.
- The protocol does not mandate a model provider; any MLLM/diffusion backend
  that a participant wraps is acceptable.
- The kernel does not define a network/auth layer; transports are pluggable and
  a host supplies trust/policy.
