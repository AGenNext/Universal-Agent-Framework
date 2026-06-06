# Composable Dashboards and Reports

## Definition

UAF dashboards and reports are composable views over the UAP workspace log and evidence entries.

They are not hardcoded product screens in the MVP. They are generated from declarative report specs, dashboard cards, filters, and evidence mappings.

```text
workspace JSONL + evidence entries + report spec -> dashboard/report/export
```

## Principle

The runtime emits evidence. Dashboards and reports compose that evidence into views for different audiences.

```text
Kernel owns: ordered workspace, evidence entries, replay
Report layer owns: query, summarize, filter, render, export
User owns: custom dashboard/report definitions
```

## MVP dashboard model

A dashboard is a JSON document:

```json
{
  "id": "uaf.session.ops.v1",
  "title": "UAF Session Operations",
  "source": {
    "type": "workspace_jsonl",
    "path": ".uaf/session.jsonl"
  },
  "cards": [
    {
      "id": "entries_total",
      "type": "metric",
      "title": "Entries",
      "query": "count(entries)"
    },
    {
      "id": "policy_verdicts",
      "type": "table",
      "title": "Policy Verdicts",
      "query": "entries[kind == 'policy_verdict']"
    },
    {
      "id": "evidence_by_control",
      "type": "grouped_count",
      "title": "Evidence by Control",
      "query": "entries[kind == 'evidence'].body.control_refs"
    }
  ]
}
```

## MVP report model

A report is a JSON or YAML spec that selects entries, maps them to sections, and exports a document.

```json
{
  "id": "uaf.audit.report.v1",
  "title": "UAF Audit Evidence Report",
  "sections": [
    {
      "title": "Session Summary",
      "query": "session.summary"
    },
    {
      "title": "Policy Decisions",
      "query": "entries[kind == 'policy_verdict']"
    },
    {
      "title": "Evidence Trail",
      "query": "entries[kind == 'evidence']"
    },
    {
      "title": "Exceptions and Errors",
      "query": "entries[kind == 'error']"
    }
  ],
  "exports": ["json", "markdown"]
}
```

## MVP output types

The MVP should support these outputs first:

```text
json
markdown
html later
pdf later
csv later
```

JSON and Markdown are enough for the first buildable MVP.

## Dashboard card types

Minimal card types:

| Type | Meaning |
| --- | --- |
| `metric` | Single count or value. |
| `table` | Rows selected from entries. |
| `timeline` | Ordered entries by `seq` or `ts`. |
| `grouped_count` | Count grouped by field. |
| `status` | Pass/fail/needs review state. |
| `text` | Rendered summary. |

Custom card types MUST be prefixed with `x-`.

## Standard MVP dashboards

The repo should include these default dashboard specs:

```text
reports/dashboards/session-ops.json
reports/dashboards/policy-evidence.json
reports/dashboards/compliance-evidence.json
```

### Session Operations Dashboard

Shows:

- session id
- final state
- total entries
- task goal
- phase transitions
- tool calls
- errors
- replay status

### Policy Evidence Dashboard

Shows:

- constraints registered
- policy verdicts
- approvals
- denials
- actions requiring human approval
- denied tool calls

### Compliance Evidence Dashboard

Shows:

- evidence by control reference
- evidence by actor
- evidence by tool
- evidence by data classification
- retention metadata coverage
- unresolved exceptions

## Standard MVP reports

The repo should include:

```text
reports/templates/session-summary.report.json
reports/templates/audit-evidence.report.json
reports/templates/policy-exceptions.report.json
```

### Session Summary Report

For operators and developers.

Sections:

- task
- result
- entries timeline
- tool calls
- errors
- replay summary

### Audit Evidence Report

For security, risk, compliance, and customer review.

Sections:

- purpose
- actor identity
- constraints
- policy verdicts
- approvals
- evidence entries
- control references
- exceptions

### Policy Exceptions Report

For governance review.

Sections:

- denied actions
- needs-human actions
- missing evidence
- degraded checks
- unresolved errors

## Composability rule

A custom dashboard or report must be able to change without changing the runtime.

Composable seams:

- report spec file
- dashboard spec file
- workspace JSONL
- evidence entry fields
- control reference labels
- export format

The kernel MUST NOT know about dashboard layouts or report sections.

## CLI target

The MVP CLI should eventually support:

```bash
uaf report \
  --workspace .uaf/session.jsonl \
  --template reports/templates/audit-evidence.report.json \
  --out reports/out/audit-evidence.md

uaf dashboard \
  --workspace .uaf/session.jsonl \
  --spec reports/dashboards/session-ops.json \
  --out reports/out/session-ops.json
```

## Evidence bundle

A report can emit an evidence bundle:

```text
evidence-bundle/
  session.jsonl
  session-summary.json
  audit-evidence.md
  policy-decisions.json
  artifacts-manifest.json
  replay-report.json
```

The bundle is portable. It can be shared with auditors, customers, security teams, or internal reviewers.

## MVP done condition

Composable dashboards and reports are MVP-ready when a user can:

1. run a UAF session,
2. replay the workspace,
3. generate a session summary report,
4. generate an audit evidence report,
5. customize a report section without changing Rust code.

## Boundary

Do not build a full web dashboard in the MVP.

First build the data model and report generator. A UI can come later and simply render the same dashboard/report specs.