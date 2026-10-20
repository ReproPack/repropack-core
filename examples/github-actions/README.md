# GitHub Actions failure-evidence example

Copy [`failure-evidence.yml`](failure-evidence.yml) into a test repository only after reviewing the selected files, artifact retention, and public visibility of the workflow. The workflow runs only after a named workflow fails and uploads one bounded, verified ReproPack bundle.

It does not create an issue, inspect the whole workspace, capture secrets, execute evidence, or reconstruct an environment. The Core CLI is installed from the public `v0.1.1` Git tag; replace that tag only through a reviewed release update.
