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
| **Foundational** | A minimal substrate others build on — a small protocol + kernel + primitives that everything else (agents, tools, solutions) layers onto. |
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
| **Digital twins** | Real-world entities have live, synchronized twins: agents observe (reality→twin), rehearse in simulation, and actuate (twin→reality) through gated commands — rehearse in the twin, act on the world. |
| **Epistemic humility** | No model is absolute truth — every ontology, world model, and even a "theory of everything" is a provisional, versioned, revisable theory, including UAF's own axioms. |
| **Multiple trinities** | A recurring design motif: two elements act and a third verifies/mediates/anchors (perceive·plan·reflect; author·kernel·gate; observe·actuate·reconcile) — the sanity check made structural. |
| **Constitutionally governed** | **Rule, Guardrail, Constitution, Duty & Right** are first-class deontic concepts; a per-domain constitution outranks all other norms and is enforced at the gates. |
| **Axiomatic** | Everything is **data · defined · measurable · published/versioned/signed · managed · governed · a product** — the invariants that make the system inspectable, reproducible, controlled, and accountable. |
| **Composable** | Primitives, modules, features/products, and solutions compose through one seam (UAP + uniform access over UDCIs); composition nests recursively and preserves every guarantee. |
| **Product & service oriented** | Every entity is a product (owner, contract, SLOs, lifecycle); products compose into **solutions delivered as a service** (self-service, metered, multi-tenant, SLO-backed). |
| **Clarity & accountability** | Clarity (legible, self-describing, rationale-recorded) and accountability (attributable, answerable, traceable) are first-class design principles. |
| **Sanity check is mandatory** | Validation is non-skippable: the reflector sanity-checks the plan before execution and the result before return. |
| **Common communication language** | UAP is the single shared language — one envelope plus a shared ontology — that every agent, model, framework, and domain speaks. |
| **Blockchain as a tool** | A distributed ledger is an *optional, pluggable* tool for notarization, versioning, federation trust anchoring, and economic settlement — never a required dependency. |
| **Universal identifier for everyone** | Every entity — agent, human, tool, model, org, domain, artifact, definition, session — has one **Universal Decentralized Canonical Identifier** (DID-compatible, self-sovereign, content-addressed). |
| **Stable World Model** | The world model is stable, versioned, internally consistent, and reproducible from point-in-time snapshots — it evolves only through governed change, never silent drift. |
| **Uniform, fine-grained access** | One interface (resolve/read/write/query/subscribe/invoke) to every UDCI-named resource, with access resolving down to the field/verb/capability under least privilege. |
| **Modular** | Composition over monolith: everything outside the minimal kernel is an independently versioned, published, signed, swappable module. |
| **Feature first** | The unit of design and delivery is the feature — a self-contained vertical slice (agents, tools, definitions, norms, access policy, tests) — not a technical layer. |
| **LLM as interpreter / translator / transformer** | The model is a bounded *function over data* (and a swappable *library*), never the oracle, authority, or source of truth — gates, the sanity check, and the human decide. |
| **Agent as actor (and as tool)** | Agents are isolated, message-passing, supervised actors *and* accountable parties with identity, duties & rights — and any actor can present a tool face, composing recursively. |
| **Actions as commands** | Every action is a reified command — issued, validated, authorized (gate + capability + governance + sanity), executed, and recorded as an immutable event. |
| **Zero trust** | Never trust, always verify: no perimeter trust, trust computed per-request from `ctx`, re-verified at every boundary, least privilege throughout. |
| **JIT authentication** | No standing access: ephemeral, per-action, time-boxed credentials minted just-in-time and expiring immediately — least privilege in time as well as scope. |
| **Time-bound everything** | Every entity, grant, fact, trust assertion, plan, norm, and identity carries temporal validity and expires; nothing is permanent by default, and expired = untrusted. |
| **Time as an anchor** | Time is the reference frame everything is pinned to — order, causality, provenance, reproducible point-in-time snapshots, and cross-domain synchronization. Time both limits authority and locates truth. |
| **Durable executions** | A session is a durable workflow: execution state is the persisted, signed, replayable log, so it survives crashes, restarts, and long async waits and resumes exactly where it left off. |
| **No single point of failure** | Resilience by redundancy: no central authority, a replicated/leader-elected kernel over a replicated journal, stateless supervised actors — components fail, the system does not. |
| **Multi-channel connectivity** | Every node is reachable over multiple transports and redundant peer-to-peer paths with channel-independent UDCI addressing and store-and-forward when disconnected. |
| **Tunnel is gated** | Every channel/tunnel is itself a gated boundary — mutual auth, connect-capability, JIT time-bound credential; no ungated path. Connecting grants reach, not trust. |
| **Military-grade discipline** | Fail-safe/deny-by-default, rules of engagement, chain of command, verify-before-act, defense-in-depth, redundancy, and after-action review — uniform and non-negotiable. |

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
- [`docs/LLM_ROLE.md`](./docs/LLM_ROLE.md) — the LLM as interpreter, translator, transformer (and as a library)
- [`docs/AGENT_AS_ACTOR.md`](./docs/AGENT_AS_ACTOR.md) — agent as actor (and as tool); the actor model + accountable actor
- [`docs/ACTIONS.md`](./docs/ACTIONS.md) — actions as commands; the command/event discipline
- [`docs/DURABLE_EXECUTION.md`](./docs/DURABLE_EXECUTION.md) — durable, crash-tolerant, resumable executions via the replayable log
- [`docs/CANONICAL_AGENTS.md`](./docs/CANONICAL_AGENTS.md) — the canonical real-world agent catalog
- [`docs/ACCESS.md`](./docs/ACCESS.md) — Uniform Access Protocol: one fine-grained interface to every UDCI-named resource
- [`docs/CLOUD_NATIVE.md`](./docs/CLOUD_NATIVE.md) — cloud-native, cloud-agnostic, K8s-native; agent-as-operator
- [`docs/FEDERATION.md`](./docs/FEDERATION.md) — federated, multi-nation/-device/-agent/-model ecosystem
- [`docs/WORLD_MODEL.md`](./docs/WORLD_MODEL.md) — reflecting real-world constraints, conflicts, knowledge, institutions, economics (+ epistemic humility)
- [`docs/DIGITAL_TWIN.md`](./docs/DIGITAL_TWIN.md) — digital twin theory: observe, rehearse, actuate
- [`docs/TRINITIES.md`](./docs/TRINITIES.md) — the recurring three-fold design motif
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
