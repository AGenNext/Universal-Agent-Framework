# Bindings — multi-language SDKs

A UAF participant is *any process that speaks UAP*. Bindings make that feel
native in each language. They are thin: serialize entries, talk to the kernel,
and offer ergonomic helpers for role-conditioned agents and tools.

## Two integration modes

| Mode | Transport | Best for |
| --- | --- | --- |
| **In-process** | kernel **C ABI** (FFI) | the executor, the MLLM bridge, latency-sensitive agents |
| **Out-of-process** | newline-delimited JSON over **stdio / socket / channel** | heterogeneous languages, remote tools, sandboxed agents |

Both carry the identical [entry envelope](../spec/PROTOCOL.md#3-the-entry-envelope),
so an agent can switch modes without changing its logic.

## What every binding provides

1. A typed **entry** model + (de)serialization validated against
   [`spec/schemas`](../spec/schemas).
2. A `connect()` that attaches over either transport.
3. Helpers to author the common kinds (`hello`, `observation`, `plan`,
   `critique`, `tool_call`, `tool_result`, `result`).
4. A small **agent loop**: subscribe → match by role/kind → handle → submit.

## Planned languages

| Language | Mode(s) | Notes |
| --- | --- | --- |
| Rust | in + out | reference; same crate family as the kernel |
| TypeScript | out (+ WASM in) | Node and browser (WASM kernel) |
| Python | out | wraps model/diffusion SDKs |
| Go | out | services and CLIs |
| Swift / Kotlin | in (FFI) | mobile hosts |

## Framework adapters (multi-framework)

UAF is a meta-framework: existing agents and tools keep their stack and join by
speaking UAP through a thin **adapter**. An adapter maps a foreign framework's
calls onto entry authoring/handling — nothing else changes.

| Foreign framework | Adapter maps |
| --- | --- |
| **MCP** server | MCP tools → `tool` participant + capability `hello`; `tools/call` → `tool_call`/`tool_result` |
| **A2A** agent | A2A messages → entries; A2A task lifecycle → session phases |
| **LangChain / LangGraph** | a chain/graph node → a role-conditioned participant |
| **AutoGen / CrewAI** | a framework agent → a participant; its tools → `tool` participants |

The rule is always the same: **be a participant, author entries.** This keeps the
core protocol-pure while absorbing the wider ecosystem.

## Minimal out-of-process agent (illustrative pseudocode)

```
conn = uap.connect(stdio)
conn.send(hello{ role: "reflector", id: "reflector-1" })

for entry in conn.entries():
    if entry.kind == "plan":
        verdict = review(entry.body)          # your model call
        conn.send(critique{
            in_reply_to: [entry.id],
            verdict: verdict.ok ? "accept" : "revise",
            notes: verdict.notes,
        })
```

> Status: design scaffold. No binding source is committed yet; this directory
> defines the contract each language SDK implements against the spec.
