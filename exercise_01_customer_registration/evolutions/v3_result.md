# Exercise 01 — Customer Registration (Version 3.0)

## Core Concept

Practice Error Handling with `Result` and `Option`.

---

## Technical Objectives

* Replace `.expect()` with safe error handling using `Result<T, E>`.
* Implement multi-layer validation using conditional logic.
* Prevent invalid data from interrupting program execution.
* Use `match` to control application flow based on success and failure scenarios.
* Introduce `Option` through safe access to potentially missing values.

---

## Problem Context

The Borrow & Brew system has gone through a turbulent week.

Yesterday evening, while José was trying to generate the fiscal reports before closing the café, the cashier system suddenly crashed in the middle of service. The reason? One customer hurriedly entered the CPF `"123.456"` (incomplete), while another typed `"abc.def.ghi-jk"`. The old implementation attempted to convert those values directly into numbers using `.parse().expect(...)`. As soon as the conversion failed, the application panicked and shut down.

As if that were not enough, Ana discovered another issue this morning. Some customers, unwilling to provide their real identification documents, began typing obvious keyboard sequences such as `"111.111.111-11"` or `"00000000000"`. Since the previous version only checked whether the CPF could be transformed into a number, those fake values were accepted by the system.

Later that afternoon, the Brazilian tax integration rejected the entire batch of invoices due to invalid CPF records, delaying José's end-of-day closing process.

Your mission in Version 3.0 is to transform the CPF sanitization system into a reliable validation pipeline. Instead of crashing when invalid data appears, the system must safely intercept predictable problems, communicate them clearly to Ana, and continue operating normally.

By the end of this exercise, the Borrow & Brew cashier will be capable of handling mistakes gracefully, rejecting suspicious data, and ensuring that only trustworthy CPF values move forward.

---

## Dummy Data

To demonstrate that the system is now resilient against real-world failures, use the following four test scenarios inside `main`:

* **Case 1 (Success):** `"123.456.789-09"`
* **Case 2 (Yesterday's Customer Error — Invalid Length):** `"123.456"`
* **Case 3 (Yesterday's Customer Error — Letters):** `"123.45a.789-00"`
* **Case 4 (Ana's Discovery — Repeated Sequence):** `"222.222.222-22"`

---

## Mandatory Steps

To solve this problem, you must follow this implementation order in your `main.rs` file:

### Step 1:

Create the CPF Sanitization and Validation Function.

Outside the `main` function, declare the following signature:

```rust
fn sanitize_and_validate_cpf(cpf: &str) -> Result<u64, String>
```

The function must return:

* `Ok(u64)` when the CPF passes all validation rules;
* `Err(String)` when any validation rule fails.

---

### Step 2:

Perform the Initial Sanitization.

Inside the function, remove periods (`.`), hyphens (`-`), and optionally spaces from the incoming CPF using chained `.replace()` calls.

Store the cleaned result in a temporary String variable.

Example:

```rust
let cleaned_cpf = ...
```

---

### Step 3:

Validate the CPF Length.

Before attempting any numeric conversion, verify that the cleaned CPF contains exactly 11 characters.

Use an `if` statement together with `.len()`.

If the length is incorrect, interrupt the function immediately using:

```rust
return Err(String::from(
    "Error: CPF must contain exactly 11 digits.",
));
```

---

### Step 4:

Reject Repeated Sequences.

Implement a validation rule capable of detecting CPF values where all digits are identical.

Examples of invalid sequences include:

```text
00000000000
11111111111
22222222222
99999999999
```

**Hint:** Retrieve the first character of the cleaned String and compare the CPF against eleven repetitions of that same character.

If the CPF matches this pattern, return:

```rust
return Err(String::from(
    "Error: Invalid CPF. Repeated sequences are not allowed.",
));
```

---

### Step 5:

Safely Handle Numeric Conversion.

Replace the previous `.expect()` approach.

Attempt to convert the cleaned CPF into a numeric type using:

```rust
.parse::<u64>()
```

Use a `match` expression to handle both possible outcomes:

* If the result is `Ok(number)`, store the converted value.
* If the result is `Err(_)`, interrupt the function and return:

```rust
return Err(String::from(
    "Error: CPF contains non-numeric characters.",
));
```

This prevents the application from crashing when customers enter letters or invalid symbols.

---

### Step 6:

Emit the Success Result.

If the CPF successfully passes all validation layers, return the final value wrapped inside Rust's success variant:

```rust
Ok(parsed_cpf)
```

---

### Step 7:

Build the Cashier Control Panel in `main`.

Inside the `main` function, process all four CPF test cases sequentially.

Use a `match` expression to decide how the cashier interface should react:

```rust
match sanitize_and_validate_cpf(cpf_case) {
    Ok(cpf) => {
        println!("✅ Success! Valid CPF registered: {}", cpf);
    }
    Err(error_message) => {
        println!("❌ Cashier Alert -> {}", error_message);
    }
}
```

The program must continue processing every scenario, even when some validations fail.

---

### Step 8:

Validate the Final Output.

Confirm that the terminal demonstrates all possible outcomes:

* A successful CPF registration;
* Detection of invalid CPF length;
* Detection of non-numeric characters;
* Detection of repeated sequences.

Most importantly, verify that none of these failures terminate the program unexpectedly.

---

## Learning Goals

The objectives of Exercise 01 — Version 3.0 focus on introducing reliability and resilience into the Borrow & Brew system.