# Actions as Commands

Every action in UAF is a **command** — a reified, first-class instruction
(data), not a side-effecting function call hidden inside an agent. An action is
*issued*, *validated*, *authorized*, *executed*, and *recorded* — and because it is
data, it can be queued, deferred, audited, replayed, and compensated.

This is the command pattern (and the command/event split of CQRS) applied to
agent behavior: it is what makes [agent-as-actor](./AGENT_AS_ACTOR.md) actions
governable and accountable.

## Command vs. event

| | Command | Event |
| --- | --- | --- |
| Expresses | **intent to act** (imperative) | **what happened** (fact) |
| Mutability | may be **rejected / revised** before it takes effect | **immutable** once recorded |
| In UAP | `task`, `step_dispatch`, `tool_call` | `observation`, `tool_result`, `artifact`, `result` |
| Direction | request a change | record a change |

A command is a *proposal to change the world*; the resulting event is the
*immutable record* of the change. The workspace log is the stream of both.

## The command lifecycle

```
   issue ─▶ validate ─▶ authorize ─▶ execute ─▶ record (event) ─▶ audit
            (defined)   (gate +       (operate)   (immutable)
                         capability +
                         governance +
                         sanity check)
```

1. **Issue** — an actor authors the command as an entry, carrying `ctx` (the five
   primitives) and its three-party signature.
2. **Validate** — it must match a published definition (the *defined* axiom); an
   undefined command is rejected at the gate.
3. **Authorize** — the [gate](./AGENT_AT_THE_GATES.md) checks capability, the
   applicable [governance](./GOVERNANCE.md) (rules/rights/duties), residency, and
   the [mandatory sanity check](./DESIGN_PRINCIPLES.md). A mutating command may
   **escalate to the human** ([operation gate](./HUMAN_AT_THE_EDGE.md)).
4. **Execute** — the command runs (e.g. an operator operating a tool operand).
5. **Record** — the outcome is an immutable **event** entry with an artifact
   reference.
6. **Audit** — command + authorization + event form a signed, replayable trail.

## What reification buys

| Property | Because the action is a command (data) |
| --- | --- |
| **Authorizable** | a command can be inspected and gated *before* it acts |
| **Deferrable / queueable** | commands can wait for approval, resources, or dependencies |
| **Reversible** | effects are undone by **compensating commands** (sagas), recorded like any other |
| **Replayable** | re-issuing the command stream reconstructs behavior deterministically |
| **Attributable** | every command is signed and bound to a [UDCI](./IDENTIFIERS.md) |
| **Measurable** | commands are metered (cost/latency/outcome) per the axioms |

## How it fits

- **Agent-as-operator** ([Cloud-Native](./CLOUD_NATIVE.md)): operating a tool is
  *issuing commands* to an operand and reconciling toward desired state; each
  reconciliation step is a command.
- **Everything as data**: commands and events are both entries — one substrate.
- **Everything governed**: no command executes outside authorization; there is no
  ungoverned action path.
- **Uniform access**: `INVOKE` is the verb that issues a command to a resource.

> Status: design direction. Actions-as-commands names the command/event discipline
> over existing UAP entry kinds; it adds no wire-format fields.
