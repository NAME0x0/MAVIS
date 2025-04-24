# Contributing to MAVIS

First off, thank you for considering contributing to MAVIS! It's people like you that make MAVIS such a great tool.

Following these guidelines helps to communicate that you respect the time of the developers managing and developing this open source project. In return, they should reciprocate that respect in addressing your issue, assessing changes, and helping you finalize your pull requests.

## Code of Conduct

MAVIS has adopted a Code of Conduct that we expect project participants to adhere to. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) so that you can understand what actions will and will not be tolerated.

## Getting Started

Contributions are made to this repo via Issues and Pull Requests (PRs).

*   **Issues:** Used to track bugs, feature requests, or general questions.
*   **Pull Requests:** Used to propose changes to the codebase.

Before starting work on a major change, please open an issue first to discuss the proposed changes and ensure they align with the project's goals.

## How to Contribute

1.  **Fork the Repository:** Create your own fork of the MAVIS repository on GitHub.
2.  **Clone Your Fork:** Clone your fork locally: `git clone https://github.com/YOUR_USERNAME/MAVIS.git`
3.  **Create a Branch:** Create a new branch for your changes: `git checkout -b feature/your-feature-name` or `fix/your-bug-fix-name`.
4.  **Make Changes:** Implement your changes or bug fixes.
    *   Ensure your code adheres to the project's coding style (details TBD - e.g., run `cargo fmt`).
    *   Add tests for any new functionality.
    *   Update documentation if necessary.
5.  **Commit Changes:** Commit your changes with a clear and descriptive commit message (following Conventional Commits format is preferred, e.g., `feat: Add new widget type`).
    ```bash
    git add .
    git commit -m "feat: Describe your feature"
    ```
6.  **Push Changes:** Push your changes to your fork: `git push origin feature/your-feature-name`.
7.  **Open a Pull Request:** Go to the original MAVIS repository and open a Pull Request from your fork's branch to the `main` branch of the MAVIS repository.
    *   Provide a clear title and description for your PR, explaining the changes and referencing any related issues (e.g., "Closes #123").

## Development Setup

Refer to the `README.md` section [10.2 Build & Run (Development/Testing)](README.md#102-build--run-developmenttesting) for instructions on setting up your development environment and building the project.

## Coding Style (TBD)

*   Run `cargo fmt` to format your code.
*   Run `cargo clippy` to catch common mistakes and style issues.
*   (Add any other project-specific style guidelines here).

## Testing

*   Run `cargo test` to execute the test suite.
*   Ensure all existing tests pass and add new tests for your changes. Refer to [15. Testing](README.md#15-testing) in the README.

## Issue and PR Templates

(Consider adding issue and PR templates to the `.github` directory later).

Thank you for your contribution!