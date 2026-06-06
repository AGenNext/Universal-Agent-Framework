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

## No single point of failure

UAF is designed so that **no single component's failure halts the system** —
resilience by redundancy at every level:

- **No central authority.** [Federation](./FEDERATION.md) is peer-to-peer; there is
  no global control plane to lose. Domains, devices, and tools **degrade
  independently** — an edge node keeps operating disconnected and resyncs later.
- **The kernel is not a singleton.** Within a domain the kernel is **replicated and
  leader-elected** (or sharded by session). It holds no irreplaceable state: the
  durable, signed [journal](./DURABLE_EXECUTION.md) is the source of truth, so any
  replica can rebuild and continue.
- **Stateless, supervised participants.** Any actor
  ([agent-as-actor](./AGENT_AS_ACTOR.md)) can crash and be rescheduled; work resumes
  by replay ([durable execution](./DURABLE_EXECUTION.md)) — no progress is lost.
- **Replicated journal and artifacts.** The workspace log and the content-addressed
  artifact store are replicated (multi-node / multi-AZ / multi-device); a
  content-addressed artifact is fetchable and verifiable from *any* replica.
- **Distributed enforcement.** Gates and governance run at **each** boundary, not a
  central policy server — enforcement has no chokepoint either.

Redundancy + replay + federation = **continuous availability**: components fail,
the system does not.

## Every node accessible and connected — multi-channel

Connectivity itself must not be a single point of failure. Every node (kernel
replica, actor, tool, device, domain peer) is **reachable over multiple channels**
and connected through **redundant paths**:

- **Transport redundancy.** A node is reachable via several transports at once —
  in-process C ABI, stdio, local socket, WebSocket/HTTP, message channel, or a
  federated link. If one channel drops, another carries the entries.
- **Channel-independent addressing.** A node is named by its
  [UDCI](./IDENTIFIERS.md) and reached via [uniform access](./ACCESS.md); the
  identifier is decoupled from the channel, so resolution picks whatever path is
  live — multipath with failover.
- **Mesh, not hub.** Nodes connect peer-to-peer across
  [domains and devices](./FEDERATION.md); there is no mandatory central relay whose
  loss isolates a node.
- **Disconnection-tolerant.** When no channel is available, the durable log is
  store-and-forward: a node queues signed entries locally and syncs when any channel
  returns ([durable execution](./DURABLE_EXECUTION.md)).
- **Uniformly secured.** Every channel is zero-trust and encrypted in transit;
  redundancy adds *paths*, not *trust* — each path is independently authenticated and
  signed.

### The tunnel is gated

Zero trust applies to the **connection itself**, not only the entries on it. Every
channel/tunnel is a [gated](./AGENT_AT_THE_GATES.md) boundary:

- **Connecting is a gated act.** Establishing a tunnel requires mutual
  [UDCI](./IDENTIFIERS.md) authentication, a capability *to connect*, and a
  residency/jurisdiction check; the tunnel carries a [JIT, time-bound](./DATA_SECURITY.md)
  credential that expires and can be revoked mid-session.
- **No ungated path.** There is no backdoor, side channel, or out-of-band tunnel —
  redundancy adds *gated* paths only. An ungated connection is, by definition, not
  part of the system.
- **Defense in depth persists.** Beyond connection-time gating, every entry on the
  tunnel still passes the admission / dispatch / egress gates. **Connecting grants
  reach, not trust.**

> Status: design direction. CRDs and controllers are proposed here and will be
> specified alongside the kernel implementation; nothing here changes the wire
> protocol.
