# Meta of Models

## Core idea

UAF needs a meta-model layer.

A model represents some part of reality, work, knowledge, skill, policy, value, or execution.

A meta-model defines how models themselves are described, related, compared, governed, tested, versioned, reused, and monetized.

```text
reality -> observation -> model -> evidence -> revision

model -> meta-model -> model family -> ecosystem knowledge
```

## Why this matters

UAF should not depend on one fixed model of work, agents, governance, compliance, research, education, or enterprise value.

Everything is contextual.

Different contexts need different models, but those models must remain comparable, reusable, and governable.

The meta-model provides the shared structure.

## Model primitive

Every model should declare:

```text
model_id
name
domain
context
purpose
scope
entities
relations
assumptions
inputs
outputs
constraints
evidence_required
validation_method
version
owner
author
license
provenance
status
```

## Meta-model primitive

A meta-model should declare:

```text
metamodel_id
name
model_family
allowed_model_types
required_fields
relation_rules
validation_rules
comparison_rules
versioning_rules
evidence_rules
governance_rules
reuse_rules
commercialization_rules
```

## Model families

UAF may use many model families:

```text
context model
intent model
outcome model
skill model
tool model
knowledge model
constraint model
policy model
evidence model
artifact model
report model
playbook model
enterprise model
research model
education model
community model
monetization model
federation model
```

## Meta-model lifecycle

```text
1. Define model family
2. Define required primitives
3. Define valid relations
4. Define evidence rules
5. Define validation rules
6. Define versioning rules
7. Define comparison method
8. Define governance boundary
9. Define reuse path
10. Define monetization context
```

## Model relation types

Models can relate through:

```text
defines
uses
extends
constrains
evidences
validates
revises
teaches
packages
monetizes
federates
```

Example:

```text
Skill Model defines required capability.
Outcome Model defines desired value.
Policy Model constrains execution.
Evidence Model validates outcome.
Report Model teaches and communicates value.
Monetization Model packages the artifact.
```

## Model evaluation

A model should be evaluated by:

```text
clarity
usefulness
fit to context
predictive value
operational value
evidence support
replayability
teachability
reuse potential
governance compatibility
commercial sustainability
```

## Meta-model and the kernel

The kernel does not need to understand every model.

The kernel only needs to preserve model references in entries and evidence:

```text
model_id
version
context
evidence_refs
artifact_refs
```

Interpretation belongs to participants, reports, dashboards, researchers, educators, and enterprise packs.

## Meta-model and research

Research creates models.

A research artifact should be able to declare:

```text
which model it proposes
which meta-model it follows
what evidence supports it
what limits apply
how it can be revised
how it can be taught
how it can become a product artifact
```

## Meta-model and enterprise

Enterprise buyers need models to be explicit.

They should know:

```text
which operating model is being used
which policy model applies
which evidence model proves work
which outcome model defines success
which report model communicates value
which support model applies
```

## Meta-model and monetization

A model becomes monetizable when it becomes:

```text
reusable
validated
supported
teachable
certifiable
packaged
trusted
context-specific
```

Paid model artifacts may include:

```text
industry outcome model
enterprise policy model
SOC2 evidence model
GDPR evidence model
skill certification model
partner delivery model
managed service operating model
```

## Avoid dogma

A meta-model prevents any one model from becoming unquestioned dogma.

Every model should be:

```text
contextual
versioned
evidence-seeking
revisable
comparable
bounded
```

## Final rule

Models organize reality.

Meta-models organize models.

UAF should let builders create, test, compare, govern, teach, reuse, and monetize models without pretending any single model is universal truth.