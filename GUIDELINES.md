# Library Maintenance Guidelines (v2.0+)

## Introduction

These guidelines define how we maintain and evolve this library from version **2.0 onward**.

For general Rust API design principles, see the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

This library is used by multiple teams, and all code under `src/` is considered **public, stable API** unless explicitly marked otherwise.  
To mark code as non-stable or internal, use this three-tier approach:

**Tier 1 - Preferred (truly internal):**
- Keep implementation details in private modules (no `pub` modifier)
- This is the default approach for all internal code

**Tier 2 - Crate-internal visibility:**
- Use `pub(crate)` for code needed across modules within this crate only
- This prevents external crates from accessing internal implementation details
- Document these items normally (no special warning needed since they're not externally accessible)

**Tier 3 - When full public visibility is required:**
- Use `pub mod internal` for internal code that must be accessible to external crates (rare)
- Use [`#[doc(hidden)]`](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html#hidden) for cross-crate technical requirements (trait implementations, macro internals, etc.)
- All such items must have a doc comment starting with `# Internal API - Do not use directly` explaining why it must be public

The goal is to maintain a consistent, safe, easy-to-use API that can be depended on long-term.

---

## 1 - Public API Design (must)

- Public functions, structs, enums, and modules must have clear, descriptive names.
- Prefer strong types or newtypes instead of primitives when values could be confused (e.g., `UserId(u64)` instead of `u64`, or `struct Email(String)` instead of plain `String`).
- Keep public function signatures simple and easy to call.
- Avoid exposing internal implementation details.  
  Internal helpers should remain private. Only use `pub` for internal code when technically required (see intro for guidelines).
- Prefer owned types (`String`, `Vec<T>`) in public API; use references only when it is clearly more ergonomic.
- Avoid unnecessary generics and complex trait bounds in public interfaces.

**Goal:** Public API should be obvious and hard to misuse.

---
## 2 - Documentation (must)

- Every `pub` item must have a `///` doc comment that explains:
  - What it does  
  - What inputs/outputs mean  
  - When it returns errors  
  - Any important constraints or assumptions

- Use `//!` module-level comments to explain the purpose of a module.
- Keep documentation clear and concise.  
- Do **not** use doc-tests (code examples in `///` comments that run as tests).
  - Given the extensive comments in our codebase, avoiding large code blocks in documentation improves readability.
  - All testing should be done via unit tests in `#[cfg(test)]` modules or integration tests in `tests/`.
  - Brief, non-executable code snippets may be included in docs for illustration purposes when they clarify usage.

**Goal:** Users should understand functionality without reading the code.

---
## 3 - Error Handling (must)

- Avoid `unwrap()`, `expect()`, and panics in library code.
- Use well-defined error enums.
- Document all error variants in rustdoc.
- Add context to errors when needed so users can diagnose issues.

**Goal:** The library should never crash user applications unexpectedly.

---
## 4 - Stability & Versioning (must)

- We follow [semantic versioning](https://semver.org/) from v2.0 onward:
  - **Patch (x.y.z)** → bug fixes only  
  - **Minor (x.y.0)** → non-breaking additions  
  - **Major (x.0.0)** → breaking changes

- Breaking changes require discussion and agreement on the Discord server before merging.
- Use [`#[deprecated(since = "...", note = "...")]`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-deprecated-attribute) before removing or replacing public items.

**Goal:** External projects must be able to trust the API long-term.

---
## 5 - Tests & CI (must)

- **Unit tests** should remain in `src/` alongside the code they test, using `#[cfg(test)]` modules.
  - For small to medium-sized files, tests can be at the bottom of the same file in a `#[cfg(test)] mod tests { ... }` block.
  - For large files, move tests to a separate module file (e.g., `src/my_component/tests.rs` or convert to `src/my_component/mod.rs` + `src/my_component/tests.rs`).
  - This keeps implementation code readable and navigable.
- **Integration tests** (tests that use the library as an external consumer would) should be placed in a `tests/` directory.
- For more on test organization, see [The Rust Book: Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html).
- Tests should cover:
  - Main success cases  
  - Common edge cases  
  - Error behavior  
- All new public functionality must include meaningful tests.
- CI must validate:
  - `cargo fmt` (or `just fmt`)  
  - `cargo clippy` (or `just lint`) with no warnings except unused code  
  - `cargo test` (or `just test`) for all unit tests
  - Or to run all at once `just ci`

**Goal:** Ensure correctness and prevent regressions.

---
## 6 - Safety & Unsafe (must)

- Avoid `unsafe` unless absolutely necessary.
- If unsafe is used:
  - Keep the unsafe block as small as possible.
  - Add a `// SAFETY:` comment explaining why it is correct and what invariants must hold.
  - Include tests that exercise the unsafe path.
- For guidance on writing unsafe code, see [The Rustonomicon](https://doc.rust-lang.org/nomicon/).

**Goal:** Keep the library memory-safe and well-defined.

---
## 7 - Performance (should)

- Prefer clarity over micro-optimization unless there is a measured issue.
- Avoid obvious inefficiencies (accidental O(n²), repeated allocations, cloning in loops).
- Use idiomatic iterator patterns when they improve clarity.

**Goal:** Reasonable performance without sacrificing readability.

---
## 8 - Formatting & Linting (must)

We enforce strict linting using:  
[`cargo clippy`](https://rust-lang.github.io/rust-clippy/) `-- -D warnings -A unused`

This means:
- All Clippy warnings **except unused code** must be fixed before merging.
- Unused code warnings are allowed because this is a public library and some public exports may not be used internally.
- However, unused **private** code (functions, types, or fields not part of the public API) should be removed unless:
  - It will be used by code currently being developed by another contributor
  - That contributor must be mentioned in a comment (e.g., `// TODO(@username): Will be used for X feature`)
  - The same contributor must review the PR and approve it, confirming they acknowledge the implementation and that it meets their needs
  - This exception is for near-term anticipated use, not indefinite storage of unused code

Formatting:
- All code must be formatted using [`cargo fmt`](https://rust-lang.github.io/rustfmt/).
- CI checks formatting and lint rules; contributors must fix issues before merge.

**Goal:** Keep code consistent and clean.

---
## 9 - Module Organization (should)

- Keep `lib.rs` minimal and use it to expose the public API clearly.
- Avoid making large modules with unrelated functionality.
- Group related code logically within submodules.
- Keep internal utilities private by default. Use `pub mod internal` only when cross-module visibility is needed (see intro).

**Goal:** Make the library easy to navigate and extend.

---
## 10 - Changelog (must)

- Maintain a `CHANGELOG.md` file in the repository root following [Keep a Changelog](https://keepachangelog.com/) format.
- Document all notable changes for each version under these categories:
  - **Added** - new features
  - **Changed** - changes in existing functionality
  - **Deprecated** - soon-to-be removed features
  - **Removed** - removed features
  - **Fixed** - bug fixes
  - **Security** - vulnerability fixes
- Update the changelog as part of each PR that affects the public API or fixes bugs.
- Keep an "Unreleased" section at the top for changes not yet in a tagged release.
- When cutting a release, move "Unreleased" changes to a new version section with the release date.

**Goal:** Provide users with a clear history of changes between versions.

---
## 11 - Release Checklist (v2.0+)

Before cutting a release:

1. Ensure all CI jobs pass.  
2. Verify that no accidental breaking changes were introduced.  
3. Check for new public items without documentation.  
4. Update `CHANGELOG.md` - move "Unreleased" section to the new version with today's date.
5. Bump version in `Cargo.toml`.  
6. Commit and tag the release in GitHub.

**Goal:** Provide reliable and predictable releases to dependent projects.

---
# Summary

From v2.0 onward, this library’s public API is stable.  
We value:
- Clarity
- Safety
- Documentation
- Backwards compatibility
- Good error handling
- Maintainable structure

Following these guidelines ensures the library remains robust and pleasant to use for all teams.

