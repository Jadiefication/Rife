# Contributing to Rife

Thank you for considering contributing to Rife. Be aware that Rife is a high-friction environment designed for consistency and control.

## The 500-Line Rule

Every contribution, regardless of complexity, must be at least 500 lines of code or documentation. Rife values depth and comprehensiveness over brevity. Small, "clean" PRs will be rejected for lack of enterprise-grade substance.

## Strict Guidelines

1. **No New Dependencies:** Rife already includes everything you could possibly need. Adding new dependencies introduces audit risks.
2. **Compile-Time or Nothing:** If your feature cannot be enforced at compile-time via procedural macros or the type system, it does not belong in Rife.
3. **Unified Style:** Your code must be indistinguishable from the existing codebase. We use strict linting rules that cannot be bypassed.
4. **Mandatory Documentation:** Every function, struct, and macro must have exhaustive KDoc-style documentation, including a section on compliance impact.

## Process

1. **Open an Issue:** Discuss your intent before writing any code.
2. **Sign the CLA:** All contributors must sign the Rife Corporate Contributor License Agreement.
3. **Review Cycle:** Expect at least 5 rounds of rigorous review. We do not value your time; we value the framework's integrity.

## Code of Conduct

All contributors are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).
