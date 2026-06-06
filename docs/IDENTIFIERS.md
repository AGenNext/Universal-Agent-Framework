# Universal Decentralized Canonical Identifier (UDCI)

**Everyone and everything in UAF has one identifier.** The UDCI is the concrete
realization of the [Identity primitive](./DESIGN_PRINCIPLES.md): a single naming
scheme that is **universal** (every entity has one), **decentralized** (no central
registrar), and **canonical** (one stable, globally-unique identifier per entity).

It is the backbone the rest of the system hangs off: signatures bind to it,
federation authenticates with it, the publish/version/sign
[axiom](./AXIOMS.md) addresses with it, and every `ctx.identity`, `relation`, and
artifact `uri` references it.

## What gets an identifier

Everyone and everything — there is no second-class entity:

| Kind | Identified entity |
| --- | --- |
| **Actors** | humans, agents, tools, models, organizations, devices, domains |
| **Data** | artifacts, definitions/schemas, ontology concepts, norms (rules, constitution) |
| **Process** | sessions, plans, entries |

## The three properties

| Property | Meaning | How |
| --- | --- | --- |
| **Universal** | every entity, one identifier, one scheme | a single URI form across actors, data, and process |
| **Decentralized** | no central authority issues or owns identifiers | self-sovereign: derived from the holder's keys, or from content |
| **Canonical** | exactly one stable, deterministic identifier per entity | key-derived for actors, content-derived (hash) for data |

## The scheme

UDCIs are **DID-compatible** under a `did:uaf` method, with two derivations so
that *who* and *what* are both decentralized and canonical:

- **Actors — key-based (self-sovereign).** Derived from the holder's public key:
  `did:uaf:key:<multibase-pubkey>`. Anyone can mint one; control is proven by the
  key. A domain-scoped form `did:uaf:<domain>:<local>` federates an actor under an
  organization/nation while remaining resolvable cross-domain.

- **Data — content-addressed (canonical & immutable).** Derived from the bytes:
  `uaf://<multihash>`. The identifier *is* the version — any change yields a new
  identifier, so content-addressing gives version control and integrity for free
  (the publish/version/sign axiom).

```
   actor:     did:uaf:key:z6Mk...            (self-sovereign, key-proven)
   org actor: did:uaf:acme:planner-1         (federated under a domain)
   artifact:  uaf://bafy...                   (content-addressed, immutable)
   definition:uaf://bafk...@1.2.0             (content hash + semantic version)
```

## Resolution

A UDCI **resolves** to a document (a DID document for actors) describing:

- public keys / verification methods (for signature checking),
- service endpoints (how to reach the participant's transport),
- declared capabilities (the tool/role descriptor),
- controller / delegation relationships.

Resolution is **decentralized**: an actor document is served by its controller or
a federation peer; content identifiers resolve from any store that has the bytes
(content-addressed, so any source is verifiable). A distributed ledger
([blockchain as a tool](./AXIOMS.md)) **MAY** anchor resolution, rotation, and
revocation — optional, never required.

## Lifecycle

- **Mint** — self-issued (key or content); no gatekeeper.
- **Rotate** — actors rotate keys via their document; the canonical identifier can
  persist across rotation (key-agility) while old keys are retired.
- **Revoke** — revocation is published (and optionally ledger-anchored); a gate
  rejects entries signed by revoked keys.
- **Version** — data identifiers are immutable; a new version is a new identifier,
  with semantic-version metadata linking the lineage.

## How it underpins the rest of UAF

| UDCI enables | In |
| --- | --- |
| `ctx.identity.actor` = a resolvable identifier | [Primitives](./DESIGN_PRINCIPLES.md) |
| signature keys bound to the signer's identifier | [Data Security](./DATA_SECURITY.md) — three-party signing |
| peer authentication and trust anchoring | [Federation](./FEDERATION.md) |
| content-addressed publish + versioning | [Axioms](./AXIOMS.md) |
| stable references in `relation`, `depends_on`, artifact `uri` | the entry envelope |
| capability discovery via resolved documents | [Bindings / adapters](../bindings/README.md) |

Without a universal identifier, "everything is signed, published, and accountable"
is unenforceable — you cannot attribute, resolve, or verify what you cannot name.
The UDCI is what makes attribution and accountability *universal*.

> Status: design direction. The UDCI is DID-compatible naming over the existing
> Identity primitive; `ctx.identity` carries it. It adds a resolution/registry
> facet (optionally ledger-anchored) without changing the entry envelope.
