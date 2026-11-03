---
title: Methodology
date: 2026-10-26
tags:
  - Methodology
---

# DevOps

This page describes how we organize the development, delivery, and operation of the project. The goal of DevOps here is simple: **ship often, in small steps, with as little risk as possible**, by automating everything that is repetitive.

## Principles

* **Everything is versioned**: code, documentation, configuration, and build scripts all live in the Git repository.
* **Everything repetitive is automated**: tests, builds, publishing, and deployment are not done by hand.
* **Small changes, shipped often**: a small change is easier to review, test, and roll back.
* **`main` is always healthy**: it must be buildable and releasable at any time.
* **Secrets never enter the repository**: keys, tokens, and passwords are managed through CI/CD secrets or environment variables.

## Development Cycle

```text
Idea / ticket
     │
     ▼
Working branch ──► Pull Request ──► Review + CI ──► Merge into main
                                                          │
                                                          ▼
                                              Build executables ──► Release
```

### 1. Planning

Each task is described in an *issue*: context, expected behavior, and acceptance criteria. One issue = one clear objective that can be completed in a short time.

### 2. Branch Management

We use a simple flow based on short-lived branches:

| Branch | Role |
|---|---|
| `main` | Stable code, ready to be released |
| `feat/<name>` | New feature |
| `fix/<name>` | Bug fix |
| `docs/<name>` | Documentation change |
| `chore/<name>` | Maintenance, dependencies, tooling |

:::tip
Never push directly to `main`: everything goes through a Pull Request.
:::

### 3. Commit Convention

Messages follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) convention:

```text
feat: add multi-instance management
fix: correct the assets path on Windows
docs: add the Methodology page
chore: update dependencies
```

This keeps the history readable and makes it possible to automatically generate release notes.

### 4. Pull Requests and Code Review

A Pull Request is merged only if:

* CI is green (build, lint, tests);
* at least one person has reviewed it;
* the description explains **what changes and why**;
* the documentation is updated if the behavior changes.

## Continuous Integration (CI)

On every Pull Request and every push to `main`, CI automatically runs:

1. **Installation** of dependencies (with caching to save time).
2. **Linting and formatting** of the code (TypeScript, Rust).
3. **Unit tests**.
4. **Build** of the application.
5. **Documentation build** to detect broken links.

Since the application is cross-platform, the build runs on an **OS matrix**:

| Platform | Target |
|---|---|
| Windows | `x64` |
| Linux | `x64` |
| macOS | `x64` (Intel) and `aarch64` (Apple Silicon) |

## Continuous Delivery (CD)

Publishing a version is triggered by a **Git tag** following semantic versioning (`vMAJOR.MINOR.PATCH`).

```text
git tag v0.2.0
git push origin v0.2.0
```

The release pipeline:

1. builds the executables for each platform;
2. computes the checksums (`sha256`) of the produced files;
3. creates a GitHub *release* with the binaries and release notes generated from the commits.

### Semantic Versioning

| Increment | When? | Example |
|---|---|---|
| `MAJOR` | Breaking change | `1.0.0` → `2.0.0` |
| `MINOR` | New backward-compatible feature | `1.0.0` → `1.1.0` |
| `PATCH` | Bug fix | `1.0.0` → `1.0.1` |

While the project is at `0.x`, the API and behavior may still change.

## Configuration and Secrets Management

* Per-environment configuration (development, production) is handled through **environment variables**.
* Secrets (tokens, signing keys, database credentials) are stored in the **repository secrets** and are never written in plain text in the code, logs, or issues.
* A `.env.example` file documents the expected variables, without their values.

## Quality and Security

* **Dependencies**: regular updates, with an automatic alert when a known vulnerability is found (Dependabot or equivalent).
* **Integrity checks**: files downloaded by the application (libraries, assets, Java) are always verified against their `sha1`.
* **Principle of least privilege**: each service and each token only has access to what it needs.

## Observability

* **Logs** are structured and readable, with no sensitive data (never passwords or tokens).
* **Error reporting** is clear on the user side, with an identifier or a log file that is easy to attach to an issue.
* **Monitoring** of online services (availability and response time) when they are in production.

## Documentation

Documentation is treated **as code**:

* it lives in the same repository and follows the same review process;
* it is published automatically on every merge to `main`;
* it is maintained in **French and English**.

:::note
A feature is only considered done once it is tested, documented, and merged.
:::

## Definition of "Done"

A task is done when:

- [ ] the code is reviewed and merged into `main`;
- [ ] CI is green on all platforms;
- [ ] the documentation is up to date;
- [ ] no secret or sensitive data has been added to the repository;
- [ ] the feature has been manually tested in a real-world case.