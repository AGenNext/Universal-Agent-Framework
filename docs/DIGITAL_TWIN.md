# Digital Twin Theory

UAF realizes the [World Model](./WORLD_MODEL.md) as a **living graph of digital
twins**: synchronized data representations of real-world entities. A twin is the
**bridge between data and world** — agents *observe* through it, *rehearse*
against it, and *act* through it. This is how "reflect the real world" becomes
operational, and how autonomous agents act **safely**: rehearse in the twin, then
act on reality.

## What a twin is

A digital twin is a **live, queryable, synchronized** representation of a
real-world entity — a device, resource, process, organization, institution, or
(with [consent](./HUMAN_AT_THE_EDGE.md)) a person. Each twin:

- has a [UDCI](./IDENTIFIERS.md) and is [accessed uniformly](./ACCESS.md);
- mirrors its counterpart's **state** as data;
- carries **fidelity** (the Trust primitive), **freshness** (Time), and
  **provenance** (Identity) on every attribute;
- is governed, versioned, and signed like everything else.

## The twin sync loop (bidirectional)

```
        observe  (reality → twin)                    actuate  (twin → reality)
   sensors / perceiver / tools  ──▶  ┌─────────┐  ──▶  agent-as-operator (gated, JIT,
   update twin state, time-anchored  │  TWIN   │       sanity-checked) drives reality
                                     └─────────┘
                         reconcile divergence ↺ (conflict → arbiter / human)
```

- **Observe.** Perceiver observations and tool/sensor inputs update the twin —
  time-anchored and provenance-stamped.
- **Actuate.** Commands on the twin propagate to reality via
  [agent-as-operator](./CLOUD_NATIVE.md) as gated, JIT-authorized
  [commands](./ACTIONS.md); the twin is the **control surface** for the real entity.
- **Reconcile.** When twin and reality diverge, the gap is surfaced as a
  [conflict](./WORLD_MODEL.md) and reconciled; a stale twin is **degraded** until
  re-synced.

## Simulation and rehearsal

Twins make **counterfactual simulation** first-class. Before committing to reality,
the planner proposes and the reflector **rehearses** candidate plans against the
twin — estimating feasibility, cost, safety, and constraint satisfaction in
simulation. The [mandatory sanity check](./DESIGN_PRINCIPLES.md) **MAY require a
passing simulation** before a mutating command is authorized.

> Rehearse in the twin; act on the world. Mistakes happen in simulation, not in
> reality.

## Fidelity, freshness, provenance

A twin is only as trustworthy as its sync. Every attribute carries:

- **Fidelity** — confidence in the value (Trust);
- **Freshness** — validity window since last sync (Time); an expired attribute is
  not relied upon ([time-bound](./AXIOMS.md));
- **Provenance** — what observation/source set it (Identity).

A twin is **stable as of** a snapshot ([stable world model](./WORLD_MODEL.md)), so
reasoning and audits are reproducible, and decisions record the fidelity they
relied on.

## Twins of everything (composable, governed)

- **Of everything** — devices, resources, processes, organizations (which are also
  [domains](./FEDERATION.md)), institutions, and consented people.
- **Composable** — a composite twin (a solution's or organization's twin) composes
  sub-twins, matching the [composability](./DESIGN_PRINCIPLES.md) of everything else.
- **Governed** — person twins require consent and honor data-subject rights;
  actuation twins are gated and human-escalated for high-stakes change.

## How it fits

| Digital twins rest on | In |
| --- | --- |
| the modeled world they instantiate | [World Model](./WORLD_MODEL.md) |
| actuation to reality | [agent-as-operator / Cloud-Native](./CLOUD_NATIVE.md) |
| simulation as a sanity gate | [mandatory sanity check](./DESIGN_PRINCIPLES.md) |
| sync, snapshots, reproducibility | [time as anchor](./DESIGN_PRINCIPLES.md) |
| fidelity / freshness / provenance / locus / graph | the five primitives |
| consent for person twins | [Human at the Edge](./HUMAN_AT_THE_EDGE.md) |

> Status: design direction. Digital twins are live World-Model entities expressed
> as data over the five primitives, actuated by commands and rehearsed in
> simulation — no wire-format change.
