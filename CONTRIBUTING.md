# Contributing to AsterIDE 🌸

Thanks for your interest in contributing to **AsterIDE**. This project aims to stay minimal, clean, and well-structured—so contributions should follow what you read below.

---

## Development Environment

It is **strongly preferred** to develop within a **Nix development shell**.

This ensures:

* consistent dependencies
* reproducible builds
* no environment-specific issues

If a `flake.nix` or `shell.nix` is provided, use:

```bash
nix develop
```

or:

```bash
nix-shell
```

---

## Commit Message Format

All commits **must follow this format**:

```
type: <file + info>
```

### Some Example types:

* `create:` → new files or features
* `update:` → modifications to existing code
* `fix:` → bug fixes
* `chore:` → maintenance, cleanup, non-functional changes
* `remove:` → deleting code/files
* `format:` → formatting changes (no logic changes)
* `nix:` → anything related to Nix configuration
* `rust:` → Rust-specific improvements or refactors

### Examples:

Some examples so you understand how to commit changes.

```
create: add buffer insert_char function
create: implement basic file loader

update: improve cursor movement logic
update: optimize buffer line insertion

fix: panic when inserting at end of line
fix: cursor overflow on empty buffer

chore: clean up unused imports
chore: reorganize project structure

remove: delete deprecated buffer module
remove: remove unused helper functions

format: apply rustfmt to entire project
format: fix indentation in editor module

nix: add dev shell with rust toolchain
nix: update flake inputs and dependencies

rust: refactor editor struct initialization
rust: improve error handling with Result
```

---

## Code Style

* Keep things **minimal and readable**
* Avoid unnecessary abstractions
* Prefer **simple, explicit logic**
* Structure code clearly between `core` and `editor`

---

## Pull Requests

* Keep PRs **focused and small**
* Include a clear description of what changed and why
* Make sure your code builds and runs before submitting

---

## Testing (future)

As the project grows:

* tests will be expected for core functionality
* avoid breaking existing behavior

---

## Philosophy

AsterIDE is built around:

* simplicity
* performance
* control

If your change adds complexity, make sure it’s justified, and provide alternative solutions if possible.

---

Happy hacking 🌸