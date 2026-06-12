# Exercise 01 — Customer Registration (Version 2.0)

## Core Concept

Practice Functions and Smart Capitalization.

---

## Technical Objectives

* Introduce code reuse by applying the DRY (Don't Repeat Yourself) principle.
* Create dedicated functions with parameters and return types.
* Apply separation of responsibilities through modular design.
* Perform advanced String manipulation using smart capitalization.

---

## Problem Context

The sanitization system you built greatly helped Ana during customer registration. However, José has now received feedback from customers: they find it unprofessional to see their names printed on cups and tax receipts entirely in lowercase, such as `"maria silva"`.

The ideal Borrow & Brew experience is both friendly and professional, with each word in the customer's name beginning with an uppercase letter, such as `"Maria Silva"`.

At the same time, João from the kitchen and José noticed another problem as the system expanded to new screens: they were repeatedly copying and pasting the same sequence of `.trim()`, `.replace()`, and `.parse()` calls throughout the codebase.

Your mission in Version 2.0 is to encapsulate all of that working logic into dedicated, reusable, and clean functions. In addition, you must implement smart capitalization so customer names are displayed properly throughout the system.

---

## Dummy Data

To maintain consistency with the previous exercise, use exactly the same initial customer data:

* Raw name typed: `"  mArIa sIlVa  "`
* Raw CPF typed: `"123.456.789-00"`

---

## Mandatory Steps

To solve this problem, you must follow this implementation order in your `main.rs` file:

### Step 1:
Create the CPF Sanitization Function.

Outside the `main` function, create the following function signature:

```rust
fn sanitize_cpf(cpf: &str) -> u64
```

Move the logic responsible for removing periods (`.`), hyphens (`-`), and optionally spaces into this function.

At the end of the function, use `.parse()` together with `.expect()` to ensure the function returns a pure integer value (`u64` or `i64`).

---

### Step 2:
Create the Name Sanitization Function (Base Version).

Outside the `main` function, create the following function signature:

```rust
fn sanitize_name(name: &str) -> String
```

Inside this function, apply `.trim()` and `.to_lowercase()` so that the name is first cleaned and standardized in lowercase.

---

### Step 3:
Implement Smart Capitalization.

Still inside the `sanitize_name` function, transform names such as:


`maria silva`


into:


`Maria Silva`


**Hint:** You may split the String into words using `.split_whitespace()`, take the first character of each word, convert it to uppercase, combine it with the remaining characters, and finally join everything back into a single `String`.

---

### Step 4:
Call the Functions Inside `main`.

Simplify the body of your `fn main()`.

Declare the customer's raw variables and replace the previous sanitization logic with simple function calls:

```rust
let sanitized_name = sanitize_name(customer_name);
let sanitized_cpf = sanitize_cpf(customer_cpf);
```

---

### Step 5:
Demonstrate Reusability.

To prove to José that the code is now reusable, declare a second fictional customer directly inside `main`, for example:


`"  jOãO dO sAnToS  "`
`"987.654.321-11"`


Process this customer using the exact same functions without duplicating any sanitization logic.

---

### Step 6:
Deliver the Final V2 Output.

Use `println!` to display both the raw and sanitized data for both customers.

Validate that the final output shows:

* Properly formatted names, such as `"Maria Silva"` and `"João Do Santos"`;
* CPF values represented as pure numeric types.

---

## Learning Goals

The objectives of Exercise 01 — Version 2.0 focus on transforming a linear, isolated program into a professional, organized, and scalable application capable of evolving alongside the Borrow & Brew system.