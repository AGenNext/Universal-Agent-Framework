# Rule of Conformance

## Definition

Conformance is the proof that an implementation, artifact, adapter, or release obeys the UAF contracts without breaking the canonical substrate.

```text
conformance = contract compliance + evidence + replayability + compatibility
```

## Primary rule

A thing conforms to UAF only if it can pass through the canonical loop:

```text
input -> entry -> workspace -> evidence -> replay -> report
```

If it bypasses the workspace, evidence, or replay boundary, it is not conformant.

## Conformance levels

### Level 0: Shape conformant

The artifact has the right structure.

Requires:

```text
valid JSON where applicable
required fields present
declared version
known artifact type
```

### Level 1: Contract conformant

The artifact obeys UAF contracts.

Requires:

```text
valid UAP entry envelope
valid workspace JSONL
accepted entry kind
accepted phase
declared compatibility boundary
```

### Level 2: Replay conformant

The artifact can be replayed.

Requires:

```text
contiguous seq
single-session replay for MVP
valid entries
no malformed workspace lines
replay command succeeds
```

### Level 3: Evidence conformant

The artifact produces or preserves evidence.

Requires:

```text
evidence entries exist
evidence has type, summary, and facts
evidence supports the claimed result
report can render evidence-backed summary
```

### Level 4: Operationally conformant

The artifact works in the expected environment.

Requires:

```text
build succeeds
run succeeds
validate succeeds
report succeeds
CI passes
```

### Level 5: Release conformant

The artifact is safe to publish.

Requires:

```text
version
license
manifest
checksum
provenance
sanitize pass
release notes
support boundary
retirement path
```

## MVP conformance command set

Current MVP conformance requires:

```bash
cargo build --locked
cargo run -- run --task examples/tasks/basic-task.json --workspace .uaf/session.jsonl
cargo run -- replay --workspace .uaf/session.jsonl
cargo run -- validate --workspace .uaf/session.jsonl
cargo run -- report --workspace .uaf/session.jsonl --out reports/out/session-summary.md
```

Expected tangible outputs:

```text
.uaf/session.jsonl
reports/out/session-summary.md
```

## Adapter conformance

An adapter is conformant only if it:

```text
declares inputs
declares outputs
declares permissions
emits evidence
preserves consent/context where applicable
never writes around the substrate
never claims success without evidence
```

## Touchpoint conformance

A touchpoint is conformant only if it maps interaction into the same grammar:

```text
Expression -> Context -> Intent -> Action -> Evidence -> Outcome -> Report
```

A touchpoint may change presentation, but not evidence semantics.

## Enterprise conformance

Enterprise features are conformant only if they preserve:

```text
identity
consent
intent
choice
policy
evidence
outcome
```

Where applicable, they must also declare:

```text
purpose
scope
retention
support boundary
compatibility level
```

## Non-conformance

The following are non-conformant:

```text
hidden state that cannot replay
claims without evidence
workspace writes outside substrate contract
breaking existing CLI without versioning
breaking existing workspace replay
network dependency hidden in kernel core
secret dependency hidden in kernel core
report that invents truth not present in evidence
adapter that bypasses policy/evidence boundary
```

## Promotion rule

A component can be promoted only after it reaches the required conformance level.

```text
experimental -> Level 2 minimum
preview -> Level 3 minimum
stable -> Level 4 minimum
official release -> Level 5 minimum
enterprise-supported -> Level 5 + support contract
```

## Final rule

No conformance by claim.

Only conformance by passing evidence-backed checks.