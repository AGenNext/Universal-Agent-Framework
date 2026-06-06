# Universal Agent Protocol (UAP)

**Version:** 0.1 (draft) · **Status:** normative for UAF · **Encoding:** JSON
(UTF-8) · **Transport:** agnostic

The Universal Agent Protocol defines how independent participants — agents,
tools, and the kernel — collaborate to perform **universal multimodal
understanding and generation**. It is the protocol-first foundation of the
Universal Agent Framework and a generalization of the MAGUS design
([arXiv:2508.10494](https://arxiv.org/abs/2508.10494)).

The keywords **MUST**, **MUST NOT**, **SHOULD**, **MAY** are to be interpreted
as in RFC 2119.

---

## 1. Design principles

1. **Coordinate by sharing, not by calling.** Participants never invoke each
   other directly. They append **entries** to a shared, ordered **workspace**.
   This is the blackboard model and the only coordination mechanism.
2. **The workspace is the source of truth.** All state — observations, plans,
   critiques, artifacts, results — lives as entries. A session can be fully
   reconstructed by replaying its workspace.
2a. **Everything as data.** There are no out-of-band control channels. Tasks,
   plans, policies, tool capabilities, phase transitions, trust verdicts, even
   human approvals are all **entries**. Control flow *is* data on the workspace,
   so the entire system is inspectable, replayable, and auditable as one log.
3. **Two decoupled phases.** Every task moves through **Cognition** (understand
   + plan) and **Deliberation** (execute), per MAGUS.
4. **Roles are conditioning, not classes.** An agent's role (`perceiver`,
   `planner`, …) describes how it behaves, not a privileged implementation.
5. **Transport- and language-agnostic.** Any process that can read and append
   JSON entries — in-process via the kernel C ABI, or out-of-process via
   stdio/socket/WASM — is a conformant participant.

---

## 2. Core objects

### 2.1 Session

A session is one task lifecycle. It has a UUID, a current **phase**, and a
monotonically increasing entry sequence (`seq`). Sessions are created by the
kernel on receipt of a `task` entry from a `client`.

### 2.2 Participant

A process bound to the session. Declared by a `hello` entry:

| Field | Type | Notes |
| --- | --- | --- |
| `id` | string | Unique within the session. |
| `role` | enum | One of the reserved roles (§4) or `x-*` for extensions. |
| `capabilities` | object | For `tool` participants, the tool descriptor (§6). |

### 2.3 Workspace

An append-only, totally-ordered log of **entries**. The kernel assigns `seq`.
Participants subscribe to the log and react to entries relevant to their role.

### 2.4 Artifact

A produced or supplied multimodal object, referenced rather than inlined:

```json
{ "modality": "image", "uri": "uaf://artifact/9f3...", "mime": "image/png", "meta": {} }
```

`modality` ∈ `text | image | audio | video`. `uri` MAY be a `uaf://` handle
managed by the kernel's artifact store, or any resolvable URI.

---

## 3. The entry envelope

Every message on the workspace is an **entry**. All entries share this envelope;
`kind` selects the `body` schema.

```json
{
  "uap": "0.1",
  "id": "5b1c2e7a-...",
  "session": "0e9d...",
  "seq": 12,
  "ts": "2026-06-06T18:20:00Z",
  "phase": "cognition",
  "from": "planner-1",
  "role": "planner",
  "kind": "plan",
  "in_reply_to": ["b3a0-..."],
  "modalities": ["text"],
  "body": { }
}
```

| Field | Req | Description |
| --- | --- | --- |
| `uap` | ✓ | Protocol version. |
| `id` | ✓ | Entry UUID (participant-generated). |
| `session` | ✓ | Session UUID. |
| `seq` | kernel | Monotonic order; assigned by the kernel, absent on submission. |
| `ts` | ✓ | RFC 3339 timestamp. |
| `phase` | ✓ | `cognition` or `deliberation` at time of authoring. |
| `from` | ✓ | Authoring participant `id`. |
| `role` | ✓ | Authoring role. |
| `kind` | ✓ | Entry kind (§5). |
| `in_reply_to` | – | Causal links to prior entry `id`s. |
| `modalities` | – | Modalities referenced by `body`. |
| `body` | ✓ | Kind-specific payload. |

The normative envelope schema is [`schemas/entry.schema.json`](./schemas/entry.schema.json).

---

## 4. Reserved roles

| Role | Phase | Responsibility |
| --- | --- | --- |
| `client` | — | Submits the `task`; receives the `result`. |
| `orchestrator` | both | Manages phase transitions and round limits. Usually the kernel. |
| `perceiver` | cognition | Grounds all input modalities into structured text. |
| `planner` | cognition | Produces an ordered, tool-annotated plan. |
| `reflector` | both | Critiques observations/plans; may verify deliberation output. |
| `executor` | deliberation | Walks the approved plan, dispatching steps. |
| `tool` | deliberation | Provides an understanding/generation capability. |

Custom roles MUST be prefixed `x-`.

---

## 5. Entry kinds

| `kind` | From → | Meaning |
| --- | --- | --- |
| `hello` | any | Join the session; declare role/capabilities. |
| `task` | client | The request, with input artifacts and goal. |
| `observation` | perceiver | Structured grounding of inputs. |
| `plan` | planner | Ordered steps (§ plan body). |
| `critique` | reflector | Verdict (`accept`/`revise`) + notes. |
| `revision` | planner | Updated plan addressing a critique. |
| `phase_transition` | orchestrator | Move `cognition → deliberation` (or abort). |
| `step_dispatch` | executor | Assign a plan step to a tool. |
| `tool_call` | executor | Concrete invocation of a tool. |
| `tool_result` | tool | Output of a `tool_call` (often an artifact). |
| `artifact` | any | Register/announce an artifact in the store. |
| `result` | executor | Final deliverable to the client. |
| `error` | any | Recoverable or fatal error (`fatal` flag). |

### Plan body

```json
{
  "goal": "Describe the image, then synthesize a matching caption card.",
  "steps": [
    {
      "id": "s1",
      "type": "understand",
      "modality": "image",
      "instruction": "Caption the input image.",
      "tool": null,
      "depends_on": [],
      "inputs": [{ "modality": "image", "uri": "uaf://artifact/in-0" }]
    },
    {
      "id": "s2",
      "type": "generate",
      "modality": "image",
      "instruction": "Render a caption card using the s1 text.",
      "tool": "image.diffusion.v1",
      "depends_on": ["s1"],
      "inputs": []
    }
  ]
}
```

Step `type` ∈ `understand | generate`. `understand` steps are answered by an
MLLM participant; `generate` steps name a `tool`. See
[`schemas/plan.schema.json`](./schemas/plan.schema.json).

---

## 6. Tool capability descriptor

A `tool` participant advertises capabilities in its `hello`:

```json
{
  "tool": "image.diffusion.v1",
  "kind": "generate",
  "modalities_in": ["text"],
  "modalities_out": ["image"],
  "params_schema": { "type": "object", "properties": { "steps": { "type": "integer" } } }
}
```

The executor MUST only dispatch a step to a tool whose `modalities_out`
satisfies the step and whose `params` validate against `params_schema`. See
[`schemas/tool.schema.json`](./schemas/tool.schema.json).

---

## 7. Phase state machine

```
        task
         │
         ▼
   ┌───────────┐   observation/plan/critique loop (≤ max_rounds)
   │ COGNITION │◀──────────────────────────────┐
   └───────────┘                                │
         │  reflector verdict = accept          │ verdict = revise → revision
         ▼                                      │
   phase_transition ─────────────────────────────
         │
         ▼
   ┌──────────────┐   step_dispatch → tool_call → tool_result → artifact   (per step, honoring depends_on)
   │ DELIBERATION │◀──────────────────────────────────────────────────────┐
   └──────────────┘   optional reflector verification → revise step ───────┘
         │  all steps complete
         ▼
       result  →  session complete
```

**Cognition.** After `task`, the perceiver emits an `observation`; the planner
emits a `plan`; the reflector emits a `critique`. On `revise`, the planner emits
a `revision` and the loop repeats up to `max_rounds`. On `accept` (or the
orchestrator forcing it at the limit), the orchestrator emits a
`phase_transition` to `deliberation`.

**Deliberation.** The executor walks the plan in dependency order. For each
step it emits `step_dispatch` then a `tool_call` (or queries an MLLM for
`understand` steps); the tool replies with `tool_result` and registers an
`artifact`. The reflector MAY verify a result and request a redo. When every
step is satisfied, the executor emits `result` and the session completes.

---

## 8. Conformance

Three levels, so minimal hosts and full systems can both claim conformance:

| Level | Requirement |
| --- | --- |
| **L0 — Cognition** | Run the cognition loop to an accepted plan; emit text-only `result`. |
| **L1 — Single-modality generation** | L0 + dispatch `generate` steps to ≥1 tool of one non-text modality. |
| **L2 — Any-to-any** | L1 + all four modalities + reflective verification in deliberation. |

A conformant participant MUST reject entries with an unknown `uap` major
version and MUST ignore (not error on) unknown optional envelope fields, to keep
the protocol forward-compatible. See [`conformance.md`](./conformance.md).

---

## 9. Versioning

`uap` is `MAJOR.MINOR`. Additive, backward-compatible changes bump MINOR.
Breaking changes bump MAJOR; participants MUST refuse mismatched MAJOR versions.

---

## 10. Core primitives — the context block

Every entry MAY carry a `ctx` object that makes five cross-cutting concerns
**first-class primitives** rather than ad-hoc metadata. Gates
([Agent at the Gates](../docs/AGENT_AT_THE_GATES.md)) and the kernel read `ctx`
to make admission, routing, and policy decisions; the human edge
([Human at the Edge](../docs/HUMAN_AT_THE_EDGE.md)) reads it to decide what needs
review.

```json
"ctx": {
  "identity": { "actor": "planner-1", "credential": "spiffe://...", "signature": "ed25519:..." },
  "time":     { "valid_from": "2026-06-06T18:20:00Z", "valid_until": "2026-06-06T18:25:00Z", "logical": 12 },
  "location": { "zone": "edge", "region": "eu-west", "device": "kiosk-7", "residency": "EU" },
  "trust":    { "level": "attested", "score": 0.92, "basis": ["signature", "gate:admission"] },
  "relation": { "delegated_by": "client-0", "depends_on": ["s1"], "org": "acme" }
}
```

| Primitive | Question | Drives |
| --- | --- | --- |
| **Identity** | *Who* authored this? | authentication, signing, attribution; the gate's ingress check |
| **Time** | *When* is this valid? | ordering, causality, validity windows, expiry of approvals/credentials |
| **Location** | *Where* did it originate / may it go? | edge-vs-cloud routing, data residency, locality of artifacts |
| **Trust** | *How much* do we believe it? | gating thresholds, escalation to a human, capability eligibility |
| **Relation** | *How* does it relate to other entries/actors? | causality, delegation chains, org/social structure |

Rules:

- `ctx` is **additive and optional**; absence means "unspecified," and a host's
  policy decides the default (e.g. `trust.level = "anonymous"`).
- `ctx.time` refines, and MUST be consistent with, the envelope `ts`/`seq`.
- `ctx.relation.depends_on` / `delegated_by` complement envelope `in_reply_to`
  (causality) with semantic relationships.
- A gate MAY raise or lower `ctx.trust` and MUST record the basis; downstream
  participants read the updated value.

These five primitives are the vocabulary the security model below is expressed
in. See [`../docs/DESIGN_PRINCIPLES.md`](../docs/DESIGN_PRINCIPLES.md).

## 11. Security considerations

- Entries cross trust boundaries (out-of-process / remote participants). A host
  MUST treat all `body` content — especially `task`, `tool_result`, and
  artifacts — as untrusted input and validate against the schemas before acting.
- Tool dispatch MUST be capability-gated: the kernel only routes `tool_call`s to
  tools that advertised the matching capability via `hello`.
- Artifact `uri`s MUST be resolved through a policy the host controls; a host
  SHOULD NOT auto-fetch arbitrary URIs.

### Data protection (normative)

All entries and artifacts MUST be **protected, auditable, secured, encrypted at
rest, and encrypted in transit**, and MUST be **three-party signed at each
exchange**:

| # | Signer | Attests | Primitive |
| --- | --- | --- | --- |
| 1 | Author | authorship of envelope + body | Identity |
| 2 | Gate | policy admission at the boundary | Trust |
| 3 | Kernel | the assigned `seq` / ordering | Time |

An entry is durable and honored only with this **3-of-3 attestation**, recorded
in `ctx.signatures` (and reflected in `ctx.trust.basis`). A verifier MUST reject
entries missing any signature, making the log tamper-evident and giving
non-repudiation across authorship, policy, and ordering. Full requirements:
[`../docs/DATA_SECURITY.md`](../docs/DATA_SECURITY.md).

When an entry crosses a domain boundary in a federated deployment, a federation
gate adds a fourth `boundary` signature after verifying the origin's 3-of-3,
giving a verifiable cross-domain chain of custody. See
[`../docs/FEDERATION.md`](../docs/FEDERATION.md).
