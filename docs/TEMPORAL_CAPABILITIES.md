# Replay, Reversal, Impersonation, Time Travel

UAF supports four powerful temporal/identity capabilities — **replay, reversal,
impersonation, time travel** — under one inviolable invariant:

> They may operate freely on **data and simulation**, but they **must not alter
> reality**.

The [immutable past](./DESIGN_PRINCIPLES.md) and the real world can be affected
*only* by new, gated, signed commands at the present edge. These capabilities
**read, simulate, and append** — they never rewrite.

## The invariant

The boundary is the **data/simulation plane vs. reality**:

- The past (the event log) is **read-only**; [twins](./DIGITAL_TWIN.md) and
  point-in-time snapshots are **sandboxes**.
- Reality is touched **only** by issuing a fresh [command](./ACTIONS.md) at the
  **gated present** — authorized, [JIT](./DATA_SECURITY.md), signed,
  [sanity-checked](./DESIGN_PRINCIPLES.md), and audited.
- So any of the four is permitted *in the model*; reaching reality always re-enters
  through the present gate. "Does not alter reality" = stay in the data plane, or
  re-authorize at the edge.

## Replay

- **What:** deterministically re-execute a session/log to reconstruct, inspect, or
  debug state ([durable execution](./DURABLE_EXECUTION.md)).
- **Why safe:** it reads the past; effects **dedupe against recorded events**
  (idempotency), so no real side effect is re-emitted; replay runs against snapshots
  / twins. Re-actuating reality during replay is **gated/blocked** unless explicitly
  re-authorized at the present.

## Reversal

- **What:** undo effects via **compensating commands** (sagas).
- **Why safe:** the past is immutable, so reversal **never deletes history** — it
  *appends corrective events*; the original and the compensation both stand. *You
  cannot rewrite history, only add to it.* Real-world reversal is itself a new gated
  command; **irreversible** effects are refused or escalated, never faked.

## Impersonation

- **What:** acting **as** another identity — i.e. authorized, scoped, time-bound
  **delegation** ([identity lifecycle](./IDENTIFIERS.md)).
- **Why safe:** fully attributed — the log records **"X acting as Y"** (both
  principal and actor); three-party signing + [UDCI](./IDENTIFIERS.md) make true
  forgery impossible. It is *assumption of role under authority*, never identity
  theft; it is revocable and expiring, and it does not alter the real record of who
  did what.

## Time travel

- **What:** re-situate computation at a past instant — query/plan/sanity-check
  against a **point-in-time snapshot** ([time as anchor](./DESIGN_PRINCIPLES.md),
  [stable world model](./WORLD_MODEL.md)), or rehearse a counterfactual "what if at
  T" in twins.
- **Why safe:** it operates on snapshots/simulations (reading the past); it cannot
  change the past or inject effects into the real present/future except through the
  present gate. Enables forensic analysis, counterfactual evaluation, reproducible
  audits, and retroactive testing.

## Powerful in the model, humble before reality

| Capability | Gives you | Guardrail |
| --- | --- | --- |
| Replay | debugging, recovery, reproducibility | dedupe on events; reality re-actuation re-gated |
| Reversal | recover from mistakes | append-only compensation; irreversibles escalate |
| Impersonation | delegation, acting-on-behalf | authorized, attributed, expiring; no forgery |
| Time travel | forensics, counterfactuals, audit | snapshots/twins only; reality via the present gate |

The immutable past plus the gated present is exactly what makes these safe: you may
do almost anything to the *model of the world*, but the *world itself* changes only
through an authorized command, now.

> Status: design direction. These capabilities are properties of the read-only
> event log, snapshots, twins, and authorized delegation — none add wire-format
> fields, and all are bounded by the does-not-alter-reality invariant.
