# Cloud-Native, Cloud-Agnostic, Kubernetes-Native

UAF runs as a **cloud-native** system, stays **cloud-agnostic**, and is
**Kubernetes-native** by treating the **agent as both the orchestrator and the
operator of tools**. This document explains how the protocol and kernel map onto
cloud and Kubernetes primitives. The [protocol](../spec/PROTOCOL.md) remains the
contract; this is a deployment topology, not a new protocol.

## 1. Cloud-native

- **Everything is a workload.** The kernel and each participant (perceiver,
  planner, reflector, executor, tools) package as containers and scale
  independently. Generation tools — the heavy, GPU-bound parts — scale on their
  own axis from the lightweight cognition agents.
- **Stateless compute, stateful log.** Participants are stateless; all session
  state is the kernel's replayable workspace log. This is what makes restarts,
  rescheduling, and horizontal scale safe — a recovering participant simply
  replays the log.
- **Observability for free.** The ordered entry log *is* the audit trail and
  trace. Every decision (observation, plan, critique, tool call) is a structured
  event, so tracing/metrics/replay come from the protocol, not bolted on.

## 2. Cloud-agnostic

- **No vendor primitives in the core.** The kernel depends on no cloud-specific
  service; transports (stdio / socket / channel / WASM) and the artifact store
  are pluggable behind interfaces a host supplies.
- **Provider-injected models.** Per the canonical-agent contract, MLLM and
  diffusion backends are injected, never hard-wired — so the same agents run on
  any cloud, on-prem, or at the edge.
- **Portable artifacts.** `uaf://` handles resolve through a host-controlled
  policy, which can back onto S3, GCS, Azure Blob, or local disk identically.

## 3. Kubernetes-native — the agent as operator

UAF adopts the **Kubernetes Operator pattern** directly: the agent is a control
loop that **reconciles desired state**. The MAGUS plan is the desired state; the
executor reconciles the cluster toward it.

### Custom Resources (proposed)

| CRD | Represents | Reconciled by |
| --- | --- | --- |
| `Session` | one task lifecycle (the workspace) | kernel controller |
| `Agent` | a role-conditioned participant (perceiver/planner/reflector/executor) | agent controller |
| `Tool` | a generation/understanding capability + its scaling policy | tool operator |
| `Plan` | the approved plan as declarative desired state | executor (operator) |

### Agent as orchestrator AND operator of tools

- **Orchestrator:** the agent drives the workspace and phase machine — it is the
  control plane for the session, sequencing perception, planning, reflection,
  and execution.
- **Operator of tools:** tools are **operands**. The agent does not just *call* a
  tool; it **operates** it — provisioning the `Tool` workload, gating dispatch by
  advertised capability, scaling it for the step, observing health, and tearing
  it down. A `generate` step becomes a reconciliation: *desired output exists →
  ensure the tool is running → dispatch → record artifact → converge.*

```
        Plan (desired state)
              │  reconcile
              ▼
   ┌──────────────────────┐      ensures/scales      ┌───────────────┐
   │  Agent = operator     │ ───────────────────────▶ │  Tool operand │  (GPU diffusion, TTS, …)
   │  (control loop)       │ ◀─────────────────────── │  workload     │
   └──────────────────────┘    tool_result/artifact   └───────────────┘
              │ converged
              ▼
           result
```

### Why the operator pattern fits

A MAGUS deliberation loop *is* a reconciliation loop: observe current artifacts,
compare to the plan's goal, act (dispatch a tool), observe again, repeat until
converged — with the reflector as the verification step. Kubernetes already
gives us the machinery (controllers, CRDs, leader election, backoff), so the
agent inherits durability, self-healing, and horizontal scale.

## 4. Topology (illustrative)

```
            ┌───────────────────────── Kubernetes cluster ─────────────────────────┐
            │  kernel Deployment ── Session CRD ── workspace log (durable)          │
            │        │                                                              │
            │   cognition agents (Deployments, autoscaled on CPU)                   │
            │        │                                                              │
            │   executor (operator) ── reconciles Plan ──▶ Tool operands           │
            │                                   │                                    │
            │                         GPU node pool: diffusion / tts / video tools  │
            └───────────────────────────────────────────────────────────────────────┘
                 cloud-agnostic: any conformant K8s, any cloud, on-prem, or edge
```

> Status: design direction. CRDs and controllers are proposed here and will be
> specified alongside the kernel implementation; nothing here changes the wire
> protocol.
