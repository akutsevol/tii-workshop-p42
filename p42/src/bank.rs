use std::collections::HashMap;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::fmt;

#[derive(Debug, Clone)]
pub struct User {
    name: String,
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
    users: HashMap<String, User>,
    name: String,
    credit_interest: u64, // in basis points (0.01%)
    debit_interest: u64,  // in basis points (0.01%)
}

impl Bank {
    pub fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self {
            users: HashMap::new(),
            name,
            credit_interest,
            debit_interest,
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.name.clone(), user);
    }

    #[allow(dead_code)]
    pub fn calc_balance(&self) -> (u64, u64) {
        let (mut liabilities, mut assets) = (0, 0);
        for user in self.users.values() {
            if user.balance < 0 {
                liabilities += user.balance.abs() as u64;
            } else {
                assets += user.balance as u64;
            }
        }
        (liabilities, assets)
    }

    #[allow(dead_code)]
    pub fn transfer_funds(&mut self, from_user: &str, to_user: &str, amount: u64) -> Result<(), String> {
        let amount_i64: i64 = amount.try_into().map_err(|_| "Amount conversion error".to_string())?;

        // Check if the users exist and get their balances
        let from_balance = self.users.get(from_user).map(|u| u.balance).ok_or_else(|| format!("User '{}' not found", from_user))?;
        let to_balance = self.users.get(to_user).map(|u| u.balance).ok_or_else(|| format!("User '{}' not found", to_user))?;

        // Check if the transfer is possible
        if from_balance < amount_i64 {
            return Err(format!("User '{}' does not have sufficient funds", from_user));
        }

        if to_balance.checked_add(amount_i64).is_none() {
            return Err("Transfer amount would cause overflow for recipient".to_string());
        }

        // Perform the mutable borrows and update the balances
        {
            let from_user = self.users.get_mut(from_user).ok_or_else(|| format!("User '{}' not found", from_user))?;
            from_user.balance -= amount_i64;
        }

        {
            let to_user = self.users.get_mut(to_user).ok_or_else(|| format!("User '{}' not found", to_user))?;
            to_user.balance += amount_i64;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn accrue_interest(&mut self) {
        for user in self.users.values_mut() {
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
        for (name, other_user) in other_bank.users.drain() {
            if let Some(user) = self.users.get_mut(&name) {
                user.add_balance(other_user.get_balance());
            } else {
                self.users.insert(name, other_user);
            }
        }
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bank: {}\nUsers: {:?}\nCredit Interest: {} basis points\nDebit Interest: {} basis points", 
                self.name, self.users.values().collect::<Vec<_>>(), self.credit_interest, self.debit_interest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut bank = Bank::new("Test Bank".to_string(), 100, 200);
        let user = User::new("Alice".to_string(), 1000, 500);
        bank.add_user(user);

        assert!(bank.users.contains_key("Alice"));
    }

    #[test]
    fn test_calc_balance() {
        let mut bank = Bank::new("Test Bank".to_string(), 100, 200);
        bank.add_user(User::new("Alice".to_string(), 1000, 500));
        bank.add_user(User::new("Bob".to_string(), 1000, -300));

        let (liabilities, assets) = bank.calc_balance();
        assert_eq!(liabilities, 300);
        assert_eq!(assets, 500);
    }

    #[test]
    fn test_transfer_funds() {
        let mut bank = Bank::new("Test Bank".to_string(), 100, 200);
        bank.add_user(User::new("Alice".to_string(), 1000, 500));
        bank.add_user(User::new("Bob".to_string(), 1000, 0));

        assert!(bank.transfer_funds("Alice", "Bob", 200).is_ok());
        assert_eq!(bank.users.get("Alice").unwrap().balance, 300);
        assert_eq!(bank.users.get("Bob").unwrap().balance, 200);

        assert!(bank.transfer_funds("Alice", "Bob", 400).is_err());
        assert_eq!(bank.users.get("Alice").unwrap().balance, 300);
        assert_eq!(bank.users.get("Bob").unwrap().balance, 200);
    }

    #[test]
    fn test_accrue_interest() {
        let mut bank = Bank::new("Test Bank".to_string(), 100, 200);
        bank.add_user(User::new("Alice".to_string(), 1000, 500));
        bank.add_user(User::new("Bob".to_string(), 1000, -300));

        bank.accrue_interest();
        assert_eq!(bank.users.get("Alice").unwrap().balance, 505); // 1% of 500 is 5
        assert_eq!(bank.users.get("Bob").unwrap().balance, -306); // 2% of 300 is 6
    }

    #[test]
    fn test_merge_bank() {
        let mut bank1 = Bank::new("Bank1".to_string(), 100, 200);
        let mut bank2 = Bank::new("Bank2".to_string(), 100, 200);

        bank1.add_user(User::new("Alice".to_string(), 1000, 500));
        bank2.add_user(User::new("Bob".to_string(), 1000, 200));
        bank2.add_user(User::new("Alice".to_string(), 1000, 300));

        bank1.merge_bank(&mut bank2);

        assert_eq!(bank1.users.get("Alice").unwrap().balance, 800); // 500 + 300
        assert_eq!(bank1.users.get("Bob").unwrap().balance, 200);
        assert!(bank2.users.is_empty());
    }
}
