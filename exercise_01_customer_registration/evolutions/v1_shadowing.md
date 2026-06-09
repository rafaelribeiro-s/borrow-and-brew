# Exercise 01 — Customer Registration

## Core Concept

Practice Shadowing.

---

## Technical Objectives

* Transform data step by step.
* Clean Strings.
* Convert a String into a number.
* Demonstrate type mutation using Shadowing.

---

## Problem Context

Ana just sat down with José to test the very first screen of the Borrow & Brew system. She reports that during peak hours, when registering customers for the loyalty program and tax invoice generation, she types everything very quickly.

The first customer in line was "  mArIa sIlVa  ", who provided the CPF "123.456.789-00". On the cashier's keyboard, the name was entered with invisible spaces at the edges and a messy mix of uppercase and lowercase letters. Additionally, the CPF came in as text full of periods and hyphens.

If this data is saved as is, João won't be able to read the correct name on the kitchen ticket, and the tax invoice system will reject the CPF/ID.

Your mission as the Borrow & Brew developer is to use Rust's Shadowing feature to process and sanitize these variables step by step, without creating messy temporary variables (like name1, name2, clean_name), keeping the code clean and safe.

---

## Dummy Data

To simulate the typing error in your code, use exactly these initial variables:

    * Raw name typed: "  mArIa sIlVa  "

    * Raw CPF typed: "123.456.789-00"

---

## Mandatory Steps

To solve Ana's problem, you must follow this implementation order in your main.rs file:

### Step 1:
The Raw Input. Declare the two initial immutable variables (let) using exactly the dummy data provided above. Print them to the terminal using `println!` to see their "dirty" state.

### Step 2:
First Shadowing of the Name (Trim Spaces). Redeclare the name variable using `let` (applying Shadowing). Use a native Rust method to remove leading and trailing whitespaces (Hint: look for the `.trim()` method in Rust's String documentation).

### Step 3:
Second Shadowing of the Name (Standardize Casing). Redeclare the name variable once more with `let`. Now, take the previous result and convert the entire text to lowercase (Hint: look for the `.to_lowercase()` method). Print the result to ensure it became "maria silva".

### Step 4:
First Shadowing of the CPF/ID (Cleaning Characters). Now, let's move on to the CPF. Redeclare the CPF/ID variable using `let`. Use Rust methods to replace or remove the periods `.` and the hyphen `-` from the text, leaving only numbers. (Hint: you can chain the `.replace(".", "")` and `.replace("-", "")` methods).

### Step 5:
Second Shadowing of the CPF (Type Mutation - The Power of Shadowing). To prove the usefulness of Shadowing, Ana needs the CPF/ID to become a 64-bit integer (`i64` or `u64`), as numbers occupy less space in the database than text. Redeclare the CPF variable using `let` and convert the pure numeric text from the previous step into a numeric type (Hint: use the `.parse().expect(...)` methods).

### Step 6:
The Final Delivery to Ana and João. Use `println!` to display the final result on the screen. Show the perfectly sanitized name and prove that the CPF/ID is now an actual number by performing a simple mathematical operation with it (like adding `+ 0` or displaying its type) to ensure the compiler accepted the type mutation.