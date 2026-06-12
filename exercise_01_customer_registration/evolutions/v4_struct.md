# Exercise 01 — Customer Registration (Version 4.0)

## Core Concept

Practice Domain Modeling with `struct`.

---

## Technical Objectives

* Introduce Domain Modeling by representing real-world entities as Rust types.
* Group related data using `struct`.
* Replace scattered variables with organized records.
* Prepare the application's architecture for future growth.
* Access and manipulate structured data using field notation.

---

## Problem Context

Borrow & Brew has expanded.

The loyalty program that Ana manages at the cashier now has hundreds of registered customers. At first, storing customer information in separate variables seemed manageable:

```rust
let customer1_name = ...;
let customer1_cpf = ...;

let customer2_name = ...;
let customer2_cpf = ...;
```

However, as the number of customers increased, José and João began to notice that the code inside `main` was becoming increasingly difficult to maintain. Every new customer required another collection of disconnected variables, making the system harder to understand and evolve.

At the same time, José started planning the next phase of the business: a loyalty points program. Customers would eventually accumulate points, update their contact information, and interact with future modules such as orders, payments, and rewards.

The team realized they could no longer think of a customer as "a name variable and a CPF variable." Instead, the system needed to treat a customer as a single entity with its own identity and characteristics.

Your mission in Version 4.0 is to model the Borrow & Brew domain by creating a `Customer` structure capable of grouping all customer-related information into one coherent type.

The sanitization and validation challenges have already been solved in previous versions. Now, your focus shifts toward software architecture: transforming validated data into meaningful business entities.

By the end of this exercise, the Borrow & Brew system will be ready to scale beyond isolated variables and move toward a more professional, maintainable design.

---

## Dummy Data

After processing the raw input using the sanitization and validation functions developed in previous versions, the system should be capable of producing the following customer entities:

* **Customer 1:**

  * Name: `"Maria Silva"`
  * CPF: `12345678909`

* **Customer 2:**

  * Name: `"Pedro Santos"`
  * CPF: `98765432101`

---

## Mandatory Steps

Create the file (or modify your existing file if you prefer) and follow this implementation order:

### Step 1:

Define the Customer Struct.

Outside the `main` function, create a custom type representing a customer.

Use the following structure:

```rust
struct Customer {
    name: String,
    cpf: u64,
}
```

Notice that the CPF should continue using a numeric type, leveraging the work completed in previous exercises.

---

### Step 2:

Integrate with Previous Validation Functions.

Keep the functions developed in earlier versions available in the same file:

* The name sanitization function from Version 2.0;
* The CPF sanitization and validation function from Version 3.0.

Inside `main`, process Maria Silva's raw input through these functions before attempting to build a customer entity.

This reinforces the idea that domain entities should be created from trusted data whenever possible.

---

### Step 3:

Instantiate the First Customer.

If the validation process succeeds, use the sanitized values to instantiate your first `Customer`.

Example:

```rust
let new_customer = Customer {
    name: sanitized_name,
    cpf: validated_cpf,
};
```

At this stage, Maria Silva officially becomes a domain entity instead of a collection of unrelated variables.

---

### Step 4:

Evolve the Struct for Future Growth.

José's plans for the loyalty program require additional information.

Update the `Customer` structure by introducing two new fields:

```rust
telephone: String,
loyalty_points: u32,
```

Your updated structure should now represent a richer view of the customer.

When creating Maria Silva's instance, initialize:

* The telephone field with an empty `String`;
* The loyalty points field with `0`.

This demonstrates how domain models evolve alongside business requirements.

---

### Step 5:

Create a Second Customer.

Instantiate a second customer directly inside `main` using static values.

Use the following information:

* Name: `"Pedro Santos"`
* CPF: `98765432101`
* Telephone: an empty `String`
* Loyalty Points: `0`

This step demonstrates how Rust allows multiple instances of the same custom type to coexist independently.

---

### Step 6:

Generate the Customer Registry Report.

Use `println!` statements to display the stored information by accessing the internal fields of each customer through dot notation.

Examples:

```rust
customer.name
customer.cpf
customer.loyalty_points
```

The final terminal output should present customers as organized records, ready to be persisted into a database or consumed by future system modules.

---

## Learning Goals

The objectives of Exercise 01 — Version 4.0 focus on transitioning from isolated data manipulation to intentional software design.

They are divided into four main pillars:

### 1. Introduce Domain Modeling

**Objective:** Learn to think about software in terms of business entities instead of disconnected variables.

**In practice:** A customer becomes a first-class concept represented by a dedicated Rust type.

---

### 2. Organize Related Data

**Objective:** Group characteristics that belong together under a single structure.

**In practice:** Instead of managing separate variables for names, CPFs, telephones, and loyalty points, all customer information lives inside one `Customer` instance.

---

### 3. Prepare the System for Growth

**Objective:** Design software that can evolve as business requirements expand.

**In practice:** Adding fields such as telephone numbers and loyalty points becomes a natural extension of the domain model rather than a disruptive change scattered throughout the codebase.

---

### 4. Understand Structs as Building Blocks of Software Architecture

**Objective:** Recognize that custom types are fundamental tools for creating expressive and maintainable systems.

**In practice:** `struct` allows developers to model real-world concepts directly in code, improving readability, reducing complexity, and establishing a foundation for future features such as methods, modules, databases, and APIs.

---

## Evolution of Exercise 01

Throughout the Borrow & Brew journey, each version introduced a new layer of professional software development:

* **Version 1.0:** Shadowing and basic data sanitization.
* **Version 2.0:** Functions, code reuse, and smart capitalization.
* **Version 3.0:** Reliability through `Result`, `Option`, and validation pipelines.
* **Version 4.0:** Domain Modeling with `struct`, transforming validated data into meaningful business entities.

The focus is no longer merely cleaning data or preventing failures.

The system is now beginning to represent the business itself.