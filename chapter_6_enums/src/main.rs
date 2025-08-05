enum IpAddrKind{
    V4,
    V6
}

fn route(ip_kind: IpAddrKind){
    match ip_kind {
        IpAddrKind::V4 => println!("Routing IPv4 address..."),
        IpAddrKind::V6 => println!("Routing IPv6 address..."),
    }
}

fn main(){
    println!("--- Chapter 6 Enums ---");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Contoh Enum untuk routing");
    route(four);
    route(six);
    
    // Contoh enum coin dan match
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    println!("Contoh enum coin dan match");
    println!("Value of Penny: {}", value_in_cents(Coin::Penny));
    println!("Value of Nickel: {}", value_in_cents(Coin::Nickel));
    println!("Value of Dime: {}", value_in_cents(Coin::Dime));
    println!("Value of Quarter: {}", value_in_cents(Coin::Quarter));

    // Contoh if let
    let config_max = Some(3u8);

    println!("\n Contoh if let");
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ... other states
    }

    enum CoinWithState {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents_with_state(coin: CoinWithState) -> u8 {
        match coin {
            CoinWithState::Penny => 1,
            CoinWithState::Nickel => 5,
            CoinWithState::Dime => 10,
            CoinWithState::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    let coin_alaska = CoinWithState::Quarter(UsState::Alaska);
    let coin_penny = CoinWithState::Penny;
    
    println!("\n Contoh if let dengan else");
    let mut count = 0;
    if let CoinWithState::Quarter(ref state) = coin_alaska {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count after checking Alaska quarter: {}", count);

    if let CoinWithState::Quarter(ref state) = coin_penny {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count after checking Penny: {}", count);

    // Contoh let else
    fn describe_state_quarter(coin_param: &CoinWithState) -> Option<String> {
        let CoinWithState::Quarter(state) = coin_param else { 
            return None;
        };

        // Asumsi ada fungsi existed_in untuk UsState
        // Untuk contoh ini, kita akan simulasikan
        let is_old = match state {
            UsState::Alabama => true,
            UsState::Alaska => false,
            // ... handle state lain
        };

        if is_old {
            Some("Old Quarter".to_string()) 
        } else {
            Some("New Quarter".to_string()) 
        }
    }

    println!("\n-- Contoh let else --");
    println!("Describe Alaska quarter: {:?}", describe_state_quarter(&coin_alaska)); 
    println!("Describe Penny: {:?}", describe_state_quarter(&coin_penny)); 
}