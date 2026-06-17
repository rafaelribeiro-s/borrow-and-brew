/// # Borrow & Brew ☕ — Exercise 01: Customer Registration (Version 5.0)
///
/// This version evolves the customer domain model by introducing
/// behavior through Rust's implementation blocks (`impl`).
///
/// The program demonstrates:
/// - Associating behavior with data using `impl`;
/// - Immutable methods through `&self`;
/// - Mutable methods through `&mut self`;
/// - Encapsulation of business rules;
/// - Domain entities capable of managing their own state.
///
/// Previous versions focused on:
/// - V1: Data sanitization through Shadowing;
/// - V2: Code reuse through Functions;
/// - V3: Error handling through Result and Option;
/// - V4: Domain Modeling through Structs.
///
/// In Version 5.0, the Customer entity evolves from a passive
/// data container into an active domain object capable of
/// performing operations on itself.
///
/// This concludes Exercise 01 — Customer Registration.
///
/// ---------------------------------------------------------
/// STEP 01: CUSTOMER DOMAIN MODEL
/// ---------------------------------------------------------
/// Represents a customer registered in the Borrow & Brew
/// loyalty program.
///
/// The structure groups all customer-related information
/// into a single domain entity.
struct Customer {
    name: String,
    cpf: u64,
    telephone: String,
    loyalty_points: u32,
}

// ---------------------------------------------------------
// STEP 02: CUSTOMER IMPLEMENTATION BLOCK
// ---------------------------------------------------------
// Associate behavior directly with the Customer entity.
//
// Instead of modifying fields throughout the application,
// all customer-related actions are centralized here.
impl Customer {
    // ---------------------------------------------------------
    // STEP 03: CUSTOMER SUMMARY METHOD
    // ---------------------------------------------------------
    // Displays the current state of the customer.
    //
    // Uses `&self` because the method only reads data and
    // does not modify the entity.
    fn display_summary(&self) {
        println!("----------------------------------------");
        println!("Customer Summary");
        println!("Name:           {}", self.name);
        println!("CPF:            {}", self.cpf);
        println!("Telephone:      {}", self.telephone);
        println!("Loyalty Points: {}", self.loyalty_points);
        println!("----------------------------------------");
    }

    // ---------------------------------------------------------
    // STEP 04: LOYALTY ACCUMULATION METHOD
    // ---------------------------------------------------------
    // Adds loyalty points to the customer's balance.
    //
    // Uses `&mut self` because the internal state of the
    // customer is modified.
    fn add_loyalty_points(&mut self, points: u32) {
        self.loyalty_points += points;

        println!("{points} loyalty points added successfully.");
    }

    // ---------------------------------------------------------
    // STEP 05: LOYALTY REDEMPTION METHOD
    // ---------------------------------------------------------
    // Allows the customer to redeem loyalty points.
    //
    // Business Rule:
    // - Redemption succeeds only if sufficient points exist.
    // - Otherwise, the operation is rejected.
    fn redeem_loyalty_points(&mut self, points: u32) {
        if self.loyalty_points >= points {
            self.loyalty_points -= points;

            println!("{points} loyalty points redeemed successfully.");
        } else {
            println!("Insufficient loyalty points for redemption.");
        }
    }
}

fn main() {
    println!("=== BORROW & BREW — CUSTOMER REGISTRATION SYSTEM (v5.0) ===\n");

    // ---------------------------------------------------------
    // STEP 06: CUSTOMER INSTANTIATION
    // ---------------------------------------------------------
    // Create Maria Silva as a fully modeled domain entity.
    //
    // The customer starts with zero loyalty points and
    // an empty telephone field.
    let mut maria = Customer {
        name: String::from("Maria Silva"),
        cpf: 12345678900,
        telephone: String::new(),
        loyalty_points: 0,
    };

    // ---------------------------------------------------------
    // STEP 07: INITIAL STATE
    // ---------------------------------------------------------
    // Display the customer before any loyalty operations.
    println!("Initial Customer State:");
    maria.display_summary();

    // ---------------------------------------------------------
    // STEP 08: LOYALTY POINT ACCUMULATION
    // ---------------------------------------------------------
    // Maria earns points through purchases.
    maria.add_loyalty_points(50);
    maria.add_loyalty_points(25);

    println!("\nAfter Accumulating Points:");
    maria.display_summary();

    // ---------------------------------------------------------
    // STEP 09: SUCCESSFUL REDEMPTION
    // ---------------------------------------------------------
    // Maria redeems part of her available balance.
    maria.redeem_loyalty_points(30);

    println!("\nAfter Redeeming 30 Points:");
    maria.display_summary();

    // ---------------------------------------------------------
    // STEP 10: FAILED REDEMPTION ATTEMPT
    // ---------------------------------------------------------
    // Attempt to redeem more points than available.
    //
    // The business rule inside the Customer entity
    // prevents the operation.
    maria.redeem_loyalty_points(100);

    // ---------------------------------------------------------
    // STEP 11: FINAL CUSTOMER STATE
    // ---------------------------------------------------------
    // Display the final state after all operations.
    println!("\nFinal Customer State:");
    maria.display_summary();
}
