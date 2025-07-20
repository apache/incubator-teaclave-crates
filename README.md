# Teaclave Dependency Crates

[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)
[![Homepage](https://img.shields.io/badge/site-homepage-blue)](https://teaclave.apache.org/)

This repository hosts Rust crates maintained by the [Teaclave community](https://github.com/apache/incubator-teaclave). These include ported and TEE-adapted dependencies designed for secure, memory-safe development in confidential computing environments.

## Purpose of This Repository

### Adapting With Target-Dependent Security Primitives

While Teaclave SDKs aim to be as compatible with std as possible, some crates cannot be used out-of-the-box due to TEE-specific security constraints. This often requires additional effort to port or adapt existing crates—such as replacing randomness sources, handling untrusted filesystems, or accommodating different security assumptions.

### Easing Upstream Integration Barriers

Ideally, we would upstream patches to add confidential computing support directly into the original crates. However, this depends on upstream maintainers' interest and alignment, which can be challenging—especially when the original crate was not designed with TEE support in mind.

This repository serves to:
- Demonstrate how crates can be adapted for TEE environments;
- Provide reusable versions that developers can depend on directly;
- Help developers learn from the diffs and port their own crates if needed.

## Principles for Management

This repository follows a structured monorepo layout under the `crates/` directory. Each subdirectory hosts a TEE-adapted version of an upstream Rust crate.

```
crates/
├── foo/ # Adaptation of the foo crate
├── bar/ # Adaptation of the bar crate
├── ...
```

Each adapted crate is:

- Maintained in its own isolated subdirectory;
- Version-aligned with the corresponding upstream crate where possible;
- Published to [crates.io](https://crates.io) under the `teaclave-*` namespace once it passes review.

For example, an adaptation of the `ring` crate would be published as `teaclave-ring`. Developers can add these crates directly in their `Cargo.toml`, and compare them with their upstream counterparts on crates.io.

The repository follows these principles:

| Phase       | Description                                                                 |
|------------|-----------------------------------------------------------------------------|
| Development| Crates must be ported from the latest stable upstream versions on crates.io.|
| Review     | Each crate undergoes a security review focused on diffs from the upstream.  |
| Testing    | TEE-specific test suites must pass before merging.                          |
| Publishing | Stable versions are published to crates.io as `teaclave-*`.                 |
| Iteration  | New upstream versions must follow the same process, replacing the old one.  |

- The repository includes only the **latest ported version** of each crate.
- [crates.io](https://crates.io) hosts all **published stable versions**.
- Users can depend on any published version using standard Cargo dependency syntax.
