# Uniform Access Protocol

UAF has two complementary protocols:

- the **Universal Agent Protocol (UAP)** — how participants *coordinate* (entries
  on the shared workspace, phases); and
- the **Uniform Access Protocol** — how participants *access* anything in the
  system: one consistent interface to **resolve, read, write, query, subscribe,
  and invoke** every entity named by a [UDCI](./IDENTIFIERS.md).

> UAP is the common **language**; the Uniform Access Protocol is the common
> **interface**. You say things in UAP; you reach things uniformly.

This is the REST "uniform interface" constraint applied to an entire agent
ecosystem: instead of N bespoke APIs (one for artifacts, one for tools, one for
the model registry, one for the world model…), there is **one** way to access
**everything**.

## Everything is a uniformly-addressed resource

Every UDCI-named entity is a **resource** reached the same way:

| Resource | Example identifier |
| --- | --- |
| Artifact | `uaf://bafy...` |
| Definition / schema | `uaf://bafk...@1.2.0` |
| Ontology concept / world record | `uaf://...` |
| Norm (rule / constitution) | `uaf://...` |
| Tool / model | `did:uaf:acme:image.diffusion.v1` |
| Agent | `did:uaf:key:z6Mk...` |
| Session | `did:uaf:session:...` |

One address space (UDCI), one interface — below.

## The uniform interface

| Verb | Meaning | Expressed in UAP as |
| --- | --- | --- |
| **RESOLVE** | identifier → document (keys, endpoints, capabilities, metadata) | identity/capability resolution |
| **READ** | fetch content at a (pinned) version | artifact fetch |
| **WRITE** | store content → mints a new content-addressed version | `artifact` registration |
| **QUERY / LIST** | select over the world model / registry by predicate on the five primitives | a query entry |
| **SUBSCRIBE** | stream entries/changes matching a selector | workspace subscription |
| **INVOKE** | call a capability of a tool/agent resource | `tool_call` → `tool_result` |

Because access operations are themselves **entries** (everything-as-data), they
inherit the whole stack — they are gated, signed, audited, and replayable like any
other entry. Access is not a side channel.

## Uniform semantics — the same everywhere

Whatever the resource, access obeys identical rules:

- **Addressed by UDCI** — one decentralized, canonical name space.
- **Capability-gated** — every access passes a [gate](./AGENT_AT_THE_GATES.md);
  one enforcement model, not per-API auth.
- **Primitive-aware** — every access carries `ctx` (Trust, Time, Location,
  Identity, Relation); policy is the same predicate language everywhere.
- **Signed & audited** — three-party signing
  ([Data Security](./DATA_SECURITY.md)) on access as on everything else.
- **Versioned** — READ pins a version (content-addressed); WRITE mints one; no
  in-place mutation (the publish/version/sign [axiom](./AXIOMS.md)).
- **Federation-aware** — cross-domain access flows through boundary gates with
  residency enforcement ([Federation](./FEDERATION.md)); same verbs, sovereign
  boundaries.
- **Transport-agnostic** — identical semantics in-process (C ABI), out-of-process
  (socket/stdio/WASM), or across domains.

## Fine-grained by default

Uniform does not mean coarse. Access resolves to the **finest meaningful
granularity**, and policy is least-privilege at that granularity:

- **Fine-grained addressing.** A UDCI addresses not just a resource but a part of
  one — a field, a fragment, a sub-record (`uaf://bafk...#/steps/2/instruction`).
  You name exactly what you mean.
- **Fine-grained authorization.** Capability grants scope to a **specific verb on a
  specific resource (or field)**, optionally conditioned on the five primitives
  (e.g. *READ this field, from the EU zone, if trust ≥ attested, until T*). Access
  is granted per-operation, not per-API.
- **Fine-grained metering.** Resource accounting
  ([World Model budgets](./WORLD_MODEL.md)) is per-call, per-token, per-byte — so
  cost and limits attach to the exact operation, not a coarse quota.
- **Fine-grained duties & rights.** [Governance](./GOVERNANCE.md) obligations and
  entitlements scope to particular resources, fields, and operations — a right to
  read one field is not a right to read the record.
- **Least privilege.** A participant holds exactly the grants its task needs and no
  more; broad access must be explicitly, auditably granted.

Fine granularity makes the uniform interface *safe*: one consistent way to reach
everything, with precise control over exactly what each actor may touch.

## How the layers compose

```
   name        ──▶  Universal Decentralized Canonical Identifier (UDCI)
   access      ──▶  Uniform Access Protocol   (resolve/read/write/query/subscribe/invoke)
   coordinate  ──▶  Universal Agent Protocol (UAP)   (entries · phases · workspace)
   enforce     ──▶  Gates + five primitives + three-party signing
```

Naming, access, coordination, and enforcement are one coherent stack: you *name*
with a UDCI, *access* it uniformly, the access *is* a UAP entry, and the entry is
*enforced* by gates over the primitives.

## Why uniform

| Benefit | From uniformity |
| --- | --- |
| Interop | tools, models, data, and norms are reached the same way → true multi-model / multi-framework / multi-domain access |
| One security model | a single gate + signing + primitive policy, not per-API bespoke auth |
| Evolvability | new resource kinds need no new access API — just a UDCI and the verbs |
| Cacheability & reproducibility | content-addressed reads are verifiable and pinnable |
| Clarity | one interface to learn and audit ([clarity](./DESIGN_PRINCIPLES.md)) |

> Status: design direction. The Uniform Access Protocol is a thin interface over
> UDCI-named resources whose operations map onto existing UAP entry kinds; it adds
> no envelope fields and reuses the gate, primitive, and signing models.
