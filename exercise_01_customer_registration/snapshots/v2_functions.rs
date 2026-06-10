/// # Borrow & Brew ☕ — Exercise 01: Customer Registration (Version 2.0)
///
/// This version evolves the original implementation by introducing
/// reusable functions and separation of responsibilities.
///
/// The program demonstrates:
/// - The DRY (Don't Repeat Yourself) principle;
/// - Function definitions with parameters and return types;
/// - Modular design through dedicated responsibilities;
/// - Advanced String manipulation using smart capitalization.
///
/// Raw customer input is transformed into clean, production-ready data
/// While keeping `main` simple and focused on application flow.
///
// ---------------------------------------------------------
// STEP 01: CPF SANITIZATION FUNCTION
// ---------------------------------------------------------
// Receives a formatted CPF string, removes presentation
// characters, and converts the result into a numeric type.
fn sanitize_cpf(cpf: &str) -> u64 {
    // Strip formatting punctuation and parse the remaining digits.
    let cpf: u64 = cpf
        .replace(".", "")
        .replace("-", "")
        .parse()
        .expect("Fatal: Failed to parse sanitized CPF into a numeric type.");

    cpf // Return variable
}

// ---------------------------------------------------------
// STEP 02 & 03: NAME SANITIZATION FUNCTION
// ---------------------------------------------------------
// Cleans the customer's name, standardizes casing,
// and applies smart capitalization so that each word
// starts with an uppercase letter.
fn sanitize_name(name: &str) -> String {
    let name = name
        .trim() // Remove leading and trailing whitespaces.
        .to_lowercase() // Standardize the entire string to lowercase.
        .split_whitespace() // Split the name into individual words.
        // Capitalize the first letter of each word.
        .map(|word| {
            let mut chars = word.chars();

            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        // Reassemble the transformed words into a single String.
        .collect::<Vec<String>>()
        .join(" ");

    name // Return variable
}

fn main() {
    // ---------------------------------------------------------
    // STEP 04: RAW DATA INPUT (FIRST CUSTOMER)
    // ---------------------------------------------------------
    // Simulates "dirty" inputs typed quickly by Ana during peak hours.
    let customer_name = "  mArIa sIlVa  ";
    let customer_cpf = "123.456.789-00";

    println!("=== ORIGINAL DATA ===");
    println!("Raw name: '{}'", customer_name);
    println!("Raw CPF: '{}'", customer_cpf);

    // ---------------------------------------------------------
    // STEP 05: PROCESS CUSTOMER DATA USING FUNCTIONS
    // ---------------------------------------------------------
    // `main` delegates the sanitization rules to dedicated functions.
    let sanitized_name = sanitize_name(customer_name);
    let sanitized_cpf = sanitize_cpf(customer_cpf);

    // ---------------------------------------------------------
    // STEP 06: FINAL OUTPUT & VALIDATION (FIRST CUSTOMER)
    // ---------------------------------------------------------
    println!("\n=== SANITIZED DATA ===");
    println!("Final name: {}", sanitized_name);
    println!("Final CPF: {}", sanitized_cpf);

    // ---------------------------------------------------------
    // STEP 07: DEMONSTRATE REUSABILITY
    // ---------------------------------------------------------
    // Process a second customer using the exact same functions,
    // without duplicating any sanitization logic.
    let customer2_name = "  jOãO do sAnToS  ";
    let customer2_cpf = "987.654.321-11";

    let sanitized_name2 = sanitize_name(customer2_name);
    let sanitized_cpf2 = sanitize_cpf(customer2_cpf);

    // ---------------------------------------------------------
    // STEP 08: FINAL OUTPUT & VALIDATION (SECOND CUSTOMER)
    // ---------------------------------------------------------
    println!("\n=== SECOND CUSTOMER ===");
    println!("Raw name: '{}'", customer2_name);
    println!("Raw CPF: '{}'", customer2_cpf);

    println!("\nFinal name: {}", sanitized_name2);
    println!("Final CPF: {}", sanitized_cpf2);
}
