/// # Borrow & Brew ☕ — Exercise 01: Customer Registration (Version 4.0)
///
/// This version evolves the customer registration workflow by
/// introducing Domain Modeling through Rust's `struct`.
///
/// The program demonstrates:
/// - Modeling real-world entities using custom types;
/// - Grouping related data into cohesive records;
/// - Integrating previous sanitization and validation logic;
/// - Combining `Result` and `Option` with domain entities;
/// - Preparing the application architecture for future growth.
///
/// Instead of managing scattered variables throughout `main`,
/// the application now treats a customer as a single business
/// entity containing its own characteristics and state.
///
// ---------------------------------------------------------
// STEP 01: CUSTOMER DOMAIN MODEL
// ---------------------------------------------------------
// Represents a real-world customer inside the Borrow & Brew
// domain by grouping all related information into a single type.
struct Customer {
    name: String,
    cpf: u64,
    telephone: String,
    loyalty_points: u32,
}

// ---------------------------------------------------------
// CPF SANITIZATION AND VALIDATION FUNCTION (VERSION 3.0)
// ---------------------------------------------------------
// Receives a fromatted CPF string and validates it through
// multiple layers before converting it into a numeric type.
//
// Returns:
// - `Ok(u64)` when all validation rules succeed;
// - `Err(String)` when any validation rule fails.
fn sanitize_and_validate_cpf(cpf: &str) -> Result<u64, String> {
    // Remove presentation characters to retain only digits.
    let cleaned_cpf = cpf.replace(".", "").replace("-", "").replace(" ", "");

    // ---------------------------------------------------------
    // LENGTH VALIDATION
    // ---------------------------------------------------------
    // A CPF mus contain exactly 11 digits.
    if cleaned_cpf.len() != 11 {
        return Err(String::from("Error: CPF must contain exactly 11 digits."));
    }

    // ---------------------------------------------------------
    // REPEATED SEQUENCE VALIDATION
    // ---------------------------------------------------------
    // Reject obvious fake values composed entirely
    // of the same repeated digit
    //
    // `chars().next()` returns an `Option<char>`,
    // acknowleding that a value may or may not exist.
    if let Some(first_char) = cleaned_cpf.chars().next() {
        let repeated_sequence = first_char.to_string().repeat(11);

        if cleaned_cpf == repeated_sequence {
            return Err(String::from(
                "Error: Invalid CPF. Repeated sequence are not allowed.",
            ));
        }
    }

    // ---------------------------------------------------------
    // NUMERIC CONVERSION
    // ---------------------------------------------------------
    // Safely convert the CPF into a numeric type.
    match cleaned_cpf.parse::<u64>() {
        Ok(parsed_cpf) => Ok(parsed_cpf),

        Err(_) => Err(String >> from("Error: CPF contains non-numeric characters.")),
    }
}

// ---------------------------------------------------------
// NAME SANITIZATION FUNCTION (VERSION 2.0)
// ---------------------------------------------------------
// Cleans the customer's name, standardizes casing,
// and applies smart capitalization so that each word
// starts with an uppercase letter.
fn sanitize_name(name: &str) -> String {
    let name = name
        .trim() // Remove surrounding whitespaces.
        .to_lowercase() // Standardize casing.
        .split_whitespace() // Split into individual words.
        .map(|word| {
            let mut chars = word.chars();

            match chars.next() {
                // Capitalize the first character.
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),

                None => String::new(),
            }
        })
        // Reassemble the transformed words.
        .collect::<Vec<String>>()
        .join(" ");

    name //Return the sanitized name.
}

fn main() {
    println!("=== BORROW & BREW - CUSTOMER REGISTRATION SYSTEM (v4.0) ===\n");

    // ---------------------------------------------------------
    // STEP 02: RAW CUSTOMER INPUT
    // ---------------------------------------------------------
    // Simulate the original customer registration scenario.
    let raw_name_maria = "  mArIa sIlVa  ";
    let raw_cpf_maria = "123.456.789-00";

    // ---------------------------------------------------------
    // STEP 03: PROCESS TRUSTED DATA
    // ---------------------------------------------------------
    // Delegate sanitization and validation responsibilities
    // to the dedicated functions built in previous versions.
    let sanitized_name_maria = sanitize_name(raw_name_maria);

    let validated_cpf_maria = sanitize_and_validate_cpf(raw_cpf_maria);

    // ---------------------------------------------------------
    // STEP 04: INSTANTIATE THE FIRST CUSTOMER
    // ---------------------------------------------------------
    // Attempt to transform validated data into a real
    // domain entity.
    //
    // Since CPF validation may fail, the customer itself
    // becomes optional:
    //
    // - `Some(Customer)` -> registration succeeded;
    // - `None`           -> registration failed.
    let customer1 = match validated_cpf_maria {
        Ok(cpf_num) => Some(Customer {
            name: sanitized_name_maria,
            cpf: cpf_num,
            telephone: String::new(),
            loyalty_points: 0,
        }),

        Err(err) => {
            println!("❌ Critical failure while registering Maria Silva: {}", err);
            None
        }
    };

    // ---------------------------------------------------------
    // STEP 05: CREATE A SECOND CUSTOMER
    // ---------------------------------------------------------
    // Demonstrate that multiple instances of the same
    // domain type can coexist independently.
    let customer2 = Customer {
        name: String::from("Pedro Santos"),
        cpf: 98765432101,
        telephone: String::new(),
        loyalty_points: 0,
    };

    // ---------------------------------------------------------
    // STEP 06: CUSTOMER REGISTRY REPORT
    // ---------------------------------------------------------
    // Display all successfully registered customers.
    println!("=== CUSTOMER REGISTRY REPORT ===");

    // Access the fields of the first customer through
    // dot notation if registration succeeded.
    if let Some(c1) = customer1 {
        println!("--------------------------------------------------");
        println!("Customer 1 (Dynamically Validated):");

        println!("Name:           {}", c1.name);
        println!("CPF:            {}", c1.cpf);
        println!("Telephone:      {}", c1.telephone);
        println!("Loyalty Points: {}", c1.loyalty_points);
    }

    println!("--------------------------------------------------");
    println!("Customer 2 (Static):");

    println!("Name:           {}", customer2.name);
    println!("CPF:            {}", customer2.cpf);
    println!("Telephone:      {}", customer2.telephone);
    println!("Loyalty Points: {}", customer2.loyalty_points);

    println!("--------------------------------------------------");
}
