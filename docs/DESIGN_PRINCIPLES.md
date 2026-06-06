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

## 3. Clarity

Clarity is a first-class design constraint, not a documentation afterthought. A
system that autonomously acts in the world, under governance, across federated
domains, must be **legible** — to the humans who hold its rights and approvals, and
to the agents that must interoperate with it.

Clarity means:

- **Explicit over implicit.** Everything is defined and self-describing
  ([Axioms](./AXIOMS.md)); given any entry you can fetch the definition that
  governs it.
- **Rationale is recorded.** Plans state their assumptions and the theory they
  reason under; gate verdicts and arbitrations record *why*. Decisions are
  explainable from data, not reconstructed after the fact.
- **Minimal, narrow interfaces.** The kernel does four things; the protocol has one
  envelope; participation is "author entries." Small surfaces are clearer surfaces.
- **Clarity over cleverness.** Where a legible design and a clever one compete, UAF
  chooses the legible one. Opacity is a defect, not an optimization.

Clarity is what turns *auditable in principle* into *understandable in practice* —
it makes the right to explanation ([Governance](./GOVERNANCE.md)) real.

## 4. Accountability

Autonomy without accountability is unsafe. In UAF every action is **attributable,
answerable, and traceable** — by construction, not by audit policy bolted on later.

- **Attribution.** Every entry is bound to an [Identity](./DESIGN_PRINCIPLES.md)
  and three-party signed ([Data Security](./DATA_SECURITY.md)) — there is no
  anonymous action; nothing unsigned is honored.
- **Answerability.** [Duties](./GOVERNANCE.md) are assigned to roles/identities and
  their discharge is tracked; rights violations are blocked at the gate; unmet
  obligations escalate to the arbiter or [the human](./HUMAN_AT_THE_EDGE.md).
- **Traceability.** The append-only, signed, versioned log
  ([Axioms](./AXIOMS.md)) is an immutable record of *who did what, when, where, and
  why* — non-repudiable across authorship, policy, ordering, and federation
  boundaries.
- **Consequence.** A violation is not silent: it produces a signed `error` entry
  and triggers escalation. Accountability has teeth.

Clarity makes the record **understandable**; accountability makes it
**attributable and answerable**. Together they are why a federated, world-acting
system can be *trusted* to act.

## 5. Sanity check is mandatory

Validation is **required, not optional**. UAF never trusts a plan or an output
because it looks plausible — it checks, every time, before acting.

- **Plan gate.** A session MUST NOT enter deliberation until the reflector has run
  a sanity check and accepted the plan (feasibility, constraint satisfaction,
  coherence with the observation). No accepted sanity check, no execution.
- **Output gate.** A `result` MUST pass a reflector sanity check — verifying it
  against the goal, the constraints, and measurable success criteria
  ([Axioms: measurable](./AXIOMS.md)) — before it is returned.
- **Non-skippable.** The check cannot be bypassed for speed; at worst it is
  *time-boxed and recorded as degraded*, never silently omitted. A skipped or
  failed check is itself a logged, signed event.

This is the reflexive backstop under everything else: clever plans, confident
models, and autonomous operators all still have to pass the sanity check.

## How they combine

Because **everything is data** and that data always carries the **five
primitives**, every decision in the system is (a) made from inspectable inputs and
(b) recorded as an inspectable output. Security, auditability, and trust are
therefore properties of the *data model itself* — see
[`DATA_SECURITY.md`](./DATA_SECURITY.md).
