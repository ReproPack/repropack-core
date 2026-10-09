# Contributing

Read `PROJECT_CONTEXT.md`, the applicable phase in `ROADMAP.md`, and the acceptance criteria before changing behavior. Keep specification changes separate from implementation changes when possible.

Start with [the maintainer guide](docs/MAINTAINER_GUIDE.md), [design review guide](docs/DESIGN_REVIEW.md), [compatibility policy](docs/COMPATIBILITY.md), and [fixture contribution guide](docs/FIXTURE_CONTRIBUTION.md). Use the pull-request template and select the smallest applicable review path.

Normative requirements use MUST, SHOULD, and MAY as defined in `docs/SPECIFICATION.md`. Every format change requires an entry in `DECISION_LOG.md`, a conformance fixture or an explicit reason one is unnecessary, and updates to affected implementation repositories.

Before submitting a change, run the checks relevant to touched files, inspect `git diff`, and state what remains unimplemented. Never include credentials, private customer data, generated packages, or fabricated fixtures.
