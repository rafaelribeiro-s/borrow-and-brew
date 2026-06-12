struct Customer {
    name: String,
    cpf: u64,
    telephone: String,
    loyalty_points: u32,
}

fn sanitize_and_validate_cpf(cpf: &str) -> Result<u64, String> {
    let cleaned_cpf = cpf.replace(".", "").replace("-", "").replace(" ", "");

    if cleaned_cpf.len() != 11 {
        return Err(String::from("Error: CPF must contain exactly 11 digits."));
    }

    if let Some(first_char) = cleaned_cpf.chars().next() {
        let repeated_sequence = first_char.to_string().repeat(11);

        if cleaned_cpf == repeated_sequence {
            return Err(String::from(
                "Error: Invalid CPF. Repeated sequence are not allowed.",
            ));
        }
    }

    match cleaned_cpf.parse::<u64>() {
        Ok(parsed_cpf) => Ok(parsed_cpf),
        Err(_) => Err(String::from("Error: CPF contains non-numeric characters.")),
    }
}

fn sanitize_name(name: &str) -> String {
    let name = name
        .trim()
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();

            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    name // Return Variable
}

fn main() {
    println!("=== BORROW & BREW — CUSTOMER REGISTRATION SYSTEM (v4.0) ===\n");

    let raw_name_maria = "  mArIa sIlVa  ";
    let raw_cpf_maria = "123.456.789-00";

    let sanitized_name_maria = sanitize_name(raw_name_maria);
    let validated_cpf_maria = sanitize_and_validate_cpf(raw_cpf_maria);

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

    let customer2 = Customer {
        name: String::from("Pedro Santos"),
        cpf: 98765432101,
        telephone: String::new(),
        loyalty_points: 0,
    };

    println!("== CUSTOMER REGISTRY REPORT ===");

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
