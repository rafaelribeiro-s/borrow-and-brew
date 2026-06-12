/// # Borrow & Brew ☕ — Exercise 01: Customer Registration (Version 3.0)
///
/// This version evolves the customer registration workflow by
/// replacing panic-drive failures with explicit error handling.
///
/// The program demonstrates:
/// - Safe error handling using `Result<T, E>`;
/// - Multi-layer validation pipelines;
/// - Pattern matching with `match`;
/// - The introduction of the `Option` mindset;
/// - Building resilient software capable of handling invalid input.
///
/// Instead of terminating execution when predictable failures occur,
/// the application communicates problems clearly and continues
/// processing subsequent requests, improving reliability at the cashier.
///
// ---------------------------------------------------------
// STEP 01: CPF SANITIZATION AND VALIDATION FUNCTION
// ---------------------------------------------------------
// Receives a formatted CPF string and validates it through
// multiple layers before converting it into a numeric type.
//
// Returns:
// - `Ok(u64)` when all validation rules succeed;
// - `Err(String)` when any validation rule fails.
fn sanitize_and_validate_cpf(cpf: &str) -> Result<u64, String> {
    // ---------------------------------------------------------
    // STEP 02: PERFORM THE INITIAL SANITIZATION
    // ---------------------------------------------------------
    // Remove presentation character to retain only the raw content.
    let cleaned_cpf = cpf.replace(".", "").replace("-", "").replace(" ", "");

    // ---------------------------------------------------------
    // STEP 03: LENGTH VALIDATION
    // ---------------------------------------------------------
    // A CPF must contain exactly 11 digits.
    if cleaned_cpf.len() != 11 {
        return Err(String::from("Error: CPF must contain exactly 11 digits."));
    }

    // ---------------------------------------------------------
    // STEP 04: REPEATED SEQUENCE VALIDATION
    // ---------------------------------------------------------
    // Reject obvious fake values composed of the same digit.
    //
    // `chars().next()` returns an `Option<char>`, introducing
    // the concept that a value may or may not exist.
    if let Some(first_char) = cleaned_cpf.chars().next() {
        let repeated_sequence = first_char.to_string().repeat(11);

        if cleaned_cpf == repeated_sequence {
            return Err(String::from(
                "Error: Invalid CPF. Repeated sequence are not allowed.",
            ));
        }
    }

    // ---------------------------------------------------------
    // STEP 05: SAFELY HANDLE NUMERIC CONVERSION
    // ---------------------------------------------------------
    // Replace the previous `.expect()` approach with explicit
    // pattern matching to prevent application crashes.
    match cleaned_cpf.parse::<u64>() {
        // ---------------------------------------------------------
        // STEP 06: SUCESS EMISSION
        // ---------------------------------------------------------
        // All validation layers succeeded.
        Ok(parsed_cpf) => Ok(parsed_cpf),

        // The CPF contains letters or invalid symbols.
        Err(_) => Err(String::from("Error: CPF contains non-numeric characters.")),
    }
}

// ---------------------------------------------------------
// SANITIZE NAME FUNCTION (VERSION 2.0)
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

    name // Return the sanitized name.
}

fn main() {
    println!("=== BORROW & BREW CASHIER VALIDATION PANEL ===");

    // ---------------------------------------------------------
    // STEP 07: CASHIER CONTROL PANEL
    // ---------------------------------------------------------
    // Simulate real-world validation scenarios encountered
    // during customer registration.
    let test_cases = [
        "123.456.789-09", // Case 1: Success
        "123.456",        // Case 2: Invalid Length
        "123.45a.789-00", // Case 3: Letters
        "222.222.222-22", // Case 4: Repeated Sequence
    ];

    // ---------------------------------------------------------
    // STEP 08: PROCESS EACH TEST CASE
    // ---------------------------------------------------------
    // Continue processing every case regardless of failures,
    // demonstrating that validation errors no longer interrrupt
    // the application's execution flow.
    for (index, cpf_case) in test_cases.iter().enumerate() {
        println!("\nProcessing Case {}:", index + 1);
        println!("Input CPF: '{}'", cpf_case);

        match sanitize_and_validate_cpf(cpf_case) {
            Ok(cpf) => {
                println!("✅ Success! Valid CPF registered: {}", cpf);
            }
            Err(error_message) => {
                println!("❌ Cashier Alert -> {}", error_message);
            }
        }
        println!("--------------------------------------------------");
    }

    // ---------------------------------------------------------
    // CUSTOMER REGISTRATION DEMONSTRATION
    // ---------------------------------------------------------
    // Preserve the original registration scenario from the
    // previous exercises while integrating the new validation flow.
    let customer_name = "  mArIa sIlVa  ";
    let customer_cpf = "123.456.789-00";

    println!("=== ORIGINAL DATA ===");
    println!("Raw name: '{}'", customer_name);
    println!("Raw CPF: '{}'", customer_cpf);

    // Delegate responsibilites to dedicated functions.
    let sanitized_name = sanitize_name(customer_name);
    let sanitized_cpf: Result<u64, String> = sanitize_and_validate_cpf(customer_cpf);

    // ---------------------------------------------------------
    // FINAL OUTPUT & VALIDATION
    // ---------------------------------------------------------
    // Demonstrate that customer registration now benefits
    // from both smart capitalization and resilient CPF validation
    println!("\n=== CUSTOMER REGISTRATION DEMO ===");
    println!("Final name: {}", sanitized_name);

    match sanitized_cpf {
        Ok(number) => {
            println!("CPF accepted by current validation rules: {}", number);
        }
        Err(error) => {
            println!("CPF rejected: {}", error);
        }
    }
}
