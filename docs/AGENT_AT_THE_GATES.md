# Agent at the Gates

*Human at the Edge* puts human judgment at the high-stakes checkpoints. **Agent
at the Gates** is its always-on, at-scale complement: at **every boundary** in
UAF, an agent stands as a **policy-enforcement point (PEP)** that validates,
gates, and guards before anything is admitted or released.

The pairing is deliberate:

> **Agents gate everything, continuously. Humans gate the residual that needs
> judgment.** The gate agents are what make a busy system safe enough that the
> human is asked only the questions that matter.

This operationalizes the protocol's zero-trust stance (§10): every entry is
untrusted until a gate clears it.

## The four gates

| Gate | Boundary | The gate agent enforces |
| --- | --- | --- |
| **Ingress** | `task` + inputs entering a session | source/auth, input schema, content policy, artifact-URI policy |
| **Admission** | any entry appended to the workspace | envelope + body schema validity, provenance/causality (`in_reply_to`), version |
| **Dispatch** | `tool_call` leaving for a tool/operand | capability match, params validation, safety/guardrail checks, mutation classification |
| **Egress** | `result` + artifacts leaving to the client | redaction/DLP, license/provenance, final policy |

## How a gate works

A gate is itself a UAP participant — role `x-gate` — bound to a boundary via the
orchestrator's admission hook. For each entry crossing its boundary it does one
of:

- **pass** — entry proceeds unchanged;
- **block** — emit an `error` (or a `critique` with `verdict: revise`) that
  prevents admission/dispatch and explains why;
- **annotate** — attach a policy verdict the next participant can read.

Because gate decisions are ordinary entries, the workspace log records *why* each
was allowed or denied — provenance for both agent and human gates in one trail.

```
 ingress ─▶ [Agent gate]─▶ workspace ─▶ [Agent gate]─▶ planner … executor
                                                           │
                                          step_dispatch ─▶ [Agent gate: dispatch]
                                                           │   pass            block
                                                           ▼                    │
                                          (mutating tool?) [Human gate] ◀───────┘ escalate
                                                           ▼
                                                        tool_call ─▶ tool/operand
                                                           ▼
                                              result ─▶ [Agent gate: egress] ─▶ client
```

## Division of labor: gates vs. edge

| | Agent at the Gates | Human at the Edge |
| --- | --- | --- |
| Coverage | every entry, every boundary | selected high-stakes checkpoints |
| Latency | inline, automated | human-time, gated |
| Strength | scale, consistency, policy | judgment, accountability |
| Escalation | blocks or **escalates** to a human gate | decides the escalated case |

A dispatch gate that classifies a `tool_call` as *mutating real infrastructure*
**MUST** escalate to the Operation gate (the human) from
[Human at the Edge](./HUMAN_AT_THE_EDGE.md) rather than passing autonomously.

## Relationship to agent-as-operator

When the agent operates tools as Kubernetes operands
([Cloud-Native](./CLOUD_NATIVE.md)), the **dispatch gate is the admission
controller** for that operator: it is where capability gating, safety, and the
mutate-vs-read classification are enforced before the operator acts on the
cluster.

> Status: design direction. The `x-gate` role and admission hooks extend the
> existing protocol; gate verdicts are ordinary entries, so this is policy and
> placement, not a wire-format change.
