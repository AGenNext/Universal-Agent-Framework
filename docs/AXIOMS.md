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

## Why these four

| Axiom | Guarantees |
| --- | --- |
| Defined | interoperability, no undefined behavior, self-description |
| Data | inspectability, replayability, one substrate |
| Measurable | observability, evidence-based decisions, enforceable SLOs |
| Published / versioned / signed | discoverability, reproducibility, provenance, accountability |

Together they mean: **anything in UAF can be found, understood, validated,
measured, reproduced, and attributed** — which is the precondition for trusting an
autonomous, federated, world-acting system.

> Status: design direction. The axioms are invariants the registry, schemas,
> metrics, and signing model enforce; they constrain *how* every other layer is
> built rather than adding wire-format fields.
