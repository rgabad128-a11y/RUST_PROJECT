use std::io::{self, Write};

fn main() {
    loop {
        let mut price = String::new();

        print!("Enter the price: ");
        io::stdout().flush().unwrap();
        //This line is because we want the print command to appear because if we do not use the stdout and flush commands we will have a problem which is that the words written in the print command will not appear

        io::stdin()
            .read_line(&mut price)
            .unwrap();
        // Using unwrap() for simplicity in this beginner project
// In production code expect("message") or proper error handling is preferred

        let clean_price: f64 = price.trim().parse().unwrap();
        //Here we transfer the value from the old variable to the new one converting the data from &str to f64 and removing the angles


        let mut discount = String::new();
    
        print!("Enter the discount : ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut discount)
            .unwrap();

        let clean_discount: f64 = discount.trim().parse().unwrap();

        let final_price = clean_price * (1.0 - clean_discount / 100.0);
        //Here we put this process to output the final result after the discount

        println!("The final price is >>> {}", final_price);
    }
}

