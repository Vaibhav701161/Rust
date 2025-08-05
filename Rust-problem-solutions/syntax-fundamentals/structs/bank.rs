//13. Create a `BankAccount` struct with deposit, withdraw, and balance inquiry methods.

struct BankAccount{
    balance: f64,
}

impl BankAccount{
    fn deposit(&mut self, amount: f64){
        self.balance+=amount;
    }

    fn withdraw (&mut self, amount:f64){
        if amount > self.balance{
            println!("Insufficient balance");
        } else{
            self.balance-=amount;
        }
    }

    fn balance(&self) -> f64{
        self.balance
    }

    fn main(){
        let mut account = BankAccount{balance: 1000.0};
    }
}