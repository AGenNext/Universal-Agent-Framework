# Foundational Axioms

Beneath every layer of UAF — protocol, kernel, primitives, governance, world
model, federation — sit a few **axioms** that hold for *everything* in the
system. They are what make the framework inspectable, interoperable, governable,
and accountable. If something cannot satisfy these axioms, it does not belong on
the workspace.

## 1. Everything is **data**

Every concept — tasks, plans, policies, capabilities, trust, control flow, norms,
the world model — is an **entry** or record on the shared log. There are no
out-of-band channels and no hidden state. Influence requires representation.

→ [Design Principles](./DESIGN_PRINCIPLES.md)

## 2. Everything is **defined**

Nothing is implicit, untyped, or ad-hoc. Every entry kind, role, tool capability,
primitive value, world record, rule, and right has an **explicit, machine-readable
definition** — a schema in the shared registry and a term in the shared
[ontology](./WORLD_MODEL.md). Consequences:

- **No undefined behavior.** An entry that does not validate against a published
  definition is rejected at the [gate](./AGENT_AT_THE_GATES.md).
- **Shared meaning.** Cross-agent and cross-domain interoperability is possible
  only because both sides resolve the same definition.
- **Self-describing.** Given an entry, you can fetch the definition that governs
  it; the system explains itself.

## 3. Everything is **measurable**

Every entity and action exposes **metrics**, so the system is observable
end-to-end and decisions are evidence-based:

| Measured | Examples |
| --- | --- |
| Cost / resources | compute, energy, money, data, attention (the [resource budgets](./WORLD_MODEL.md)) |
| Time | latency, queueing, validity windows, deadlines |
| Trust / quality | trust scores, conformance, verification pass/fail |
| Outcome | result vs. goal, constraint satisfaction, duty discharge |

- **Plans carry success metrics**; the reflector verifies against *measured*
  outcomes, not assertions.
- **SLOs/KPIs are predicates over measurements**, gated and audited like any other
  policy.
- Measurements are themselves **data** (axiom 1) — recorded, signed, replayable.

## 4. Everything is **published, version-controlled, and signed**

Every definition, norm (constitution/rules), capability, agent, tool, artifact —
and the protocol itself — is:

- **Published** — registered in a discoverable registry so the ecosystem can find,
  reuse, and federate it ([Federation](./FEDERATION.md),
  [framework adapters](../bindings/README.md)).
- **Version-controlled** — immutable and content-addressed; a change yields a new
  version with preserved history and a semantic version. The append-only workspace
  log is the runtime version history; definitions and norms carry explicit
  versions (e.g. constitution amendments).
- **Signed** — cryptographically signed with verifiable provenance, using the
  three-party (and federation boundary) signing model
  ([Data Security](./DATA_SECURITY.md)). Nothing anonymous or unversioned is
  honored.

### Pluggable notary/registry — blockchain as a tool

The publication, versioning, and signature-anchoring backend is **pluggable**, and
a **blockchain / distributed ledger is offered as a (optional) tool** for it —
not a mandatory dependency. As a `tool` participant
([canonical `uaf.tool.ledger`](./CANONICAL_AGENTS.md)) it can provide:

- **tamper-evident publication & versioning** of definitions, norms, and the
  constitution (immutable, content-addressed history);
- **signature/notary anchoring** of the three-party-signed log for independent
  verifiability;
- **federation trust anchoring** — a shared root for cross-domain identity and
  trust ([Federation](./FEDERATION.md));
- **resource/economic settlement** — accounting and clearing for the budgets and
  markets in the [World Model](./WORLD_MODEL.md).

Because it is a tool, a deployment MAY use a ledger, a signed transparency log, or
a plain content-addressed store interchangeably — preserving cloud-agnosticism and
avoiding lock-in. The axiom is *published/versioned/signed*; the ledger is one way
to satisfy it.

## 5. Everything is **managed**

Every entity is a **managed entity** with an explicit, governed lifecycle — it is
created, published, versioned, deployed, deprecated, and retired; never appearing
or disappearing silently. The lifecycle **state is data** on the entity's record,
every transition is **signed and governed**, and each stage is **measurable**.

This generalizes the per-kind lifecycles already defined:

| Managed entity | Lifecycle |
| --- | --- |
| Features | [Feature lifecycle management](./DESIGN_PRINCIPLES.md) |
| Identities | [Identity lifecycle management](./IDENTIFIERS.md) |
| Definitions, norms, tools, models, agents, world records | same pattern: propose → publish → evolve → deprecate → retire |

Because management is itself data, you can always answer *what state is this in,
how did it get here, who authorized each step, and what did it cost* — for
anything in the system. "Managed" is what turns *defined, measurable, published*
into a controlled lifecycle rather than a static snapshot.

## 6. Everything is **governed**

No entity, action, transition, or access sits outside the normative layer. Every
entry passes a [gate](./AGENT_AT_THE_GATES.md); every action is bound by the
applicable [constitution and rules](./GOVERNANCE.md); every actor holds duties and
rights; every decision is accountable and sanity-checked. **There is no
privileged, ungoverned path** — the kernel, gates, and constitution apply to all
participants equally, including operators and humans (within their rights).

Governance is universal and non-optional. Combined with *everything is managed*,
it means every lifecycle transition is not just recorded but **authorized**: a
feature ships, a key rotates, a norm amends, a tool is invoked — each only under
the authority that governs it.

## 7. Everything is **a product**

Every entity is treated as a **product**, not an artifact that merely exists. A
product in UAF has:

- an **owner** accountable for it ([accountability](./DESIGN_PRINCIPLES.md));
- named **consumers** and a documented **interface contract** (its definition);
- **quality / SLOs** that are measured and enforced;
- a **roadmap and lifecycle** (managed), and a clear **value proposition**.

Product thinking turns the other axioms into a discipline:

| Product practice | Rests on |
| --- | --- |
| Discoverable in a catalog | published / versioned / signed |
| Self-service consumable | [Uniform Access Protocol](./ACCESS.md) |
| Compatibility promises across versions | defined + managed |
| Consumer-driven quality (SLOs) | measurable |
| Clear ownership & support | governed |

The [canonical catalog](./CANONICAL_AGENTS.md) is therefore a **product catalog**;
features, agents, tools, models, definitions, datasets, identities — and the
protocol itself — are each products with owners and consumers. This favors
**reuse over rebuild** and makes value, not just function, explicit.

### Solution as a Service

Products **compose into solutions**, and solutions are delivered **as a service**.
This is the composition ladder:

```
   primitives & modules  →  features (= products)  →  solutions  →  delivered as a service
```

- A **solution** composes multiple products/features into an end-to-end answer to
  a real-world problem (e.g. an entire claims-processing or research workflow), not
  a single capability.
- Delivered **as a service** means: **self-service** to consume
  ([Uniform Access](./ACCESS.md)), **metered** and billable (the
  [resource/economic](./WORLD_MODEL.md) accounting), **multi-tenant** across
  [federated domains](./FEDERATION.md), **SLO-backed** (measurable), **governed**
  (owned, rule-bound), and **lifecycle-managed** (versioned, supported, retired).
- Because solutions are themselves products (with owners, contracts, SLOs, and a
  UDCI), the whole stack is uniform: a service is a governed, managed product that
  happens to compose others. Reuse and recursion all the way up.

## 8. Everything is **time-bound**

Nothing is permanent by default. Every entity, grant, and assertion carries
**temporal validity** (`valid_from` / `valid_until`) via the Time primitive and
**expires**:

| Time-bound thing | Expires / is bounded by |
| --- | --- |
| Credentials & authorizations | [JIT auth](./DATA_SECURITY.md) — minted on demand, tight TTL |
| Trust assertions | `ctx.trust` with a validity window; stale trust is re-earned |
| Knowledge & facts | beliefs carry validity; stale facts are no longer relied upon |
| Plans & sessions | deadlines and time budgets |
| Delegations | JIT and expiring ([identity lifecycle](./IDENTIFIERS.md)) |
| Norms | effective dates and sunset windows ([governance](./GOVERNANCE.md)) |
| Features & identities | lifecycle stages with deprecation/retirement |

Consequences:

- **Bounded by default.** Absent an explicit window, a conservative default TTL
  applies; *“forever” must be stated explicitly and is itself governed.*
- **Expired = untrusted.** A [gate](./AGENT_AT_THE_GATES.md) rejects any entry,
  grant, or fact outside its validity window — time-based revocation is implicit.
- **Reproducible.** Point-in-time validity is what makes
  [stable-world-model](./WORLD_MODEL.md) snapshots and replay deterministic.
- **Least privilege in time.** Authority that is never permanent cannot
  silently accumulate.

Time-bounding is the temporal dimension of *everything managed and governed*:
things do not just exist and persist — they are valid *for a stated time*, then
must be renewed, re-verified, or retired.

## Why these axioms

| Axiom | Guarantees |
| --- | --- |
| Defined | interoperability, no undefined behavior, self-description |
| Data | inspectability, replayability, one substrate |
| Measurable | observability, evidence-based decisions, enforceable SLOs |
| Published / versioned / signed | discoverability, reproducibility, provenance, accountability |
| Managed | controlled lifecycle; nothing appears or disappears silently |
| Governed | universal authority; no privileged ungoverned path |
| Time-bound | temporal validity by default; expired = untrusted; no permanent authority |

Together they mean: **anything in UAF can be found, understood, validated,
measured, reproduced, attributed, lifecycle-managed, and governed** — which is the
precondition for trusting an autonomous, federated, world-acting system.

> Status: design direction. The axioms are invariants the registry, schemas,
> metrics, and signing model enforce; they constrain *how* every other layer is
> built rather than adding wire-format fields.
