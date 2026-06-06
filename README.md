# Universal Agent Framework (UAF)

> A **protocol-first**, **kernel-native** runtime for **multi-platform**,
> **multi-language** multimodal agents.

UAF is original research that generalizes the multi-agent design introduced in
[*A Unified Multi-Agent Framework for Universal Multimodal Understanding and
Generation* (arXiv:2508.10494)](https://arxiv.org/abs/2508.10494) — the
**MAGUS** system — into a portable runtime and an open wire protocol that any
language, on any platform, can speak.

Where MAGUS demonstrates that role-conditioned agents (Perceiver, Planner,
Reflector) collaborating in a shared textual workspace can drive training-free
any-to-any multimodal understanding and generation, UAF asks the systems
question that follows:

> *What is the smallest stable substrate that lets those agents run anywhere,
> in any language, on any platform — and interoperate?*

UAF's answer has three layers.

## The three layers

### 1. Protocol first — the Universal Agent Protocol (UAP)

Everything starts from a language-agnostic, transport-agnostic wire protocol.
Agents do not import each other; they exchange **entries** in a shared
**workspace** (a blackboard) and advance through two phases — **Cognition** and
**Deliberation** — defined entirely by the protocol. The protocol is the
contract; implementations are interchangeable.

→ [`spec/PROTOCOL.md`](./spec/PROTOCOL.md)

### 2. Kernel-native runtime core

A small **native kernel** is the reference host for UAP. It owns the workspace,
enforces the phase state machine, routes entries between participants, and
dispatches generation/understanding tools. It is compiled (no managed runtime
required), exposes a stable **C ABI**, and is **multi-platform** — the same core
embeds into a desktop app, a server, an edge device, or a browser (via WASM).

→ [`kernel/README.md`](./kernel/README.md) · [`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md)

### 3. Multi-language bindings

Because participation is "speak UAP to the kernel," agents and tools can be
written in **any language**. Thin SDK bindings (over the C ABI in-process, or
over stdio/sockets out-of-process) make each language feel native.

→ [`bindings/README.md`](./bindings/README.md)

## Why these constraints matter

| Principle | What it buys |
| --- | --- |
| **Protocol first** | Implementations, languages, and platforms stay interchangeable; the spec is the source of truth, not any one codebase. |
| **Kernel native** | One small compiled core to port and audit; deterministic scheduling; no per-language reimplementation of orchestration. |
| **Runtime core** | The kernel is a *library you embed*, not a service you must deploy — agents are hosted, not glued together. |
| **Multi-language** | Use the best model SDK / library per agent, regardless of language. |
| **Multi-model** | Model-agnostic by construction: backends are injected, so each role can use a different MLLM/diffusion model and providers can be mixed or swapped without touching the protocol. |
| **Multi-framework** | A meta-framework: agents and tools built in other frameworks (LangChain, AutoGen, CrewAI, MCP servers, A2A) join as UAP participants through thin adapters — speak the protocol, keep your stack. |
| **Multi-platform** | The same agents run on server, desktop, edge, and web without change. |
| **Cloud-native, cloud-agnostic, K8s-native** | Participants are workloads; the **agent is the orchestrator and the operator of tools** (the Kubernetes operator/control-loop pattern), with no vendor lock-in. |
| **Canonical real-world agents** | A curated standard library of reference agents bound to real tasks, not toy demos. |
| **Human at the Edge** | The human is a first-class participant at the decision boundary (and the network edge), gating high-stakes steps. |
| **Agent at the Gates** | An agent stands as a policy-enforcement point at every boundary; agents gate at scale, humans gate the residual. |
| **Everything as data** | No out-of-band control channels — tasks, plans, policies, trust, even control flow are entries on one replayable, auditable log. |
| **Five primitives** | **Trust, Time, Location, Identity, Relation** are first-class on every entry (the `ctx` block); all policy is a predicate over them. |
| **Secure by data model** | Every entry is protected, auditable, encrypted at rest and in transit, and **three-party signed** (author + gate + kernel) at each exchange. |
| **Federated ecosystem** | Independent domains (nations, orgs, clouds, devices) interoperate peer-to-peer with no central authority — a **multi-nation, multi-device, multi-agent, multi-model** ecosystem. |
| **Sovereign by boundary** | Cross-domain federation gates enforce residency/jurisdiction and re-sign at the border, giving verifiable cross-domain chain of custody. |
| **World-grounded** | Reflects real-world constraints, conflicts, capabilities, knowledge, resources, theories, institutions, organizations, and the social-economic fabric as a modeled World Model — so plans are feasible, lawful, affordable, and accountable. |
| **Constitutionally governed** | **Rule, Guardrail, Constitution, Duty & Right** are first-class deontic concepts; a per-domain constitution outranks all other norms and is enforced at the gates. |
| **Axiomatic** | Everything is **data, defined, measurable**, and **published, version-controlled & signed** — the invariants that make the system inspectable and reproducible. |
| **Clarity & accountability** | Clarity (legible, self-describing, rationale-recorded) and accountability (attributable, answerable, traceable) are first-class design principles. |
| **Sanity check is mandatory** | Validation is non-skippable: the reflector sanity-checks the plan before execution and the result before return. |
| **Common communication language** | UAP is the single shared language — one envelope plus a shared ontology — that every agent, model, framework, and domain speaks. |
| **Blockchain as a tool** | A distributed ledger is an *optional, pluggable* tool for notarization, versioning, federation trust anchoring, and economic settlement — never a required dependency. |
| **Universal identifier for everyone** | Every entity — agent, human, tool, model, org, domain, artifact, definition, session — has one **Universal Decentralized Canonical Identifier** (DID-compatible, self-sovereign, content-addressed). |
| **Stable World Model** | The world model is stable, versioned, internally consistent, and reproducible from point-in-time snapshots — it evolves only through governed change, never silent drift. |

## Architecture at a glance

```
                          ┌──────────────── Universal Agent Protocol (UAP) ────────────────┐
                          │            entries · phases · workspace · tool calls            │
                          └────────────────────────────────────────────────────────────────┘
                                                       ▲   ▲   ▲
                in-process (C ABI)                     │   │   │             out-of-process (stdio / socket / WASM host)
        ┌───────────────────────────────────┐         │   │   │        ┌───────────────────────────────────────────┐
        │  Kernel (native runtime core)      │◀────────┘   │   └───────▶│  Remote participants (any platform)         │
        │  • shared workspace (blackboard)   │             │            └───────────────────────────────────────────┘
        │  • phase state machine             │             │
        │  • entry router + scheduler        │      ┌──────┴──────┐
        │  • tool dispatch                   │      │  Bindings   │  Rust · TypeScript · Python · Go · Swift · …
        └───────────────────────────────────┘      └─────────────┘
                          ▲
        ┌─────────────────┴───────────────────────────────────────────────────┐
   Agents:  Perceiver   Planner   Reflector   Executor      Tools: image/audio/video/text generators
```

## Status

This repository is being **initialized protocol-first**. The current contents
define the protocol, the kernel/runtime architecture, the binding model, and a
worked example session. Implementation of the native kernel and the first
bindings follows from this spec.

## Repository layout

```
spec/        Universal Agent Protocol — the normative specification + JSON Schemas
kernel/      Native runtime core (the protocol's reference host) — design + scaffold
bindings/    Multi-language SDK strategy and per-language binding scaffolds
docs/        Architecture, design decisions (ADRs), and rationale
```

## Documentation

- [`spec/PROTOCOL.md`](./spec/PROTOCOL.md) — the Universal Agent Protocol (normative)
- [`spec/conformance.md`](./spec/conformance.md) — conformance levels and rules
- [`docs/DESIGN_PRINCIPLES.md`](./docs/DESIGN_PRINCIPLES.md) — everything-as-data, five primitives, clarity, accountability, mandatory sanity check
- [`docs/AXIOMS.md`](./docs/AXIOMS.md) — defined · measurable · published/versioned/signed (+ blockchain as a tool)
- [`docs/GOVERNANCE.md`](./docs/GOVERNANCE.md) — Rule, Guardrail, Constitution, Duty & Right as first-class concepts
- [`docs/IDENTIFIERS.md`](./docs/IDENTIFIERS.md) — Universal Decentralized Canonical Identifier (UDCI) for every entity
- [`docs/DATA_SECURITY.md`](./docs/DATA_SECURITY.md) — protection, audit, encryption, three-party signing
- [`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md) — kernel, runtime core, multi-platform/-language design
- [`docs/CANONICAL_AGENTS.md`](./docs/CANONICAL_AGENTS.md) — the canonical real-world agent catalog
- [`docs/CLOUD_NATIVE.md`](./docs/CLOUD_NATIVE.md) — cloud-native, cloud-agnostic, K8s-native; agent-as-operator
- [`docs/FEDERATION.md`](./docs/FEDERATION.md) — federated, multi-nation/-device/-agent/-model ecosystem
- [`docs/WORLD_MODEL.md`](./docs/WORLD_MODEL.md) — reflecting real-world constraints, conflicts, knowledge, institutions, economics
- [`docs/HUMAN_AT_THE_EDGE.md`](./docs/HUMAN_AT_THE_EDGE.md) — human as a first-class, edge-deployed participant
- [`docs/AGENT_AT_THE_GATES.md`](./docs/AGENT_AT_THE_GATES.md) — agents as policy-enforcement points at every boundary
- [`docs/adr/0001-kernel-language.md`](./docs/adr/0001-kernel-language.md) — kernel implementation decision

## License

[MIT](./LICENSE)

## Citation

```bibtex
@article{magus2025,
  title         = {A Unified Multi-Agent Framework for Universal Multimodal Understanding and Generation},
  year          = {2025},
  eprint        = {2508.10494},
  archivePrefix = {arXiv}
}
```
