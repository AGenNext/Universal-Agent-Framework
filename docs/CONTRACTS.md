# UAF Contracts

## Purpose

Contracts define the stable boundaries of UAF.

They prevent bleeding at the edge, protect the substrate, and allow the system to expand without breaking trust.

```text
Touchpoints -> Adapters -> Runtime -> Substrate -> Workspace/Evidence/Replay
```

## Contract rule

Each layer may depend only on the layer below it through an explicit contract.

No layer may bypass evidence, replay, or workspace semantics.

## 1. Substrate Contract

The substrate is the smallest trusted base.

It owns:

```text
Entry
Workspace
Evidence
Replay
Outcome summary
```

It provides:

```text
validate_entry(entry)
append(workspace, entry)
read_entries(workspace)
replay(workspace)
validate(workspace)
```

It must preserve:

```text
append-only JSONL
contiguous seq
single-session replay for MVP
UAP version
entry kind validation
replay determinism
```

It must not own:

```text
network
secrets
shell execution
business workflow
UI
marketplace
identity provider
payment
cloud control plane
```

## 2. Entry Contract

An accepted MVP entry has:

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

Current stable MVP kinds:

```text
task
evidence
result
error
```

Current stable MVP phases:

```text
cognition
complete
```

Compatibility rule:

```text
Existing fields remain valid.
New fields must be optional until a major version change.
```

## 3. Workspace Contract

The workspace is the durable edge-local record.

```text
one entry per line
newline-delimited JSON
append-only by default
replayable without external services
```

Current default:

```text
.uaf/session.jsonl
```

Workspace rules:

```text
no hidden mutation
no implicit machine scan
no network dependency
no database dependency for MVP
```

## 4. Replay Contract

Replay reconstructs the accepted session from committed entries.

Replay must:

```text
read JSONL
validate each entry
verify session consistency
verify seq ordering
count tasks/evidence/results/errors
produce final summary
```

Replay must not:

```text
execute tools
call network
modify workspace
infer hidden state
```

## 5. Runtime Contract

Runtime coordinates work using the substrate.

It may:

```text
read task file
create task entry
emit evidence entry
produce result entry
call substrate append/read/replay
```

It must not:

```text
bypass entry validation
write malformed workspace lines
change replay semantics
hide side effects
```

## 6. Report Contract

Reports render evidence and replay state into human-readable artifacts.

Reports may:

```text
summarize session
show counts
show final result
write markdown/json/html later
```

Reports must not:

```text
invent truth
change workspace
replace replay
make unsupported compliance claims
```

Current default:

```text
reports/out/session-summary.md
```

## 7. Adapter Contract

Adapters connect UAF to outside systems.

Adapters may touch:

```text
models
tools
APIs
Kubernetes
OPA
cloud systems
enterprise systems
marketplaces
```

Adapters must:

```text
declare permissions
declare inputs/outputs
emit evidence
respect consent/context
use substrate for committed truth
avoid bypassing replay
```

Adapters must not:

```text
write directly to workspace without substrate contract
silently collect personal data
execute without policy where policy is required
claim success without evidence
```

## 8. Touchpoint Contract

Touchpoints are user/system interfaces.

Examples:

```text
CLI
API
chat
IDE
web
mobile
reports
dashboards
Kubernetes Job
```

Every touchpoint maps to the same interaction grammar:

```text
Expression -> Context -> Intent -> Action -> Evidence -> Outcome -> Report
```

Touchpoints may differ in presentation but not in evidence semantics.

## 9. Enterprise Contract

Enterprise features add governance without bloating the substrate.

Enterprise layer owns:

```text
consent
intent
choice
policy
approval
audit
lifecycle
support boundary
```

Enterprise features must be:

```text
additive
versioned
evidence-emitting
privacy-respecting
compatible with old workspaces
```

## 10. Stability Contract

Do not break without a major version:

```text
uaf run
uaf replay
uaf validate
uaf report
workspace JSONL
entry envelope
seq replay
basic report output
```

Prefer additive evolution:

```text
optional fields
new commands
new entry kinds
new report templates
new adapters
new context packs
```

## 11. Gate Contract

Every change must pass the appropriate gates:

```text
compatibility
evidence
security
privacy
ACID/idempotency where state changes
sanitize
sign/provenance for release
lifecycle/retirement for published artifacts
```

## Final rule

Contracts are the guardrails.

The substrate stays stable. The runtime expands carefully. Adapters expand contact. Touchpoints improve access. Evidence keeps everything accountable.