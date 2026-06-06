# The LLM as Interpreter, Translator, Transformer

UAF gives the LLM a **precise, bounded role**. It is a *function over data* with
three faces — **interpreter, translator, transformer** — not an oracle, an
authority, or a source of truth. Ground truth lives in the
[World Model](./WORLD_MODEL.md); decisions are disposed by [gates](./AGENT_AT_THE_GATES.md),
the [mandatory sanity check](./DESIGN_PRINCIPLES.md), the arbiter, and
[the human](./HUMAN_AT_THE_EDGE.md). The LLM *proposes within* that frame.

This framing is what lets a **non-deterministic** component live safely inside a
**deterministic, accountable** system: determinism and accountability come from the
protocol, not from the model.

## The three faces

| Face | What it does | Where in UAF | Example |
| --- | --- | --- | --- |
| **Interpreter** | parse ambiguous natural-language / multimodal input into a structured representation grounded in the shared ontology | Perceiver; `understand` steps; reading a `task` | image → structured observation; a user request → a typed goal |
| **Translator** | faithfully convert *between* languages, modalities, formats, ontologies, and protocols | cross-modality/-language steps; [framework adapters](../bindings/README.md); cross-domain concept mapping at [federation](./FEDERATION.md) boundaries | text → image prompt; a LangChain/MCP call → a UAP entry; Domain A's concept → Domain B's concept |
| **Transformer** | restructure, synthesize, or rewrite content into a new form | Planner (goal → plan); `generate`/transformation steps | long document → brief; caption → caption-card prompt |

Each face is a **defined input → output contract** (interpret, translate,
transform) — which is exactly what makes an LLM call inspectable, testable, and
[measurable](./AXIOMS.md), rather than an open-ended act of cognition.

## What the LLM is *not*

- **Not the source of truth.** Facts and knowledge live in the World Model with
  provenance, confidence, and validity; the LLM *interprets and translates* them —
  it does not author ground truth.
- **Not the authority.** It proposes; gates, the reflector's mandatory sanity
  check, the arbiter, and the human dispose. A model's confidence is not consent.
- **Not ungoverned.** Every LLM act is an entry — defined input/output, signed,
  gated, measured, sanity-checked. *Everything governed* applies to the model too.
- **Not privileged or fixed.** Agents are role-conditioned, provider-agnostic
  ([multi-model](../README.md)) variants; a model is a swappable
  module/product like any other.

## Mapping to roles and steps

| UAF role / step | LLM face(s) |
| --- | --- |
| Perceiver | Interpreter |
| Planner | Interpreter (of the goal) + Transformer (into a plan) |
| Reflector | Interpreter (checks meaning, feasibility, and the sanity check) |
| `understand` step | Interpreter / Translator |
| `generate` / transformation step | Transformer |
| Framework adapter / federation boundary | Translator |

## The LLM as a library

The model is consumed as a **library**, not a service you defer to or an oracle
you trust. Like any library you embed:

- **Stateless & callable.** It exposes the three faces (interpret / translate /
  transform) as callable capabilities behind a stable contract; it holds no
  session authority of its own.
- **Versioned, published, swappable.** A model library has a [UDCI](./IDENTIFIERS.md),
  is published and versioned ([axioms](./AXIOMS.md)), and is a
  [multi-model](../README.md), swappable dependency — different agents/roles can
  bind different model libraries.
- **No transfer of authority.** *Calling a library does not hand it your decision
  rights.* The calling actor remains responsible for the result; gates, the sanity
  check, and the human still dispose.
- **Operated via commands, governed & metered.** Each invocation is an
  [action-as-command](./ACTIONS.md) — defined I/O, signed, gated, and metered like
  any dependency call.

So the LLM is a *dependency the actor calls and remains accountable for*, not
infrastructure the actor obeys.

## Why this framing matters

| Benefit | Because the LLM is bounded to interpret / translate / transform |
| --- | --- |
| **Safety** | a non-deterministic component is confined to function-over-data; the protocol supplies determinism and accountability |
| **Interoperability** | the *translator* face is the engine of multi-language / -model / -framework / -domain interop — translating everything into the common language and shared ontology |
| **Clarity & testability** | every call has a defined contract, so it can be specified, measured, and verified |
| **Swappability** | because the role is a contract, any conformant model fulfils it |

> Status: design direction. This is a role constraint on how models are used —
> realized through role-conditioning, defined I/O contracts, and the gate/sanity
> mechanisms — not a wire-format change.
