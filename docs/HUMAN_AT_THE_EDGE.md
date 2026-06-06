# Human at the Edge

In UAF the human is **not** an external operator poking a black box. The human is
a **first-class protocol participant** positioned at the **edge** — both senses:

1. **At the decision edge** — the human sits at trust and judgment boundaries:
   approving a plan before execution, gating a tool dispatch that touches real
   infrastructure, and accepting (or rejecting) the final result.
2. **At the network edge** — the same participant can run on-device / at the edge
   for privacy, latency, and data residency, since participation is just
   "speak UAP over a transport."

This is the human-in-the-loop counterpart to *agent-as-operator*: the agent
reconciles toward the goal, and the human holds the checkpoints where autonomy
should pause.

## The human as a participant

A human joins via a `hello` with role `x-human` and authors ordinary entries —
no special channel:

| Human acts as | Authors | Effect |
| --- | --- | --- |
| Requester | `task` | Starts the session, supplies inputs. |
| Reviewer (reflector) | `critique` (`accept`/`revise`) | Gates the cognition plan. |
| Approver | `critique` on a `step_dispatch` | Gates an individual tool operation. |
| Acceptor | `critique` on the `result` | Final sign-off. |

Because these are the same entries agents use, the workspace log captures human
decisions identically — full, replayable provenance of *who approved what*.

## Approval gates (configurable checkpoints)

A host declares where human consent is required. Suggested defaults:

| Gate | When it fires | Default |
| --- | --- | --- |
| **Plan gate** | before `phase_transition` to deliberation | optional |
| **Operation gate** | before a `tool_call` whose tool mutates real state (the operator dispatching infra) | **required** |
| **Result gate** | before `result` is returned to the client | optional |

At a required gate the orchestrator **MUST** hold the transition until a human
`critique` with `verdict: accept` appears (or a configured timeout / fallback
policy triggers). This is how *agent-as-operator* stays safe: autonomous
reconciliation, human-gated mutation.

```
   planner.plan ─▶ [Plan gate] ─▶ executor
                      ▲
              x-human.critique(accept)

   executor.step_dispatch ─▶ [Operation gate] ─▶ tool_call (operate tool / infra)
                                  ▲
                          x-human.critique(accept)   ← required for mutating tools
```

## Edge deployment

The human participant is typically the lightest workload and the most
latency/privacy sensitive, so it is a natural fit to run at the edge:

- On-device approval UIs that never ship raw inputs to the cloud.
- Local resolution of sensitive `uaf://` artifacts under a human-controlled
  policy.
- Disconnected operation: the workspace log lets an edge human review and
  approve, then sync decisions back when reconnected.

> Status: design direction. The `x-human` role and gate semantics extend the
> existing protocol (human entries are ordinary entries); gate enforcement is a
> kernel/orchestrator policy, not a wire-format change.
