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

## 6. Modular

UAF is **composition over monolith**. The kernel does only four things
([Architecture](./ARCHITECTURE.md)); *everything else is a module*: agents, tools,
gates, bindings, world-model packs, governance/constitution sets, registries.

- **Clear boundaries.** Modules interact only through UAP entries and the
  [Uniform Access Protocol](./ACCESS.md) — never internal coupling. The protocol is
  the seam.
- **Independently versioned & published.** Each module has a [UDCI](./IDENTIFIERS.md)
  and is published, versioned, and signed ([Axioms](./AXIOMS.md)) on its own.
- **Swappable.** Any module that meets the interface and passes
  [conformance](../spec/conformance.md) can replace another — an alternate kernel,
  a different model, a stricter gate policy — without touching the rest.

Small, sharply-bounded modules are what make the system portable, auditable, and
evolvable.

## 7. Feature First

The unit of design, delivery, and reuse is the **feature** — a real-world
capability — not a technical layer. A feature is a **self-contained vertical
slice** that bundles everything it needs:

- the agents and tools that perform it,
- the definitions/ontology terms, world records, and norms (rules/rights) it
  relies on,
- its access policy and fine-grained grants,
- its tests and canonical real-world fixtures.

Consequences:

- **Capability drives design.** We build "document-to-brief" or "story-to-clip"
  ([Canonical Agents](./CANONICAL_AGENTS.md)) — whole features — not orphaned
  layers waiting to be wired up.
- **Independently shippable.** A feature is added or removed as a unit; the
  canonical catalog *is* a set of feature bundles.
- **Modular by construction.** Features are the vertical modules (§6); they compose
  through the protocol, so two features interoperate without knowing each other's
  internals.

Feature-first keeps the framework grounded in what it actually does for the world,
and keeps growth additive: ship a feature, don't refactor a layer.

### Feature lifecycle management

A feature is a managed entity with an explicit, governed lifecycle — its current
state is itself data on the feature's [UDCI](./IDENTIFIERS.md) record, and every
transition is versioned, signed, governed, and measurable:

| Stage | What happens | Bound by |
| --- | --- | --- |
| **Propose** | feature defined; UDCI minted; definition drafted | [defined axiom](./AXIOMS.md) |
| **Develop** | agents/tools/norms built; canonical fixtures + conformance tests | [conformance](../spec/conformance.md) |
| **Publish** | signed, semantically versioned, registered for discovery | publish/version/sign axiom |
| **Deploy / activate** | enabled in a domain, gated, constitution-bound | [Governance](./GOVERNANCE.md), [Federation](./FEDERATION.md) |
| **Evolve** | new versions via governed amendment; stable identifiers, immutable history | [Stable World Model](./WORLD_MODEL.md) |
| **Deprecate** | marked deprecated with a sunset window | Time primitive |
| **Retire / revoke** | withdrawn, keys/grants revoked, optionally ledger-anchored | [Data Security](./DATA_SECURITY.md) |

Lifecycle transitions are governed entries (proposals, approvals) recorded on the
log, so *what changed, when, by whom, and under what authority* is auditable — and
measurable (adoption, cost, conformance, incidents) at every stage. Nothing enters
or leaves service silently.

## 8. Composable

Composition is the primary way to build in UAF. Units at **every** level —
primitives, modules (§6), features/products, solutions — compose through the *same
seam*: UAP entries plus [Uniform Access](./ACCESS.md) over [UDCIs](./IDENTIFIERS.md),
with no bespoke glue.

- **Uniform & recursive.** A composition is itself a unit — a solution is a
  product is a composable thing ([Axioms](./AXIOMS.md)) — so composition nests
  arbitrarily without special cases.
- **Loosely coupled.** Parts interoperate by speaking the protocol, never by
  sharing internals; any conformant part substitutes for another.
- **Invariant-preserving.** Composing preserves the guarantees: a composite is
  defined, measurable, published, managed, governed, signed, and accountable
  *because its parts are* — plus the composite's own contract. Nothing leaks out of
  composition.
- **Emergent capability.** Complex real-world solutions emerge from composing
  simple, conformant, canonical parts — reuse over rebuild.

Composability is what makes modularity, feature-first delivery, and
solution-as-a-service actually pay off: the same small, governed pieces snap
together into arbitrarily large, still-governed wholes.

## How they combine

Because **everything is data** and that data always carries the **five
primitives**, every decision in the system is (a) made from inspectable inputs and
(b) recorded as an inspectable output. Security, auditability, and trust are
therefore properties of the *data model itself* — see
[`DATA_SECURITY.md`](./DATA_SECURITY.md).
