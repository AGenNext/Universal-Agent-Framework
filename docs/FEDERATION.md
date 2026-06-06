# Federation — a Multi-Nation, Multi-Device, Multi-Agent, Multi-Model Ecosystem

UAF is designed to scale past a single deployment into a **federated ecosystem**:
many independent UAF **domains** — nations, organizations, clouds, devices — that
interoperate through the [protocol](../spec/PROTOCOL.md) without any central
authority. Federation is not a new protocol; it is the existing protocol plus
**boundary gates** that enforce sovereignty using the five primitives.

## What federates

| Axis | Meaning |
| --- | --- |
| **Federated** | Each domain runs its own kernel and owns its own workspace; domains exchange entries peer-to-peer. No global control plane, no shared single source of truth — trust is established pairwise. |
| **Multi-nation** | Domains map to jurisdictions. Data residency, lawful-basis, and cross-border rules are enforced at the boundary via `ctx.location.residency` and `ctx.identity`. |
| **Multi-device** | Devices (phones, kiosks, vehicles, sensors) are first-class edge participants; a session can span devices and survive disconnection, reconciling via the replayable log. |
| **Multi-agent** | An open population of agents from many vendors/orgs collaborates — the canonical catalog plus framework adapters make them interoperable. |
| **Multi-model** | Each domain/agent chooses its own MLLM/diffusion models; models are never assumed global. |

## The domain boundary

A **domain** is one kernel + its workspace + its trust root. Domains connect
through a **federation gate** — a specialization of
[Agent at the Gates](./AGENT_AT_THE_GATES.md) that sits on the boundary between
two domains and governs every entry crossing it.

```
        Domain A (nation/org)                         Domain B (nation/org)
   ┌──────────────────────────┐                 ┌──────────────────────────┐
   │ kernel · workspace · gates│  ⇄  federation  │ kernel · workspace · gates│
   │ agents · tools · models   │     gate (mTLS) │ agents · tools · models   │
   └──────────────────────────┘                 └──────────────────────────┘
        residency: EU                                  residency: US
```

A federation gate, for each cross-domain entry, MUST:

1. **Authenticate the peer domain** (federated identity, e.g. SPIFFE/mTLS), and
   verify the entry's three-party signatures from the origin domain.
2. **Enforce residency & jurisdiction** — apply `ctx.location.residency` rules;
   block or redact entries/artifacts that may not cross.
3. **Re-establish trust** — map the origin domain's `ctx.trust` into the local
   trust frame (a peer's `verified` is not automatically local `verified`).
4. **Re-sign at the boundary** — add a fourth, boundary signature so the
   importing domain has local non-repudiation. The origin's 3-of-3 plus the
   boundary signature gives a verifiable cross-domain chain of custody.

## Sovereignty via the five primitives

Federation needs no new vocabulary — it is policy over the existing
[primitives](./DESIGN_PRINCIPLES.md):

| Primitive | Federation role |
| --- | --- |
| **Identity** | federated/peer identity; which domain vouches for an actor |
| **Location** | jurisdiction, residency, where an entry/artifact may travel |
| **Trust** | per-domain trust frames; trust is re-evaluated at each boundary |
| **Time** | validity windows on cross-border grants; expiry of federation tokens |
| **Relation** | delegation across domains; org/treaty structure between peers |

## Multi-device sessions

Because participation is "speak UAP over a transport," a device joins a session
like any participant — typically over the edge transport with a local, possibly
partial replica of the workspace.

- **Locality** — sensitive inputs resolve to `uaf://` artifacts under the
  device's residency; raw data need not leave the device.
- **Disconnected operation** — a device records human approvals
  ([Human at the Edge](./HUMAN_AT_THE_EDGE.md)) and observations offline, then
  syncs the signed entries when reconnected; ordering reconciles via the kernel.
- **Hand-off** — a session can migrate across devices by moving its log; the
  signatures make the migrated history verifiable.

## Ecosystem properties

| Property | How federation provides it |
| --- | --- |
| No lock-in | every domain interchangeable behind UAP; cloud-agnostic by construction |
| Sovereignty | residency/jurisdiction enforced at boundary gates, not by a central operator |
| Interop | canonical agents + framework adapters let heterogeneous agents/models collaborate |
| Accountability | cross-domain chain of custody via origin 3-of-3 + boundary re-signing |
| Resilience | no central plane to fail; domains and devices degrade independently |

> Status: design direction. Federation gates, peer identity, and boundary
> re-signing extend the gate and security models additively; the wire entry
> format is unchanged.
