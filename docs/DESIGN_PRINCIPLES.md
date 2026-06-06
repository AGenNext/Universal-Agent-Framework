# Design Principles

The Universal Agent Framework rests on two foundational principles that shape the
protocol, the kernel, and every participant.

## 1. Everything as Data

There are no out-of-band control channels. **Every concept in UAF is data on the
workspace** — an entry:

| Concept | Is the entry |
| --- | --- |
| A request | `task` |
| A perception, plan, critique | `observation`, `plan`, `critique` |
| Control flow (advancing a phase) | `phase_transition` |
| A capability | a `hello` carrying a tool descriptor |
| A policy decision / gate verdict | a `critique` / `error` from an `x-gate` |
| A human approval | a `critique` from an `x-human` |
| Trust, identity, time, location | the `ctx` block on every entry |

Consequences:

- **Inspectable** — the whole system state is one readable log.
- **Replayable** — re-running the log reconstructs the session deterministically.
- **Auditable** — *who did what, when, where, and why* is in the data, not in
  ephemeral RPC.
- **Portable** — moving a session between platforms is moving its log.

If something influences behavior, it MUST be expressible as an entry. Hidden
state is a protocol violation, not an optimization.

## 2. Five primitives: Trust, Time, Location, Identity, Relation

These are not metadata bolted onto messages; they are **first-class primitives**
carried in every entry's [`ctx` block](../spec/PROTOCOL.md#10-core-primitives--the-context-block):

| Primitive | Question | Examples of what it drives |
| --- | --- | --- |
| **Identity** | *Who* authored this? | signing, attribution, ingress auth |
| **Time** | *When* is this valid? | ordering, causality, credential/approval expiry |
| **Location** | *Where* is it from / allowed? | edge↔cloud routing, data residency |
| **Trust** | *How much* do we believe it? | gate thresholds, human escalation |
| **Relation** | *How* does it relate? | causality, delegation chains, org structure |

Every policy in UAF — gating, routing, human escalation, residency — is
expressed as a predicate over these five primitives. That is what lets *Agent at
the Gates* and *Human at the Edge* reason uniformly: they read the same five
dimensions on the same entries.

## How the two combine

Because **everything is data** and that data always carries the **five
primitives**, every decision in the system is (a) made from inspectable inputs and
(b) recorded as an inspectable output. Security, auditability, and trust are
therefore properties of the *data model itself* — see
[`DATA_SECURITY.md`](./DATA_SECURITY.md).
