/// # Borrow & Brew ☕ — Exercise 01: Customer Registration
///
/// This program demonstrates the core concept of **Shadowing** in Rust.
/// It simulates a real-world scenario where raw, poorly formatted customer input
/// from the cashier interface is progressively sanitized and transformed into clean,
/// production-ready data types without creating unnecessary temporary variables.
fn main() {
    // ---------------------------------------------------------
    // STEP 01: RAW DATA INPUT
    // ---------------------------------------------------------
    // Simulates "dirty" inputs typed quickly by Ana during peak hours.
    let customer_name = "  mArIa sIlVa  ";
    let customer_cpf = "123.456.789-00";

    println!("=== ORIGINAL DATA ===");
    println!("Raw name: '{}'", customer_name);
    println!("Raw CPF: '{}'", customer_cpf);

    // ---------------------------------------------------------
    // STEP 02: NAME SANITIZATION - First Shadowing
    // ---------------------------------------------------------
    // .trim() returns a string slice (&str) removing leading/trailing whitespaces.
    let customer_name = customer_name.trim();

    // ---------------------------------------------------------
    // STEP 03: NAME SANITIZATION - Second Shadowing
    // ---------------------------------------------------------
    // .to_lowercase() allocates a new String with standardized casing.
    // Shadowing allows us to reuse the same variable name instead of 'clean_name'.
    let customer_name = customer_name.to_lowercase();

    // ---------------------------------------------------------
    // STEP 04: CPF SANITIZATION - Shadowing Replacement
    // ---------------------------------------------------------
    // Chain replacements to strip formatting puncuation out of the string.
    let customer_cpf = customer_cpf.replace(".", "").replace("-", "");

    // ---------------------------------------------------------
    // STEP 05: CPF SANITIZATION - Shadowing Type Mutation
    // ---------------------------------------------------------
    // Shadowing allows changing the variable type completely.
    // I transform the cleaned text into a 64-bit unsigned integer (u64)
    // to optimize database storage, as integer consume less meomory than strings.
    let customer_cpf: u64 = customer_cpf
        .parse()
        .expect("Fatal: Failed to parse sanitized CPF into a numeric type.");

    // ---------------------------------------------------------
    // STEP 06: FINAL OUTPUT & VALIDATION
    // ---------------------------------------------------------
    println!("\n=== SANITIZED DATA ===");
    println!("Final name: {customer_name}");
    println!("Final CPF: {customer_cpf}");

    // Mathematical proof to ensure the compuler successfully mutated the type to u64
    println!("Type Verification (CPF + 0): {}", customer_cpf + 0);
}
