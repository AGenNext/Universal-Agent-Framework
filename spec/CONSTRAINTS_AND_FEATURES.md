# Constraints and Features

**Status:** draft normative extension for UAP 0.1  
**Goal:** grow UAF without growing the kernel.

UAF adds capability by adding protocol vocabulary, schemas, constraints, policies,
and conformance examples — not by adding new runtime services. The kernel remains
small. Intelligence, governance, identity, safety, cost, approval, federation,
and product behavior are expressed as data on the workspace.

## Rule of growth

A UAF feature is complete when it has:

1. a protocol entry kind or body shape,
2. a JSON Schema,
3. a policy or conformance rule,
4. an example session showing the feature in the workspace log.

A feature SHOULD NOT require a new service, framework, scheduler, datastore, or
agent class unless the protocol cannot represent it as data.

```text
Feature = schema + entry kind + policy rule + conformance example
Not     = new service + new framework + new runtime
```

## Minimal kernel contract

The kernel only needs to understand the stable primitives:

- entry
- workspace
- phase
- context (`ctx`)
- capability
- constraint
- policy verdict
- artifact
- result

Everything else is expressed inside entries and interpreted by participants,
adapters, gates, or policy engines.

## New entry kinds

UAP 0.1 may be extended with the following data-first entry kinds:

| Kind | Meaning |
| --- | --- |
| `constraint` | Declares a rule that limits or guides a task, plan, tool call, artifact, or result. |
| `feature` | Declares an available protocol feature and the entries/capabilities it requires. |
| `policy_verdict` | Records a gate decision over a constraint, action, plan, or capability. |
| `approval` | Records human or delegated approval. |
| `rejection` | Records human or delegated rejection. |

The kernel MUST NOT hard-code business meaning for every constraint or feature.
It only needs to enforce whether a required verdict exists before a gated action
continues.

## Constraint model

A constraint is an entry whose body declares scope, type, rule, and severity.

```json
{
  "kind": "constraint",
  "body": {
    "id": "cost.max_total",
    "scope": "session",
    "type": "cost",
    "rule": "max_total_usd <= 1.00",
    "severity": "block",
    "applies_to": ["tool_call", "step_dispatch"]
  }
}
```

### Constraint fields

| Field | Required | Meaning |
| --- | --- | --- |
| `id` | yes | Stable identifier for the constraint. |
| `scope` | yes | Scope where the rule applies. |
| `type` | yes | Constraint category. |
| `rule` | yes | Human-readable or policy-engine-readable rule. |
| `severity` | yes | `block`, `warn`, or `record`. |
| `applies_to` | no | Entry kinds or operations covered by the constraint. |
| `reason` | no | Why the constraint exists. |
| `owner` | no | Identity or role responsible for the rule. |

### Constraint types

Reserved constraint types:

- `cost`
- `time`
- `security`
- `privacy`
- `location`
- `jurisdiction`
- `human_approval`
- `quality`
- `safety`
- `license`
- `model`
- `tool`
- `data_residency`
- `reversibility`
- `auditability`
- `explainability`

Custom constraint types MUST be prefixed with `x-`.

## Feature model

A feature is declared as protocol data. It says what the system can do and what
entries, capabilities, or constraints are required for that behavior.

```json
{
  "kind": "feature",
  "body": {
    "id": "human.approval.v1",
    "title": "Human approval gate",
    "requires": ["identity", "capability", "signature"],
    "emits": ["approval", "rejection", "policy_verdict"],
    "status": "available"
  }
}
```

Reserved feature categories:

- human approval
- budget gate
- policy gate
- tool permission
- model routing
- artifact signing
- memory read/write
- replay
- rollback simulation
- federation boundary
- identity verification
- trust scoring

A feature declaration does not execute anything by itself. It makes a capability
visible and testable in the workspace.

## Policy verdict model

A policy verdict records the result of evaluating a constraint, action, plan,
capability, or boundary crossing.

```json
{
  "kind": "policy_verdict",
  "body": {
    "target": "entry:8fa3...",
    "constraint": "cost.max_total",
    "verdict": "allow",
    "reason": "Estimated cost is below the session budget.",
    "evaluator": "gate.cost.v1"
  }
}
```

Allowed verdicts:

- `allow`
- `deny`
- `revise`
- `needs_human`

A blocking constraint MUST have an `allow` verdict before the covered action
continues. A `needs_human` verdict MUST be followed by either `approval` or
`rejection`.

## Approval and rejection

Approvals and rejections are entries, not side channels.

```json
{
  "kind": "approval",
  "body": {
    "target": "entry:8fa3...",
    "approved_by": "did:uaf:human:admin-1",
    "reason": "The tool call is within budget and uses approved data.",
    "expires": "2026-06-05T20:30:00Z"
  }
}
```

The approval itself SHOULD carry `ctx.identity`, `ctx.capability`, and
`ctx.signatures` so the decision is attributable and replayable.

## Conformance rules

A conformant UAF implementation:

1. MUST preserve all constraint, feature, policy verdict, approval, and rejection
   entries in the workspace log.
2. MUST reject malformed entries that do not match their schemas.
3. MUST NOT continue a gated action when a blocking constraint has no `allow`
   verdict or valid approval.
4. MUST make replay produce the same effective gate decisions from the same log.
5. SHOULD allow external policy engines to append `policy_verdict` entries rather
   than embedding policy logic into the kernel.

## Design boundary

The kernel owns ordering, validation, phase transitions, and replay.

Participants, adapters, tools, humans, and policy engines own interpretation.

This keeps UAF universal: the framework becomes more capable through better data,
not a larger runtime.