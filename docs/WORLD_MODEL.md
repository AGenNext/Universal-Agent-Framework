# Reflecting the Real World

Agents do not act in a vacuum — they act in a world of **constraints, conflicts,
challenges, capabilities, knowledge, resources, tools, theories, concepts,
institutions, organizations, and a social-economic fabric**. UAF treats that
world as a first-class, *modeled* substrate rather than implicit context.

This is *everything-as-data* taken to its conclusion: **the world itself is
data** — a shared, typed, versioned **World Model** that perceivers ground into,
planners reason over, gates enforce against, and reflectors check feasibility
with. Every world record carries the core [primitives](./DESIGN_PRINCIPLES.md)
(Trust, Time, Location, Identity, Relation), so knowledge can be sourced,
expired, located, attributed, and related — just like any other entry.

## What UAF models

| Real-world facet | Represented in UAF as | Bound by |
| --- | --- | --- |
| **Constraints** | enforceable predicates (legal, physical, resource, temporal, ethical) | gate policy + reflector feasibility check |
| **Conflicts** | competing goals/claims surfaced as entries; resolved by negotiation/arbitration | `x-arbiter` role, Human at the Edge |
| **Challenges** | the `task` goal + open uncertainty recorded as world unknowns | planner decomposition, reflector |
| **Capabilities** | what an agent/org *can* do — advertised in `hello`, scoped by identity/trust | capability gating |
| **Knowledge** | facts/beliefs with provenance, confidence, and validity windows | Trust + Time primitives |
| **Resources** | scarce, accounted quantities (compute, energy, money, data, attention) | budgets + metering on dispatch |
| **Tools** | operands the agent operates | capability descriptor, agent-as-operator |
| **Theories** | explicit models/methods an agent applies to reason | knowledge records, cited in plans |
| **Concepts** | a shared ontology/vocabulary giving cross-agent/-domain interop meaning | the World Model schema |
| **Institutions** | rules of the game (law, norms, standards) | constraint/policy sets bound to jurisdictions |
| **Organizations** | structured actors (firms, agencies, NGOs) | Identity + Relation + federation domains |
| **Social-economic fabric** | relationships, markets, incentives, trust networks | Relation primitive at scale |

## Constraints as enforceable policy

Real-world limits are not advice — they are **enforced**. A constraint is a
predicate over the core primitives and the World Model. It lives as data and is
applied at two points:

- **Gates** ([Agent at the Gates](./AGENT_AT_THE_GATES.md)) block an entry or
  `tool_call` that would violate a constraint (e.g. residency, budget, safety,
  legality) — the constraint *is* the gate's policy.
- **Reflector** rejects a `plan` that is infeasible under the active constraints,
  forcing a `revision`.

A plan that cannot satisfy its binding constraints does not execute; the
violation is recorded as data, auditable after the fact.

## Conflict and arbitration

The world has competing goals, contested resources, and adversarial actors. UAF
surfaces conflict instead of hiding it:

- Disagreement is expressed as ordinary entries (a `critique`, a competing
  `plan`, a contested knowledge record).
- An **`x-arbiter`** participant mediates — applying a declared resolution
  policy (priority, negotiation, voting, market clearing) and recording the
  rationale as data.
- **Value tensions** and high-stakes trade-offs escalate to
  [Human at the Edge](./HUMAN_AT_THE_EDGE.md) rather than being auto-resolved.

## Resources and the social-economic fabric

Resources are scarce and **accounted**, not assumed free:

- Sessions and agents carry **budgets**; tool dispatch is **metered** (compute,
  energy, money, data, attention). Exceeding a budget is a constraint violation
  gated like any other.
- The agent-as-operator ([Cloud-Native](./CLOUD_NATIVE.md)) provisions and scales
  tool operands *within* resource policy.
- The **social-economic fabric** — markets, incentives, trust networks — is the
  Relation primitive at scale: agents and organizations allocate, trade, and
  delegate under budgets and relationships, all recorded as data.

## Knowledge, theories, and concepts (epistemics)

- **Knowledge** records carry provenance (Identity), confidence (Trust), and
  validity (Time) — beliefs can be sourced, doubted, and expired. Contested
  knowledge is handled like any other conflict.
- **Theories** are explicit, citable models an agent applies; a plan can name the
  theory it reasons under, making its assumptions inspectable.
- **Concepts** form a shared **ontology** — the vocabulary that makes
  cross-agent and cross-domain ([Federation](./FEDERATION.md)) collaboration
  *mean* the same thing on both sides.

## Theory of everything is a theory (epistemic humility)

No model is absolute truth. Every model, ontology, World Model, digital twin — and
even a "theory of everything" — is a **theory**: explicit, assumption-bearing,
confidence-rated, time-bound, versioned, and **revisable**. UAF builds this
humility in, because a system that acts in the world will confidently act on error
unless it holds its models as provisional.

- **The map is not the territory.** The World Model and its
  [digital twins](./DIGITAL_TWIN.md) *approximate* reality; divergence is expected
  and reconciled, never assumed away.
- **Fallibilism.** Any belief can be wrong. Contradicting evidence is first-class —
  it triggers [revision](#conflict-and-arbitration), not denial. Knowledge carries
  confidence (Trust) and validity (Time).
- **Provisional by construction.** Theories are cited in plans *with their
  assumptions*; competing theories may coexist and be arbitrated; superseded
  theories are versioned, not erased.
- **Including this framework.** UAF's own axioms and principles are a theory —
  defined, versioned, and open to amendment under its own
  [governance](./GOVERNANCE.md). No self-exemption.
- **Decisions hedge.** Under uncertainty, plans carry confidence and fallbacks; the
  [sanity check](./DESIGN_PRINCIPLES.md) accounts for model risk; high-stakes calls
  escalate to [the human](./HUMAN_AT_THE_EDGE.md).

This is the guardrail against false certainty: hold every model — however
comprehensive — as a theory, subject to evidence.

## Institutions and organizations

- **Institutions** (law, regulation, norms, standards) are encoded as constraint
  and policy sets bound to jurisdictions — the same sets a federation gate
  enforces at a multi-nation boundary.
- **Organizations** are structured actors modeled via Identity, Relation, and
  federation domains: an org is a domain (or sub-domain) with its own trust root,
  members, capabilities, and delegation chains.

## A stable World Model

The World Model is only useful if agents can **rely** on it. Stability is a
requirement, not an emergent property — the model must not drift unpredictably
under agents' feet:

- **Stable identifiers.** Every world record and ontology concept has a canonical
  [UDCI](./IDENTIFIERS.md) — content-addressed for data, so a reference never
  silently changes meaning. What you cited is what you get.
- **Versioned, not mutated.** The ontology and world records evolve only through
  **governed, versioned amendment** ([Governance](./GOVERNANCE.md),
  [Axioms](./AXIOMS.md)); prior versions remain immutable and resolvable. Change
  produces a new version, never an in-place rewrite — no silent drift.
- **Internally consistent.** Contradictions are detected and reconciled as
  [conflicts](#conflict-and-arbitration) (arbiter / human), so the model maintains
  consistency invariants rather than accumulating quiet contradictions.
- **Point-in-time snapshots.** Facts carry validity windows (the Time primitive),
  so the model is **stable as of** a snapshot. Reasoning is reproducible: replay a
  session against the same snapshot and get the same grounding.
- **Convergent under federation.** Domains share a stable **core ontology** and
  extend it locally; the shared core changes slowly and by agreement, so
  cross-domain meaning stays stable even as edges innovate.

Stability + the core primitives is what lets the planner trust an observation, the
reflector reproduce a check, and a federated peer agree on what a concept *means*.

## Why this matters

Grounding agents in a modeled world is what makes their output **feasible,
lawful, affordable, and accountable** rather than plausible-in-a-vacuum. The
planner proposes within constraints, the gates enforce the rules of the game, the
arbiter resolves conflict, resources are paid for, and every assumption —
knowledge, theory, institution — is inspectable data with provenance.

> Status: design direction. The World Model is content expressed as data and
> policy over the existing core primitives; it adds no envelope fields. New roles
> (`x-arbiter`) and typed world records extend the system additively.
