# Universal Agent Framework (UAF)

> MVP: a small, protocol-first runtime for agents that coordinate through an
> append-only workspace.

UAF is being reduced into a practical MVP. The first product is not a broad
platform, marketplace, federation layer, or full governance system. The first
product is a working reference loop:

```text
entry -> validate -> append -> order -> transition -> replay
```

The MVP proves that independent participants can coordinate by writing protocol
entries to a shared workspace. The kernel stays small. Constraints, approvals,
features, policies, tool calls, and results are represented as data in the log.

## MVP scope

The MVP has one job:

> Accept a task, create an ordered workspace log, move through cognition and
> deliberation, apply thin constraints, produce a result, and replay the session.

## MVP layers

### 1. Universal Agent Protocol

A small JSON protocol for workspace entries, roles, phases, tool calls,
constraints, approvals, and results.

- [`spec/PROTOCOL.md`](./spec/PROTOCOL.md)
- [`spec/CONSTRAINTS_AND_FEATURES.md`](./spec/CONSTRAINTS_AND_FEATURES.md)

### 2. Reference kernel

A minimal host that validates entries, assigns sequence numbers, enforces the
phase state machine, and replays the workspace log.

MVP kernel responsibilities:

- validate entry envelope
- assign `seq`
- append to JSONL workspace
- enforce `cognition -> deliberation -> result`
- require accepted plan before execution
- require policy/approval entries for blocking constraints
- replay a session from JSONL

### 3. CLI demo

A tiny CLI is enough for MVP:

```text
uaf validate examples/sessions/basic.jsonl
uaf replay examples/sessions/basic.jsonl
uaf run examples/tasks/basic-task.json
```

No UI is required for the MVP.

## What is deliberately out of scope

The MVP does not include:

- marketplace
- registry
- federation
- blockchain
- payment
- full identity platform
- web console
- multi-cloud orchestration
- production Kubernetes operator
- complex model routing
- many language bindings

These can be expressed later as protocol entries, constraints, features, adapters,
or conformance profiles. They should not expand the core runtime.

## Constraint-led growth

UAF grows by adding protocol vocabulary, schemas, constraints, policy rules, and
examples — not by expanding the kernel.

```text
Feature = schema + entry kind + policy rule + conformance example
Not     = new service + new framework + new runtime
```

## MVP repository layout

```text
spec/        Protocol, schemas, constraints, and conformance rules
kernel/      Minimal reference runtime scaffold
examples/    Replayable JSONL sessions and task inputs
bindings/    Future SDK bindings; not required for MVP
```

## MVP success criteria

UAF MVP is ready when this works from a clean checkout:

```text
uaf run examples/tasks/basic-task.json
uaf replay .uaf/sessions/<session>.jsonl
```

And the replay shows:

1. task received
2. observation appended
3. plan appended
4. critique accepted
5. phase transition recorded
6. constrained tool call approved or allowed
7. result emitted
8. final state reconstructed from the log

## Design principle

The kernel is not the intelligence. The kernel is the accountable substrate.

Agents, tools, humans, and policy engines interpret entries. The kernel orders,
validates, gates, and replays them.