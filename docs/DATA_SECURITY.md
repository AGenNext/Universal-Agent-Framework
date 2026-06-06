# Data Security

Because UAF is *everything-as-data*, securing the system means securing the data.
Every entry on the workspace MUST be **protected, auditable, and secured**:
encrypted at rest, encrypted in transit, and **three-party signed at each
exchange**. These are normative requirements layered on the
[protocol security model](../spec/PROTOCOL.md#11-security-considerations) and
expressed in terms of the five primitives.

## Requirements

| Property | Requirement |
| --- | --- |
| **Protected** | Entry bodies and artifacts are access-controlled by policy over `ctx` (identity, trust, location/residency). No participant reads what its trust/identity does not permit. |
| **Auditable** | The ordered, signed entry log is the audit trail. Every decision — including gate verdicts and human approvals — is an entry with full `ctx`. The log is tamper-evident (see signing). |
| **Secured** | Zero-trust: every inbound entry is untrusted until a gate clears it; capability-gated dispatch; least privilege per `ctx.identity`. |
| **Encrypted at rest** | The workspace log and the `uaf://` artifact store are encrypted at rest; keys are host/residency-scoped per `ctx.location.residency`. |
| **Encrypted in transit** | All transports (socket, channel, remote) use authenticated encryption (e.g. mTLS); in-process C-ABI exchanges stay within a single trust domain. |
| **Three-party signed** | Every exchange accrues three independent signatures (below) before an entry is durable and honored. |

## Zero trust

UAF assumes **no implicit trust** — *never trust, always verify*:

- **No perimeter trust.** Trust is never granted by network location, domain
  membership, or a prior handshake. Every entry and every access is verified at
  **every** boundary, including within a single domain.
- **Per-request trust.** Trust is **computed** per entry from `ctx.trust` (with
  identity, time, location), never assumed; a [gate](./AGENT_AT_THE_GATES.md) may
  raise or lower it and records the basis.
- **Authenticate and authorize every action.** Three-party signatures are
  verified; capability and [governance](./GOVERNANCE.md) are gated; nothing
  unsigned is honored.
- **Continuous verification.** [Federation](./FEDERATION.md) re-evaluates trust at
  each boundary (a peer's `verified` is not automatically local `verified`); `ctx`
  validity windows expire trust over time.
- **Least privilege & microsegmentation.** Fine-grained grants and layered gates
  isolate every actor; a breach of one is not a breach of the system.

Zero trust is the **posture**; the gates, five primitives, and three-party signing
below are its **implementation**.

## Three-party signing at each exchange

An entry becomes part of the immutable log only when it carries a **3-of-3
attestation**. The three signers map to the three things that must be true for an
entry to be trustworthy — *it is authentic, it was permitted, and it is ordered*:

| # | Signer | Attests | Primitive |
| --- | --- | --- | --- |
| 1 | **Author** (the originating participant) | "I wrote this body/envelope." | Identity |
| 2 | **Gate** (the admitting `x-gate`) | "This passed policy at this boundary." | Trust |
| 3 | **Kernel** (the orchestrator/notary) | "I assigned this `seq` and bound it to the log." | Time |

```
   author ──signs(content)──▶ [Gate ──signs(verdict)──▶] kernel ──signs(seq)──▶ durable log
        1                              2                          3
   (Identity)                       (Trust)                     (Time)
```

These signatures are recorded in `ctx.signatures`, and the set is exactly the
`ctx.trust.basis`. An entry missing any of the three MUST NOT be honored: a
verifier rejects it, and the gap is itself logged. This gives **non-repudiation**
across the three roles — authorship, policy, and ordering can each be proven and
none can be forged by a single party.

### `ctx.signatures` shape

```json
"ctx": {
  "signatures": [
    { "by": "planner-1",   "role": "author", "alg": "ed25519", "sig": "...", "over": "envelope+body" },
    { "by": "gate-admit",  "role": "gate",   "alg": "ed25519", "sig": "...", "over": "envelope+verdict" },
    { "by": "kernel",      "role": "kernel", "alg": "ed25519", "sig": "...", "over": "envelope+seq" }
  ],
  "trust": { "level": "verified", "score": 1.0, "basis": ["author", "gate", "kernel"] }
}
```

## Verification rules

1. A participant **MUST** verify all three signatures against the declared keys
   before acting on an entry; a `verified` trust level requires 3-of-3.
2. The kernel signature binds `seq`, so reordering or insertion is detectable —
   the log is **tamper-evident**.
3. Key material is scoped by `ctx.location.residency`; cross-residency exchange
   re-signs at the boundary gate.
4. Signature failures are emitted as `error` entries (themselves signed by the
   detector), preserving auditability of the failure.

## Threat coverage (summary)

| Threat | Mitigation |
| --- | --- |
| Forged authorship | author signature (Identity) |
| Policy bypass | gate signature (Trust) — no entry is durable without an admitting gate |
| Replay / reorder / insertion | kernel signature over `seq` + `ctx.time` validity windows |
| Eavesdropping | encryption in transit |
| Data exfiltration at rest | encryption at rest, residency-scoped keys |
| Over-broad access | least privilege over `ctx.identity` + capability gating |

> Status: design direction. The signing scheme and `ctx.signatures` extend the
> existing entry model additively; enforcement lives in the kernel and gates.
