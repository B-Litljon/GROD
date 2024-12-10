## 11-15-24

### Project Goal:
Create a trading system that is focused on automation, and allows the user to define their own strategy using prewritten indicators.

### Steps to Achieve:
1. Build indicators
    - Either using enums, or structs
2. Capture user input from the CLI and allow them to define which indicators to use and when
3. Define the logic to mix and match indicators and their parameters to create a `Strategy` struct/enum
    - (I'm still learning these concepts - check out the chapter on structs and the one on enums!)
4. Save the strategy and parameters to some type of configuration file or database for reuse

## 11-16-24

I progressed through the Rust book and finished the chapter where it goes over enums and also match statements - very helpful! The strategy builder could easily be built using structs, enums, and pattern matching. Essentially write the code to calculate the indicators, then use enums to build a strategy constructor (basically what it would be).

## 11-17-24

I figured out a good way to handle user input for defining strategies! Rust has something called `Option` which allows us to literally specify all potential inputs, making it very easy to prevent some bugs by deciding what to do when a certain value is passed or no value whatsoever. You can read more about that in the Enums and Pattern Matching chapter.

### Example:
```rust
use std::io;

fn get_user_input(asset: Option<String>) {
    println!("What asset would you like to trade?");

    match asset {
        Some(ticker) => println!("Trading asset: {}", ticker),
        None => {
            println!("Grod needs a string value to know what asset you want to trade, please input the asset's ticker symbol. eg: AAPL, TSLA, MSFT");
            // Here you would likely want to prompt the user for input again
        }
    }
}

fn main() {
    let asset_input = Some(String::from("AAPL")); // Example with Some value
    get_user_input(asset_input);

    let no_asset_input = None; // Example with None value
    get_user_input(no_asset_input);
}
```

Use code with caution.

Of course, this is a simplified example. In a real application, you'd probably want to use a loop to keep prompting the user until they provide valid input. But the gist of it is there. Basically, we can define options so that Grod doesn't panic when it gets a strange value. In the case of integers for indicator parameters, we can even set it to a default value if the user doesn't specify what they'd like. In the case of Bollinger bands, the default period is 20, and the +/- deviation is 2. For RSI, the default overbought zone is 70 and above, whereas 30 and below means oversold.


## 11-19-24

I was considering switching to python for the sake of getting it done as quickly as possible. especially because I thought that I'd have to code all the indicators manually... turns out I don't have to, rust has all the crates I need to offer roughly the same functionality as I would with python, except in this case I have the awesome error handling capabilities of rust, as well as its 'safe' nature. so now we can focus on the main functionality of the project rather than worrying about how to write the indicators most efficiently. check out the cargo.toml file for what we installed! and change your plans a bit to accomodate your new tools