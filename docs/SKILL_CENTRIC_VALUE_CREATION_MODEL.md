# Skill-Centric Value Creation Model

## Intent

UAF should create value for all stakeholders through a skill-centric operating model.

Skills are the bridge between people, agents, artifacts, outcomes, education, enterprise adoption, and economic opportunity.

```text
skill -> work -> artifact -> evidence -> outcome -> recognition -> reward -> growth
```

## Principle

Outcome-driven work becomes sustainable when it is connected to skill growth.

The ecosystem should not only ask:

```text
What work was completed?
```

It should also ask:

```text
What skill created the value?
What evidence proves it?
Who should be recognized?
How can this skill be taught, reused, certified, and rewarded?
```

## Stakeholder value map

| Stakeholder | Value created | Value received |
| --- | --- | --- |
| Learners | practice, feedback, completed labs | skills, credentials, opportunity |
| Contributors | docs, examples, tests, fixes | attribution, reputation, progression |
| Builders | policies, reports, dashboards, connectors | revenue, certification, enterprise access |
| Researchers | methods, papers, benchmarks | citations, funding, productization path |
| Educators | courses, labs, mentorship | income, recognition, ecosystem growth |
| Maintainers | quality, review, releases | funding, authority, sustainability |
| Enterprises | real-world validation, sponsorship | trusted outcomes, evidence, support |
| Partners | implementation and support | services revenue, verified status |
| Company/steward | coordination, trust, brand, platform | sustainable revenue, IP, ecosystem health |
| Community | adoption, feedback, local growth | open access, opportunity, shared trust |

## Skill object

A UAF skill should be represented as data.

```json
{
  "id": "skill.uaf.policy_author.v1",
  "title": "UAF Policy Author",
  "description": "Ability to write OPA/Rego policies that gate UAF tool calls and emit policy verdict evidence.",
  "level": "practitioner",
  "evidence_required": [
    "policy file",
    "workspace log",
    "policy_verdict entries",
    "evidence report"
  ],
  "artifacts_produced": [
    "policy/opa/*.rego",
    "reports/out/policy-evidence.md"
  ],
  "outcomes_supported": [
    "safe tool execution",
    "audit-ready policy evidence"
  ]
}
```

## Skill progression ladder

```text
Awareness
  -> Beginner
  -> Practitioner
  -> Builder
  -> Verified Builder
  -> Operator
  -> Expert
  -> Maintainer
  -> Trainer
  -> Steward
```

Progression should be evidence-based, not purely title-based.

## Skill-to-artifact mapping

| Skill | Produces | Evidence |
| --- | --- | --- |
| Protocol reader | valid UAP entries | validated workspace |
| Kernel developer | runtime code, tests | CI results, replay success |
| Policy author | Rego policies | policy verdict entries |
| Evidence designer | evidence schema/report | evidence bundle |
| Dashboard builder | dashboard specs | rendered dashboard output |
| Report author | report templates | generated reports |
| Connector builder | connector artifact | integration run log |
| Kubernetes operator | Helm/K8s deployment | deployment report |
| Researcher | paper/benchmark | reproducibility bundle |
| Educator | lab/course | learner artifacts |
| Enterprise advisor | deployment plan | customer outcome report |

## Skill-to-outcome mapping

Skills should be tied to outcomes.

Example:

```text
Outcome: audit-ready agent execution
Required skills:
  - UAP protocol modeling
  - OPA policy authoring
  - evidence design
  - report generation
  - Kubernetes operations
  - enterprise governance mapping
```

This helps learners understand why a skill matters and helps enterprises identify what capability they need.

## Skill evidence

Every skill claim SHOULD be supported by signed evidence.

Evidence may include:

```text
workspace JSONL
replay report
policy verdicts
evidence bundle
report output
signed artifact
merged contribution
review record
certification task
customer outcome report
teaching/lab completion
```

## Skill-centric rewards

Rewards should connect to verified skills and outcomes.

Reward paths:

```text
badge
credential
artifact marketplace access
paid implementation work
trainer eligibility
partner eligibility
maintainer role
research grant
enterprise advisory opportunity
revenue share
```

## Inclusive growth

Skill-centric does not mean elitist.

The ecosystem should support:

```text
free beginner paths
localized education
mentorship
small-business partner paths
low-cost certification options
scholarships where possible
public examples
clear contribution ladders
```

## Enterprise value

Enterprises benefit because skills become visible and verifiable.

They can ask:

```text
Who built this artifact?
What skill does it require?
Who maintains it?
What evidence proves quality?
Which outcomes has it supported?
What support boundary applies?
```

## Community value

Community members benefit because skill development becomes economically meaningful.

```text
learn skill
  -> produce artifact
  -> get reviewed
  -> earn credential
  -> publish artifact
  -> support enterprise use
  -> teach others
```

## Research value

Researchers benefit because methods can become skills, labs, benchmarks, and enterprise artifacts.

```text
research method
  -> reproducible artifact
  -> educational lab
  -> certification task
  -> enterprise playbook
```

## Skill catalogs

The MVP should eventually include:

```text
skills/uaf-foundations.json
skills/uaf-policy-author.json
skills/uaf-kernel-developer.json
skills/uaf-evidence-designer.json
skills/uaf-kubernetes-operator.json
skills/uaf-report-builder.json
skills/uaf-researcher.json
skills/uaf-trainer.json
```

## Done condition

The skill-centric model is working when:

1. every major artifact maps to one or more skills,
2. every certification requires real evidence,
3. every enterprise outcome maps to required skills,
4. community contributors can progress through visible skill levels,
5. builders can monetize verified skills responsibly,
6. enterprises can trust signed artifacts and qualified people,
7. education creates both capability and opportunity.

## Final rule

Skills are the human side of the protocol.

UAF should not only automate work. It should help people build capability, create value, prove outcomes, earn trust, and grow economically.