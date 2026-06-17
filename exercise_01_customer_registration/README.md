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
| V5      | Encapsulate customer behavior | `impl`          | ✅ Completed |

---

## Current State

**Implemented version:** V5

The customer registration workflow currently supports:

* Customer name sanitization with smart capitalization;
* CPF sanitization and validation;
* Explicit error handling using `Result<T, E>`;
* Safe handling of optional values through `Option<T>`;
* Domain modeling using a dedicated `Customer` struct;
* Encapsulation of customer behavior through methods (`impl`);
* Loyalty points accumulation and redemption;
* Separation between data (`struct`) and behavior (`impl`).

---

## Learning Journey

The exercise evolves through five progressive stages:

1. **V1 — Shadowing**

   * Transform raw customer input into clean data.

2. **V2 — Functions**

   * Extract reusable business logic into dedicated functions.

3. **V3 — Result / Option**

   * Handle failures safely and validate CPF data.

4. **V4 — Struct**

   * Model customers as domain entities.

5. **V5 — impl**

   * Attach business behavior directly to the customer entity.

---

## Outcome

At the end of Exercise 01, the Borrow & Brew system is capable of:

* Receiving raw customer data;
* Sanitizing and validating input;
* Creating domain entities from trusted information;
* Managing customer state through encapsulated behavior;
* Preparing the foundation for future modules such as Orders, Payments, and Loyalty Programs.

---

## Next Step

### Exercise 02 — Order Registration

The next exercise shifts focus from customers to orders.

The learning path will evolve from:

Customer Registration → Order Registration

where new concepts such as collections, iteration, searching, and aggregation will be introduced through the order management workflow.