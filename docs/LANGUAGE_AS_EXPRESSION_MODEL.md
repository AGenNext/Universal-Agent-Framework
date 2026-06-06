# Language as Expression Model

## Intent

UAF should recognize language as expression.

Language is how people express emotion, meaning, intent, uncertainty, ambition,
responsibility, and desired outcomes. The system should not treat language as a
mere command string. It should treat language as the first human interface to
work.

```text
emotion -> expression -> meaning -> intent -> protocol -> artifact -> evidence -> outcome
```

## Principle

Natural language is expressive. Protocol is executable. Evidence is verifiable.

UAF should preserve this chain:

```text
human expression
  -> interpreted intent
  -> structured task
  -> governed execution
  -> tangible artifact
  -> evidence-backed outcome
```

## Why language matters

Language carries more than instructions.

It can carry:

```text
need
fear
hope
urgency
ambiguity
context
preference
constraint
judgment
responsibility
purpose
identity
culture
```

The system should convert this into structured work without erasing the human
meaning behind it.

## Expression-to-protocol mapping

| Human expression | Structured UAF form |
| --- | --- |
| "I need help" | task goal |
| "This is risky" | constraint |
| "Do not share this" | privacy/security constraint |
| "Ask me before acting" | approval requirement |
| "Make it audit-ready" | evidence requirement |
| "Use only open tools" | tool/model constraint |
| "I need this by tomorrow" | time constraint |
| "This is for a customer" | purpose/context |
| "I am not sure" | uncertainty marker / needs review |
| "This should create value" | outcome definition |

## Language boundary

Language is the input interface, not the source of truth.

The source of truth is the workspace log.

```text
Expression is captured.
Intent is structured.
Execution is governed.
Evidence is replayed.
```

## Natural-language task model

A natural-language task should be wrapped as a UAP task entry:

```json
{
  "kind": "task",
  "role": "client",
  "body": {
    "input_type": "natural_language",
    "expression": "Create an audit-ready remediation plan, but ask me before any destructive action.",
    "interpreted_intent": {
      "goal": "Create an audit-ready remediation plan.",
      "constraints": [
        "human approval required before destructive action",
        "evidence required for audit readiness"
      ],
      "desired_outcome": "approved remediation plan with evidence bundle"
    }
  }
}
```

## Preserve original expression

The original expression SHOULD be preserved.

Why:

- auditability,
- user trust,
- dispute resolution,
- learning,
- context recovery,
- cultural nuance,
- improvement of interpretation.

The interpreted structure may be wrong. The original expression helps humans and
systems correct it.

## Expression evidence

When natural language is interpreted, the system SHOULD emit evidence:

```text
original expression captured
intent extracted
constraints inferred
uncertainty identified
human review requested if needed
```

Example evidence entry:

```json
{
  "kind": "evidence",
  "body": {
    "type": "intent_interpretation",
    "summary": "Natural-language request was interpreted as an audit-ready remediation planning task with human approval before destructive actions.",
    "facts": {
      "input_type": "natural_language",
      "constraints_inferred": 2,
      "human_review_required": true
    }
  }
}
```

## Multilingual and cultural expression

A federated inclusive ecosystem must respect language diversity.

UAF should support:

```text
local languages
translation artifacts
localized education
regional terminology
cultural context
plain-language explanations
```

The protocol can stay common while expression remains local.

```text
many languages -> one protocol -> many local outcomes
```

## Research connection

Language as expression is a research track.

Possible research questions:

- How does natural-language intent become governed execution?
- How should uncertainty be preserved in protocol entries?
- How can human emotion become constraints without manipulation?
- How can multilingual expression map to a shared protocol?
- How can original expression and interpreted intent be compared during replay?

## Education connection

Education should teach people how to express work clearly.

Skills include:

```text
outcome writing
constraint writing
policy-aware prompting
evidence requirement definition
approval boundary expression
risk communication
```

This makes language a practical skill, not only a communication habit.

## Enterprise connection

Enterprises need language to become accountable work.

A good enterprise system should show:

```text
what was asked
what was understood
what was constrained
what was approved
what was done
what evidence proves it
what outcome was created
```

## Anti-patterns

Avoid:

```text
treating every phrase as a command
losing the original user expression
automating ambiguous language without review
hiding interpretation errors
erasing cultural/local meaning
turning emotional expression into manipulation
claiming certainty where the language was uncertain
```

## Done condition

The language model is working when UAF can:

1. capture original expression,
2. convert it into structured intent,
3. infer or attach constraints,
4. preserve ambiguity and uncertainty,
5. request review where needed,
6. execute through protocol,
7. emit evidence of interpretation,
8. replay both the expression and the interpreted work.

## Final rule

Language is expression.

Expression carries human meaning.

UAF should convert expression into accountable work without losing the human intent that gave the work meaning.