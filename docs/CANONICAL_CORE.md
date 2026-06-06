# Canonical Core

## Purpose

The canonical core is the stable substrate of UAF.

It is the part of the system that must remain small, trusted, backward-compatible, replayable, and protected from accidental expansion.

## Canonical core rule

```text
Protect the core.
Expand around it.
Never bypass it.
```

## Canonical core owns

```text
Entry
Workspace
Evidence
Replay
Outcome summary
```

## Canonical core guarantees

```text
valid entries
ordered commits
append-only workspace
contiguous sequence
replayable session
verifiable evidence
stable CLI contract
```

## Canonical core MVP contract

Commands that must remain stable:

```text
uaf run
uaf replay
uaf validate
uaf report
```

Workspace format that must remain stable:

```text
append-only JSONL
one UAP entry per line
```

Entry kinds that must remain valid:

```text
task
evidence
result
error
```

Default tangible outputs:

```text
.uaf/session.jsonl
reports/out/session-summary.md
```

## What must not enter the canonical core

```text
UI
marketplace
billing
payment
network API server
cloud control plane
Kubernetes controller
SSO/SCIM provider
business workflow logic
industry-specific logic
model/provider-specific logic
dashboard rendering
enterprise sales logic
```

These belong around the core as adapters, packs, reports, enterprise modules, or ecosystem layers.

## Promotion rule

A concept may enter the canonical core only if all are true:

1. it is required for replayable truth,
2. it is context-independent,
3. it is needed across touchpoints,
4. it can be versioned and kept backward-compatible,
5. it emits or preserves evidence,
6. it does not require network, secrets, or external services,
7. it strengthens the substrate rather than adding product surface.

## Core change gates

Any change touching the canonical core must pass:

```text
compatibility gate
evidence gate
replay gate
security gate
privacy gate
ACID/idempotency review when state changes
sanitize gate
lifecycle gate
```

## Backward compatibility rule

Do not break without a major version:

```text
CLI commands
workspace JSONL
entry envelope
seq replay
evidence shape
basic report generation
```

Prefer additive changes:

```text
optional fields
new commands
new adapters
new report templates
new context packs
```

## Edge boundary rule

The core must not bleed into the world.

Adapters touch the outside world. The canonical core preserves truth.

```text
world -> adapter -> runtime -> canonical core -> workspace/evidence/replay
```

Adapters must not write around the core or bypass evidence.

## Security posture

The canonical core should remain:

```text
local-first
no network by default
no secrets
no shell execution
explicit file paths
strict validation
minimal dependencies
append-only by default
```

## Final rule

The canonical core is not where every important idea goes.

It is where only the unavoidable truth-preserving primitives go.

Everything else composes around it.