# Durable Executions

A UAF session is a **durable execution**: it survives process crashes, kernel
restarts, and arbitrarily long asynchronous waits, and it resumes **exactly where
it left off**. This is durable-workflow semantics — and in UAF it is not a bolt-on
engine, it falls directly out of *everything-as-data*: execution state **is** the
append-only, persisted, signed, replayable workspace log.

## The log is the execution journal

- The workspace is the **durable journal**. Every [command and event](./ACTIONS.md)
  is persisted (and three-party signed) **before** it is honored — progress is the
  log, not in-memory state.
- **Stateless compute, durable journal.** Actors
  ([agent-as-actor](./AGENT_AS_ACTOR.md)) hold no irreplaceable state; everything
  needed to continue is on the log. Recovery is *replay*, not restore-from-RAM.

## Recovery by replay

- A crashed or restarted actor — or the kernel itself — reconstructs its state by
  **replaying the journal** from the last checkpoint. Deterministic replay
  (anchored by [time](./DESIGN_PRINCIPLES.md) against
  [stable-world-model snapshots](./WORLD_MODEL.md)) yields the same state every
  time.
- The **executor resumes the plan from the last completed step** (the `depends_on`
  graph). Completed steps are immutable *events*, so they are never re-run; only
  the unfinished frontier continues.

## Long-running and asynchronous waits

A durable execution can sleep for milliseconds or months:

- **Human-gate approvals** ([Human at the Edge](./HUMAN_AT_THE_EDGE.md)) — the
  execution waits, as data, for an `x-human` `critique`.
- **Deferred / scheduled commands** — steps pinned to a future time (time-as-anchor)
  wake when due.
- **External tool / operand latency** — long generations or operator
  reconciliations.

The execution **waits as data and wakes on the awaited entry** — no process needs
to stay resident holding the wait. This is what makes month-long, human-in-the-loop
workflows durable.

## Exactly-once effects

- Commands carry a unique `id`/`seq` and are **idempotent or compensatable**, so
  **replay never double-applies a side effect** — re-execution dedupes against the
  recorded event.
- Failures **roll forward** via compensating commands (sagas), recorded on the
  journal — never by losing or rewriting it.
- The [mandatory sanity check](./DESIGN_PRINCIPLES.md) is re-evaluated on resume, so
  an execution never continues from an unverified state.

## Durability guarantees

| Guarantee | How |
| --- | --- |
| **Persisted before honored** | entries are durably written and signed before any actor acts on them |
| **Survives actor crash** | restart + replay; isolated, supervised actors |
| **Survives kernel restart** | the journal is the source of truth; the kernel rebuilds from it |
| **Distributed durability** | [federation](./FEDERATION.md) / multi-device replicas of the signed log |
| **Reproducible** | deterministic replay against point-in-time snapshots |

## How it fits

| Durable execution rests on | In |
| --- | --- |
| journal = state | [everything-as-data](./DESIGN_PRINCIPLES.md) |
| restart & supervision | [agent-as-actor](./AGENT_AS_ACTOR.md), [Cloud-Native](./CLOUD_NATIVE.md) |
| idempotent, compensatable steps | [actions-as-commands](./ACTIONS.md) |
| deterministic replay & scheduling | [time as anchor](./DESIGN_PRINCIPLES.md) |
| replicated durability | [Federation](./FEDERATION.md) |

> Status: design direction. Durable execution is a property of the persisted,
> replayable log and the command/event discipline — it adds no wire-format fields,
> only durability and idempotency guarantees on the kernel and executor.
