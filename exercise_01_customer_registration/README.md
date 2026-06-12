# Exercise 01 — Customer Registration

## Core Concept

Progressively introduce Rust fundamentals through the implementation of a real-world customer registration workflow.

---

## Evolution

| Version | Objective                     | Concept         | Status      |
| ------- | ----------------------------- | --------------- | ----------- |
| V1      | Sanitize customer data        | Shadowing       | ✅ Completed |
| V2      | Reuse code                    | Functions       | ✅ Completed |
| V3      | Validate CPF                  | Result / Option | ✅ Completed |
| V4      | Domain modeling               | Struct          | ✅ Completed |
| V5      | Encapsulate customer behavior | `impl`          | 🚧 Planned   |

---

## Current State

**Implemented version:** V4

The customer registration workflow currently supports:

* Customer name sanitization with smart capitalization;
* CPF sanitization and validation;
* Explicit error handling using `Result<T, E>`;
* Safe handling of optional values through `Option<T>`;
* Domain modeling using a dedicated `Customer` struct;
* Creation of validated customer entities ready for future system expansion.

---

## Next Step

**Version 5.0 — Encapsulate Customer Behavior**

The next iteration will introduce Rust's `impl` blocks to move business behavior into the `Customer` type itself, allowing domain entities to manage their own operations and responsibilities.