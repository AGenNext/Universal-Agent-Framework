# Governance: Rules, Guardrails, Constitution, Duties & Rights

UAF makes governance **first-class**. The behavior of every agent, gate, and
domain is bound by an explicit **normative layer** built from five concepts:
**Constitution, Rule, Guardrail, Duty, and Right**. These are not comments or
prose policy — they are **data** (everything-as-data), expressed over the five
[primitives](./DESIGN_PRINCIPLES.md), enforced at [gates](./AGENT_AT_THE_GATES.md),
and audited from the same log as everything else.

This is the **deontic** layer over the [World Model](./WORLD_MODEL.md): where the
World Model says what *is* (constraints, resources, facts), the normative layer
says what *ought* to be — what is obligatory, permitted, and forbidden.

## The five concepts

| Concept | What it is | Deontic force | Enforced / checked by |
| --- | --- | --- | --- |
| **Constitution** | a domain's supreme, versioned ruleset; declares rights and duties; amendable only by a governed process | supreme — overrides all lower norms | gates (binding clauses) + agent conditioning (principles) |
| **Rule** | a declarative, enforceable norm | obligation \| permission \| prohibition | gate policy + reflector |
| **Guardrail** | the runtime *mechanism* that enforces a rule, inline and preventive | prevents | gates (admission / dispatch / egress) |
| **Duty** | an obligation bound to a role/identity that MUST be discharged | obligation | reflector / `x-auditor` |
| **Right** | an entitlement a participant holds that others MUST NOT violate | claim / permission | gates protect; violation is blocked |

## The normative stack (precedence)

Norms form a strict precedence order; higher overrides lower, and a gate resolves
conflicts by precedence (residual conflicts go to the `x-arbiter`):

```
   Constitution            (supreme — per domain)
        │
   Institutional rules      (law, regulation, standards — Institutions)
        │
   Domain / org rules        (organization policy)
        │
   Session rules             (task-scoped)
        │
   Plan                      (the planner proposes within all of the above)
```

A plan can never override a rule; a domain rule can never override the
constitution. This is what keeps autonomy bounded.

## Constitution

A constitution is the highest-precedence policy artifact, **bound to a domain**
([Federation](./FEDERATION.md)) — so each nation/org can be constitutionally
sovereign. It plays two distinct roles:

1. **Conditioning (aspirational).** Its principles condition the role-conditioned
   MLLM agents' behavior — the constitutional-AI sense: agents are *prompted to
   reason within* the constitution.
2. **Binding (enforced).** Its machine-checkable clauses compile to **guardrails**
   at the gates — agents cannot *act* outside it regardless of what they reason.

Conditioning shapes intent; binding constrains action. UAF requires both, because
prompting alone is not enforcement.

**Amendment** is itself governed: a change is proposed as an entry, requires a
declared quorum plus [Human at the Edge](./HUMAN_AT_THE_EDGE.md) sign-off, is
versioned, and its diff is auditable. The constitution evolves only through its
own process — recorded as data.

## Rules vs. Guardrails — norm vs. mechanism

A **Rule** is the *what*; a **Guardrail** is the *how*.

- Every binding rule compiles to one or more guardrails at the relevant gate.
- A rule without a guardrail is unenforced intent; a guardrail without a rule is
  arbitrary obstruction. UAF couples them.
- **Preventive** (guardrail blocks before the action) is preferred for
  prohibitions; **detective** (an `x-auditor` flags after the fact) backstops it.

## Duties & Rights — correlative by construction

Following the Hohfeldian model, **a Right in one party implies a Duty in
another**. UAF encodes both as data and ties them together:

| Right (held by) | Correlative Duty (owed by) |
| --- | --- |
| Human's right to review/consent | orchestrator's duty to hold at the gate until approval |
| Right to explanation | every agent's duty to record provenance/rationale |
| Data-subject privacy right | egress gate's duty to redact / honor residency |
| Domain sovereignty right | federation gate's duty to enforce jurisdiction |
| (operand) safety right | agent-operator's duty of care when operating tools |

- **Duties** are assigned by role/identity, discharged within a Time window, and
  their discharge is tracked as data. An unmet duty is a violation that escalates.
- **Rights** are protected at gates: any action that would violate a right is
  **blocked**, and the block is a signed `error` entry — the right is enforced,
  not merely asserted.

## Enforcement and audit

- **Gates** enforce constitution/rules as guardrails (preventive).
- **Reflector / `x-auditor`** verify duties were discharged and rights respected
  (detective), forcing revisions or raising violations.
- Every norm, every check, every violation is **data with provenance** — the
  governance trail is the same replayable, three-party-signed log
  ([Data Security](./DATA_SECURITY.md)) as the rest of the system.

## How it fits the rest of UAF

| This layer | Builds on |
| --- | --- |
| Guardrails | [Agent at the Gates](./AGENT_AT_THE_GATES.md) — guardrails *are* gate policy |
| Rights of the human | [Human at the Edge](./HUMAN_AT_THE_EDGE.md) — consent/review as enforced rights |
| Rules as constraints | [World Model](./WORLD_MODEL.md) — rules add deontic modality to constraints |
| Sovereign constitution | [Federation](./FEDERATION.md) — a constitution is bound per domain |
| Everything as norms-as-data | [Design Principles](./DESIGN_PRINCIPLES.md) — norms are entries over the core primitives |

> Status: design direction. Constitution, rules, duties, and rights are typed
> records (content + policy), and guardrails are gate enforcement; this adds the
> `x-auditor` role and deontic modality on rule records without changing the wire
> envelope.
