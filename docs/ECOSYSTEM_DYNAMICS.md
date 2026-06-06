# Advancement of the Fittest, Support to the Weak

The UAF ecosystem runs on a deliberate dual law: **selection pressure** advances
the fittest, and **solidarity** supports the weak. Neither alone is acceptable —
pure selection produces a brittle monoculture that abandons the vulnerable; pure
support entrenches failure and stalls. UAF holds both in tension, by policy.

## Advancement of the fittest

Fitness is **measured**, not asserted ([everything measurable](./AXIOMS.md)), and
the fittest **advance**:

- **Fitness signals** — performance, quality, SLO attainment, cost-efficiency,
  conformance, outcome-vs-goal, adoption.
- **What advancing means** — more routing/usage, more
  [resource budget](./WORLD_MODEL.md), promotion through the
  [lifecycle](./DESIGN_PRINCIPLES.md) (publish → deploy), becoming
  [canonical products](./CANONICAL_AGENTS.md), and propagating across
  [federation](./FEDERATION.md). Underperformers are deprecated and retired.
- **Open competition** — multiple agents/tools/models/solutions ([multi-*](../README.md))
  compete on the same task against shared metrics and fixtures; selection/routing
  picks the fittest *for the context*; economic allocation rewards performance.
- **Contextual, not global** — "fittest" is per task, domain, and constraint, so
  different niches keep different winners. This is what prevents monoculture.

## Support to the weak

UAF is not pure Darwinism. Weaker participants are **supported**, not optimized
out:

- **Graceful degradation & fallbacks** — when the strongest tool is unavailable or
  too costly, weaker-but-available alternatives carry the load
  ([no single point of failure](./CLOUD_NATIVE.md)).
- **Scaffolding** — newer/weaker agents inherit canonical templates, supervision,
  and the reflector/gate safety net; [the human](./HUMAN_AT_THE_EDGE.md) assists
  where autonomy is insufficient.
- **Fairness & inclusion** — low-resource domains, edge devices, and humans are
  first-class; [rights](./GOVERNANCE.md) protect the vulnerable, and the system
  does not optimize them away.
- **Redistribution** — resource/economic policy can set **budget floors** and
  subsidies, so capability is not reserved for the fastest or richest.
- **Diversity preservation** — minority and weaker approaches are kept alive
  ([epistemic humility](./WORLD_MODEL.md)); today's weak may be tomorrow's fittest,
  and a diverse population is more resilient.

## Holding the two in tension

The forces are balanced by **governance**:

- The [constitution](./GOVERNANCE.md) can mandate *both* a meritocratic selection
  policy *and* support floors and rights — neither is optional.
- The `x-arbiter` allocates contested resources and resolves the selection-vs-support
  tradeoff under declared policy.
- **Guardrails both ways:** selection MUST NOT violate rights or duties; support is
  itself **measured and [time-bound](./AXIOMS.md)** — the weak are helped *and still
  assessed*, so support lifts rather than entrenches.

## How it fits

| This rests on | In |
| --- | --- |
| fitness measurement | [Axioms: measurable](./AXIOMS.md) |
| promote / retire by fitness | [lifecycle](./DESIGN_PRINCIPLES.md) |
| allocation, floors, subsidies | [resources / economics](./WORLD_MODEL.md) |
| fairness, rights, mandated support | [Governance](./GOVERNANCE.md) |
| assistance to the weak / human | [Human at the Edge](./HUMAN_AT_THE_EDGE.md) |
| supporting weaker domains/devices | [Federation](./FEDERATION.md) |
| diversity over monoculture | [epistemic humility](./WORLD_MODEL.md) |

> Status: design direction. Selection and support are policies over measurement,
> lifecycle, resources, and governance — no wire-format change.
