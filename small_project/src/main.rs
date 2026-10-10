use std::io::{self, Write};

fn main() {
    loop {
        let mut price1 = String::new();

        print!("Enter the price: ");
        io::stdout().flush().unwrap();
        //This line is because we want the print command to appear because if we do not use the stdout and flush commands we will have a problem which is that the words written in the print command will not appear

        io::stdin()
            .read_line(&mut price1)
            .unwrap();
        let price1 = clean_price1.trim();

        if clean_price1 == "q" {
            return;
        }

        let clean_price1: f64 = match price1.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error try again");
                continue;
            }
        };
        //Here we transfer the value from the old variable to the new one converting the data from &str to f64 and removing the angles


        let mut discount = String::new();
    
        print!("Enter the discount : ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut discount)
            .unwrap();

        let clean_discount: f64 = match discount.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error try again");
                continue;
            }
        };

        let final_price = clean_price * (1.0 - clean_discount / 100.0);
        //Here we put this process to output the final result after the discount

        println!("The final price is >>> {}", final_price);
    }
}

