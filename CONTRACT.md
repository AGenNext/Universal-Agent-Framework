# UAF Contract

## Contract 0: Canonical Substrate

The substrate is conformant when it can:

```text
accept input
create entries
append workspace
emit evidence
replay state
render report
```

The substrate must not require:

```text
network
secrets
cloud
database
shell execution
external scheduler
```

## Contract 1: Entry

A UAF entry is valid when it has:

```text
uap
id
session
ts
phase
from
role
kind
body
seq
```

MVP accepted kinds:

```text
task
evidence
result
error
```

MVP accepted phases:

```text
cognition
complete
```

Compatibility rule:

```text
existing fields stay valid
new fields are optional
breaking changes require major version
```

## Contract 2: Workspace

A workspace is valid when it is:

```text
newline-delimited JSON
one entry per line
append-only by default
ordered by seq
replayable locally
```

Default path:

```text
.uaf/session.jsonl
```

## Contract 3: Evidence

Evidence is valid when it records:

```text
type
summary
facts
```

Evidence must support a claim.

No evidence means no trust claim.

## Contract 4: Replay

Replay is valid when it:

```text
reads workspace
validates entries
checks seq
checks session consistency
summarizes tasks/evidence/results/errors
returns final result when present
```

Replay must not:

```text
write workspace
call network
execute tools
invent state
```

## Contract 5: Runtime

Runtime is valid when it:

```text
reads declared task input
creates valid entries
uses workspace contract
emits evidence
returns tangible output
```

Runtime must not bypass:

```text
entry validation
workspace append
replay semantics
evidence generation
```

## Contract 6: Report

A report is valid when it:

```text
renders replay state
references actual evidence/result state
creates a tangible file
```

Report must not invent truth outside replayed state.

Default output:

```text
reports/out/session-summary.md
```

## Contract 7: Adapter

An adapter is valid when it declares:

```text
capability
inputs
outputs
permissions
evidence emitted
failure mode
```

Adapters may touch external systems.

Adapters must not write around the substrate.

## Contract 8: Release

A release is conformant when it has:

```text
version
license
changelog
conformance result
sanitize result
manifest/checksum later
support boundary
retirement path
```

## MVP Conformance

Current conformance commands:

```bash
cargo build --locked
cargo run -- run --task examples/tasks/basic-task.json --workspace .uaf/session.jsonl
cargo run -- replay --workspace .uaf/session.jsonl
cargo run -- validate --workspace .uaf/session.jsonl
cargo run -- report --workspace .uaf/session.jsonl --out reports/out/session-summary.md
```

Expected outputs:

```text
.uaf/session.jsonl
reports/out/session-summary.md
```

## Final Rule

A component may extend the stack.

It may not contradict the contracts.
