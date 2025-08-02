# Agent Guidelines for asciiquarium-rs

This document outlines the conventions and commands for agentic coding in this repository.

## Build, Lint, and Test Commands

*   **Build:** `cargo build`
*   **Run:** `cargo run`
*   **Lint:** `cargo clippy`
*   **Format:** `cargo fmt` (to fix formatting) or `cargo fmt -- --check` (to check formatting)
*   **Test All:** `cargo test --all-features --all-targets`
*   **Run Single Test:** `cargo test <test_name>` (e.g., `cargo test my_specific_test`)

## Code Style Guidelines

## Agent Workflow

When making any changes, always follow these steps:

1. **Understand the Task:**
   - Read `TODO.md` to identify the next task.
   - Review `SPEC.md` for the current feature specification.
   - Update and improve the todo and spec as needed.

2. **Plan:**
   - If the task is large or complex, break it down into smaller, manageable sub-tasks.
   - Update `TODO.md` with these new sub-tasks.

3. **Implement:**
   - Write or modify code to address the task or sub-task.

4. **Lint & Format:**
   - Run `cargo clippy` to check for linting issues.
   - Run `cargo fmt` to ensure code is properly formatted.

5. **Test:**
   - Run relevant tests using `cargo test <test_name>` or `cargo test --all-features --all-targets` to verify correctness. Prefer `cargo test` over `cargo run` for validation.

6. **Document:**
   - Update any relevant documentation (e.g., `README.md`, `SPEC.md`, `TODO.md`) to reflect your changes.

7. **Commit:**
   - Write a clear and meaningful git commit message summarizing your changes, .


*   **Imports:** Use `use` statements at the top of each module, grouped by `crate`, `super`, `self`, and external crates.
*   **Formatting:** Adhere to `rustfmt` standards. Run `cargo fmt` to automatically format code.
*   **Naming Conventions:**
    *   `snake_case` for functions, variables, and modules.
    *   `PascalCase` for types (structs, enums, traits).
    *   `SCREAMING_SNAKE_CASE` for constants.
*   **Types:** Explicitly type where clarity is improved, otherwise leverage Rust's type inference.
*   **Error Handling:** Prefer `Result` and `Option` for error handling. Use `color_eyre` for enhanced error reporting as seen in `main.rs`.
*   **Comments:** Use `///` for documentation comments and `//` for inline comments. Explain *why* code is written, not just *what* it does.
