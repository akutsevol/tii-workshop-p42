mod bank;

fn main() {
    use bank::{Bank, User};

    let user1 = User::new("Alice".to_string(), 1000, 0);
    let user2 = User::new("Bob".to_string(), 2000, 500);
    
    let mut my_bank = Bank::new("MyBank".to_string(), 10, 20);
    my_bank.add_user(user1.clone());
    my_bank.add_user(user2.clone());

    let user3 = User::new("Charlie".to_string(), 3000, 1000);
    let user4 = User::new("Alice".to_string(), 1000, 200);

    let mut other_bank = Bank::new("OtherBank".to_string(), 5, 15);
    other_bank.add_user(user3.clone());
    other_bank.add_user(user4.clone());

    println!("Initial State of MyBank:");
    println!("{}", my_bank);

    println!("Initial State of OtherBank:");
    println!("{}", other_bank);

    my_bank.merge_bank(&mut other_bank);

    println!("State of MyBank After Merge:");
    println!("{}", my_bank);

    println!("State of OtherBank After Merge:");
    println!("{}", other_bank);
}
