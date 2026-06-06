# Kernel — UAF native runtime core

The kernel is the protocol's **reference host**: a small, native, embeddable
runtime that implements the [Universal Agent Protocol](../spec/PROTOCOL.md). It
contains **no model logic** — only coordination.

## Responsibilities

1. **Workspace** — append-only, totally-ordered entry log; assigns `seq`;
   guarantees replayability. Backs the `uaf://` artifact store.
2. **Phase state machine** — enforces Cognition → Deliberation and the cognition
   `max_rounds` limit.
3. **Router + scheduler** — fan-out of each entry to matching participants;
   total order over concurrent submissions.
4. **Tool dispatch** — capability-gated routing of `tool_call`s.

## Surface (planned)

The kernel exposes a stable **C ABI**. Sketch (illustrative, not final):

```
uaf_kernel*  uaf_kernel_open(const uaf_config*);
uaf_session* uaf_session_create(uaf_kernel*, const char* task_json);
int          uaf_attach(uaf_session*, const char* hello_json);   // join as participant
int          uaf_submit(uaf_session*, const char* entry_json);   // append an entry
int          uaf_poll(uaf_session*, uaf_entry* out, int timeout_ms); // receive next entry
void         uaf_session_free(uaf_session*);
void         uaf_kernel_close(uaf_kernel*);
```

The same core compiles to **WASM** for browser hosts, where `uaf_submit` /
`uaf_poll` are bridged to a host message channel.

## Module layout (planned)

```
kernel/
├── workspace/     append-only log + artifact store
├── phases/        cognition/deliberation state machine
├── router/        subscription + scheduling
├── dispatch/      capability-gated tool routing
├── abi/           C ABI surface + WASM bindings
└── conformance/   harness that runs spec/examples/*.jsonl
```

## Implementation language

See [ADR 0001](../docs/adr/0001-kernel-language.md). A native systems language
with a C ABI and WASM target (Rust recommended) — pending confirmation. Any
implementation that passes the conformance harness is acceptable, since the
**protocol** is the contract.

> Status: design scaffold. No kernel source is committed yet; this directory
> defines the shape the implementation will take from the spec.
