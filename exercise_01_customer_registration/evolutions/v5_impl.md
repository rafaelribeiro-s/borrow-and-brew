# Exercise 01 — Customer Registration (Version 5.0)

## Core Concept

Practice Methods and Behavior using `impl`.

---

## Technical Objectives

* Introduce implementation blocks (`impl`) in Rust.
* Associate behavior with domain entities.
* Learn the difference between immutable (`&self`) and mutable (`&mut self`) method receivers.
* Encapsulate business rules inside the `Customer` type.
* Strengthen Domain Modeling by moving from data containers to active entities.

---

## Problem Context

The Borrow & Brew loyalty program has become a success.

After organizing customer information using the `Customer` struct, José finally launched the loyalty initiative he had been planning for months. Customers now earn points with every purchase and can redeem them for discounts and complimentary beverages.

However, a new problem emerged.

Ana noticed that every time a customer earned points, the cashier software manually modified the fields directly:

```rust id="9ih7nl"
customer.loyalty_points += 10;
```

Whenever someone redeemed rewards, another piece of code appeared elsewhere:

```rust id="r6r4bi"
customer.loyalty_points -= 50;
```

As the business continued to grow, João became concerned. The rules of the loyalty system were spreading throughout the codebase. Different developers could accidentally implement them differently, leading to inconsistencies and bugs.

José proposed a new approach:

> "A customer should know how to manage their own loyalty program."

Instead of treating `Customer` as a passive collection of fields, the system should attach behaviors directly to the entity itself.

Your mission in Version 5.0 is to teach the `Customer` struct how to perform actions through methods defined inside an `impl` block.

By the end of this exercise, customers in the Borrow & Brew system will no longer be simple records. They will become active participants capable of managing their own loyalty state.

---

## Dummy Data

Use the following customer as the primary scenario for this exercise:

* Name: `"Maria Silva"`
* CPF: `12345678909`
* Telephone: `""`
* Loyalty Points: `0`

Throughout the exercise, Maria Silva will interact with the loyalty system.

Use the following operations in sequence:

1. Maria earns **50 loyalty points**.
2. Maria earns **25 additional points**.
3. Maria redeems **30 points**.
4. Attempt to redeem **100 points**, even though she does not have enough balance.

---

## Mandatory Steps

To solve this problem, you must follow this implementation order in your `main.rs` file:

### Step 1:

Reuse the `Customer` Struct.

Start from the domain model created in Version 4.0.

Ensure your structure contains the following fields:

```rust id="xn8s4w"
struct Customer {
    name: String,
    cpf: u64,
    telephone: String,
    loyalty_points: u32,
}
```

---

### Step 2:

Create an Implementation Block.

Outside the `main` function, define an `impl` block associated with `Customer`.

Example:

```rust id="vg7mws"
impl Customer {
    // Methods go here.
}
```

This block will contain the behaviors that belong to the customer entity.

---

### Step 3:

Implement a Summary Method.

Create a method capable of displaying a customer summary.

Signature:

```rust id="d1utw7"
fn display_summary(&self)
```

The method should print the customer's information, including:

* Name;
* CPF;
* Telephone;
* Loyalty Points.

**Hint:** Use `&self` because this method only reads data and does not modify the customer.

---

### Step 4:

Implement the Loyalty Accumulation Method.

Create a method that allows customers to earn points.

Signature:

```rust id="5zwy5h"
fn add_loyalty_points(&mut self, points: u32)
```

Inside the method:

* Increase the customer's loyalty points balance.
* Display a confirmation message showing how many points were added.

Example:

```text id="jq11m8"
50 loyalty points added successfully.
```

**Hint:** Use `&mut self` because the method changes the internal state of the customer.

---

### Step 5:

Implement the Loyalty Redemption Method.

Create a method responsible for redeeming points.

Signature:

```rust id="x4w8tp"
fn redeem_loyalty_points(&mut self, points: u32)
```

Implement the following business rule:

* If the customer has enough points, subtract the requested amount.
* Otherwise, display an alert indicating insufficient balance.

Example outputs:

```text id="wjlwmh"
30 loyalty points redeemed successfully.
```

or

```text id="6vfzmy"
Insufficient loyalty points for redemption.
```

This step introduces decision-making inside domain behaviors.

---

### Step 6:

Instantiate the Customer.

Inside `main`, create Maria Silva using the `Customer` struct.

Initialize:

* Name: `"Maria Silva"`
* CPF: `12345678909`
* Telephone: `String::new()`
* Loyalty Points: `0`

---

### Step 7:

Execute the Loyalty Workflow.

Using Maria's instance, perform the following sequence:

1. Display the initial summary.
2. Add 50 points.
3. Add 25 points.
4. Display the updated summary.
5. Redeem 30 points.
6. Display the updated summary.
7. Attempt to redeem 100 points.
8. Display the final summary.

Observe how the same entity changes over time through its own methods.

---

### Step 8:

Validate the Final Output.

Confirm that the terminal demonstrates:

* The customer's initial state;
* Successful point accumulation;
* Successful redemption;
* Prevention of invalid redemption attempts;
* The customer's final loyalty balance.

The final output should clearly illustrate that behaviors are now encapsulated inside the domain entity.

---

## Learning Goals

The objectives of Exercise 01 — Version 5.0 focus on transforming entities from passive data holders into active participants of the business domain.