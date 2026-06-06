# ADR 0001 — Kernel-native runtime in a systems language with a C ABI

- **Status:** Proposed
- **Date:** 2026-06-06
- **Context:** Universal Agent Framework, initialization

## Context

UAF is protocol-first and must run **multi-platform** (server, desktop, edge,
web) and host **multi-language** agents. The runtime core ("kernel") owns the
workspace, phase state machine, routing, and tool dispatch. We must choose how
the kernel is implemented and exposed.

## Decision

Implement the kernel as a small **native, compiled runtime with no managed
runtime dependency**, exposing a stable **C ABI**, and compilable to **WASM**
for the web. Agents and tools integrate either in-process (over the C ABI) or
out-of-process (newline-delimited JSON entries over stdio/socket/channel).

A systems language with strong memory safety and first-class WASM and FFI
support (e.g. **Rust**) is the recommended implementation language for the
kernel. This is a recommendation of the *kernel* language only; participants
remain language-free by design.

## Rationale

- A C ABI is the lowest common denominator every language can bind to, which is
  what makes "multi-language" real without N reimplementations.
- "Embed, don't deploy" (SQLite model) fits a *runtime core* better than a
  service: lower latency, simpler ops, deterministic local scheduling.
- WASM compilation is what lets the *same* kernel run "multi-platform" down to
  the browser.
- Keeping all model logic out of the kernel keeps the trusted, ported core tiny.

## Consequences

- We maintain one native artifact plus thin per-language bindings, not a kernel
  per language.
- The protocol — not the kernel's source language — is the compatibility
  contract; an alternate kernel in another language is permitted if it passes
  conformance.
- Out-of-process participants pay JSON serialization cost; acceptable given the
  coordination granularity (entries, not tight loops).

## Open questions

- Final kernel language is pending confirmation; Rust is the leading candidate.
- Default transport for out-of-process participants (stdio vs. local socket) to
  be fixed in a follow-up ADR.
