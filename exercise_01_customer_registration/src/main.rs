struct Customer {
    name: String,
    cpf: u64,
    telephone: String,
    loyalty_points: u32,
}

impl Customer {
    fn display_summary(&self) {
        println!("----------------------------------------");
        println!("Customer Summary");
        println!("Name:           {}", self.name);
        println!("CPF:            {}", self.cpf);
        println!("Telephone:      {}", self.telephone);
        println!("Loyalty Points: {}", self.loyalty_points);
        println!("----------------------------------------");
    }

    fn add_loyalty_points(&mut self, points: u32) {
        self.loyalty_points += points;
        println!("{points} loyalty points added successfully.");
    }

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

    let mut maria = Customer {
        name: String::from("Maria Silva"),
        cpf: 12345678900,
        telephone: String::new(),
        loyalty_points: 0,
    };

    println!("Initial Customer State:");
    maria.display_summary();

    maria.add_loyalty_points(50);
    maria.add_loyalty_points(25);

    println!("\nAfter Accumulating Points:");
    maria.display_summary();

    maria.redeem_loyalty_points(30);

    println!("\nAfter Redeeming 30 Points:");
    maria.display_summary();

    maria.redeem_loyalty_points(100);

    println!("\nFinal Customer State:");
    maria.display_summary();
}
