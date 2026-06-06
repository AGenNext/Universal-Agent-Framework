# Multiple Trinities

Three-fold structures recur throughout UAF. This is not coincidence but a **design
motif**: three is the minimal structure that pairs *production* with a *check* —
two elements that act, and a **third that verifies, mediates, or anchors**. The
recurring "third" is the structural form of UAF's
[mandatory sanity check](./DESIGN_PRINCIPLES.md) and
[accountability](./DESIGN_PRINCIPLES.md): nothing acts without something that
checks it.

## The trinities

| Trinity | Elements | The "third" |
| --- | --- | --- |
| **Cognition roles** | Perceiver · Planner · **Reflector** | Reflector *checks* |
| **LLM faces** ([role](./LLM_ROLE.md)) | Interpreter · Translator · Transformer | three faces of one function over data |
| **Signing** ([security](./DATA_SECURITY.md)) | Author · Kernel · **Gate** | Gate *verifies policy* (author makes, kernel orders) |
| **Architecture layers** | Protocol · Kernel · Bindings | the seam, the core, the reach |
| **Core stack** ([access](./ACCESS.md)) | Name (UDCI) · Access (uniform) · Coordinate (UAP) | name → reach → act |
| **Digital-twin loop** ([twin](./DIGITAL_TWIN.md)) | Observe · Actuate · **Reconcile** | Reconcile *checks twin vs. reality* |
| **Gate outcomes** ([gates](./AGENT_AT_THE_GATES.md)) | Pass · Block · **Escalate** | Escalate *defers to judgment* |
| **Oversight** | Gate (prevent) · Human (judge) · **Log** (record) | the Log *makes it accountable* |
| **Conformance** ([levels](../spec/conformance.md)) | L0 cognition · L1 single-modality · L2 any-to-any | increasing capability |
| **Temporal** ([time](./DESIGN_PRINCIPLES.md)) | Past (real) · Present (**edge**) · Future (provisional) | the Present *gates* change between settled past and provisional future |

## The pattern: two act, the third checks

Most UAF trinities take one of two shapes, and both put a verifier in the third
seat:

- **make · order · verify** — e.g. signing (author makes, kernel orders, **gate
  verifies**); cognition (perceive, plan, **reflect**).
- **two poles · a mediator** — e.g. twin (observe ↔ actuate, **reconcile**
  between); oversight (prevention ↔ judgment, **record** binds them).

This is why the design *feels* balanced: wherever two forces could run unchecked,
a third element — a reflector, a gate, a reconciler, a log — sits between them and
keeps the system honest. The trinities are the sanity-check principle made
structural.

## Why name them

- **Clarity.** Recognizing the shared shape makes the architecture easier to learn:
  learn the motif once, see it everywhere.
- **Consistency.** New features should reuse the motif — when you add two acting
  parts, add the third that checks them.
- **Robustness.** A missing "third" is a smell: a place where something acts without
  verification. The trinities are a checklist against unchecked power.

> Status: observational. The trinities describe structure already present across the
> spec; naming them is a design discipline, not a wire-format change.
