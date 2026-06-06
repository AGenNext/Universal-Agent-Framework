# UAF Canonical Stack

## Stack

```text
Touchpoints
  -> Adapters
  -> Enterprise Core
  -> Runtime Core
  -> Canonical Core
  -> Workspace / Evidence / Replay
```

## Canonical Core

Owns truth.

```text
entry
workspace
evidence
replay
outcome summary
```

Current modules:

```text
src/entry.rs
src/workspace.rs
src/replay.rs
```

Rules:

```text
no network
no secrets
no shell execution
no business workflow
strict validation
append-only JSONL
```

## Runtime Core

Coordinates work.

Current modules:

```text
src/runtime.rs
src/report.rs
```

Rules:

```text
orchestrate
render
never own truth
never bypass canonical core
```

## Enterprise Core

Adds governance.

Future modules:

```text
identity
consent
choice
policy
approval
audit
lifecycle
```

Rules:

```text
additive
optional
evidence-emitting
privacy-respecting
backward-compatible
```

## Adapters

Touch outside systems.

Examples:

```text
OPA
Kubernetes
OCI
LLM providers
APIs
connectors
```

Rules:

```text
declare permissions
declare inputs and outputs
emit evidence
never write around the core
```

## Touchpoints

Present interaction.

Examples:

```text
CLI
API
chat
IDE
web
reports
dashboards
Kubernetes Job
```

Rules:

```text
same interaction grammar
same evidence semantics
same replay boundary
```

## Current Build Order

```text
1. keep CLI stable
2. finish module split
3. build and fix
4. validate MVP loop
5. add ACID batch commit
6. add idempotency
7. add recovery
8. add signing manifest
9. add OCI/Kubernetes packaging
```

## Non-Negotiable

```text
Many interfaces.
One contract.
Many tools.
One evidence log.
Many contexts.
One replay boundary.
```
