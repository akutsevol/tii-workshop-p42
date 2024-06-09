use std::fmt;

#[derive(Debug, Clone)]
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64, // Positive number means debit, negative credit
}

impl User {
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self {
            name,
            credit_line,
            balance,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_balance(&self) -> i64 {
        self.balance
    }

    pub fn add_balance(&mut self, amount: i64) {
        self.balance += amount;
    }
}

#[derive(Debug, Clone)]
pub struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64, // in basis points (0.01%)
    debit_interest: u64,  // in basis points (0.01%)
}

impl Bank {
    pub fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self {
            users: Vec::new(),
            name,
            credit_interest,
            debit_interest,
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn calc_balance(&self) -> (u64, u64) {
        let (mut liabilities, mut assets) = (0, 0);
        for user in &self.users {
            if user.balance < 0 {
                liabilities += user.balance.abs() as u64;
            } else {
                assets += user.balance as u64;
            }
        }
        (liabilities, assets)
    }

    pub fn transfer_funds(&mut self, from_user: &str, to_user: &str, amount: u64) -> Result<(), String> {
        let from_user_index = self.users.iter().position(|u| u.name == from_user);
        let to_user_index = self.users.iter().position(|u| u.name == to_user);
        
        if let (Some(from_index), Some(to_index)) = (from_user_index, to_user_index) {
            let from_balance = self.users[from_index].balance;
            let to_balance = self.users[to_index].balance;

            if from_balance < amount as i64 {
                return Err(format!("User '{}' does not have sufficient funds", from_user));
            }

            if to_balance.checked_add(amount as i64).is_none() {
                return Err("Transfer amount would cause overflow for recipient".to_string());
            }

            self.users[from_index].balance -= amount as i64;
            self.users[to_index].balance += amount as i64;

            Ok(())
        } else {
            Err("One or both users not found".to_string())
        }
    }

    pub fn accrue_interest(&mut self) {
        for user in &mut self.users {
            if user.balance < 0 {
                let interest = (-user.balance as f64 * self.debit_interest as f64 / 10_000.0) as i64;
                user.balance -= interest;
            } else {
                let interest = (user.balance as f64 * self.credit_interest as f64 / 10_000.0) as i64;
                user.balance += interest;
            }
        }
    }

    pub fn merge_bank(&mut self, other_bank: &mut Bank) {
        for other_user in &other_bank.users {
            if let Some(user) = self.users.iter_mut().find(|u| u.name == other_user.name) {
                user.add_balance(other_user.get_balance());
            } else {
                self.add_user(other_user.clone());
            }
        }
        other_bank.users.clear();
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bank: {}\nUsers: {:?}\nCredit Interest: {} basis points\nDebit Interest: {} basis points", 
                self.name, self.users, self.credit_interest, self.debit_interest)
    }
}