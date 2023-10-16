#[warn(unused_imports)]
use rand::Rng;
use std::io::{self, Read};

fn main() {
    /* Write a Rust program that prints "Hello, Rust!" to the console. */
    // println!("Hello, Rust!");

    /*Create a Rust program that asks the user for their name and then prints a personalized greeting.*/
    /* let mut name = String::new();
    println!("Enter your name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Unable to read your name!!");

    println!("Assalam-o-Alaikum, {}", name); */

    /* Write a Rust program that converts temperatures between Celsius and
    Fahrenheit. Allow the user to input the temperature and the scale (C or F) and
    print the converted temperature.
     */
    // temperature_cal();

    /* Implement a Rust function that calculates the factorial of a given non-negative integer using recursion. */
    /* let mut num: i64 = 0;
    let mut value = String::new();
    println!("Enter a value:");
    io::stdin()
        .read_line(&mut value)
        .expect("Unable to read value!!");
    num = value.trim().parse().expect("Not a valid input."); */

    /* if num >= 0 {
        println!("{}", factorial(num));
    } else {
        println!("Number should be greater than 0.");
    } */

    // even odd
    /* if num >= 0 {
        even_odd(num);
    } else {
        println!("Number should be greater than 0.");
    } */

    /* Create a Rust program that calculates the sum of all integers from 1 to N, where N is provided by the user.

    if num >= 0 {
        println!("{}", sum_of_numbers(num));
    } else {
        println!("Number should be greater than 0.");
    } */

    /* Write a Rust program that prints numbers from 1 to 100. For multiples of 3,
    print "Fizz" instead of the number. For multiples of 5, print "Buzz" instead of
    the number. For numbers which are multiples of both 3 and 5, print
    "FizzBuzz." */

    /* let mut counter = 1;

    while counter <= 100 {
        if counter % 3 == 0 && counter % 5 == 0 {
            println!("FizzBuzz");
        } else if counter % 5 == 0 {
            println!("Buzz");
        } else if counter % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{counter}");
        }
        counter += 1;
    } */

    /* Checking palindrome */
    /* let mut word = String::new();
    println!("Enter word:");
    io::stdin()
        .read_line(&mut word)
        .expect("Unable to read word.");
    println!("{}", palindrome(&word)); */

    /* Create a Rust program that generates a random number between 1 and 100
    and allows the user to guess it. Provide feedback on whether the guess is too
    high or too low
    guessing(); */

    /* Develop a Rust program that acts as a basic calculator. It should allow the
    user to enter two numbers and an operation (+, -, *, /) and display the result
    let num1 = input_take("Enter number:");
    let num2 = input_take("Enter number:");

    let mut choice = String::new();
    println!("Enter your choice:");
    println!("1 to add:");
    println!("2 to sub:");
    println!("3 to mul:");
    println!("4 to div:");
    io::stdin().read_line(&mut choice).expect("Invalid input.");
    let choice = choice.trim().parse().expect("Invalid input!");

    match choice {
        1 => add(num1, num2),
        2 => sub(num1, num2),
        3 => mul(num1, num2),
        4 => div(num1, num2),
        _ => println!("Not a valid choice."),
    }; */

    /* Write a Rust function that finds and returns the largest number in an array of
    integers.
    println!("{}", largest_num()); */

    /* Implement a Rust function that takes a string as input and returns the string
    reversed */
    /* let mut word = String::new();
    println!("Enter word:");
    io::stdin()
        .read_line(&mut word)
        .expect("Unable to read word.");
    reversed(&word); */

    /* Write a Rust function that generates the Fibonacci sequence up to a specified
    number of terms, N */
    // println!("{}", fibonacci());

    /* Prime numbers */
    // prime_num();

    /* Ownership and borrowing */
    /* let nova = String::from("Hi");
    println!("{}", owner_borrow(nova)); */

    /* Define a Rust function that takes two string references as parameters and
    returns the longest one. Include lifetime annotations in the function signature */

    /* let msg1 = input_take_str("Enter string one:");
    let msg2 = input_take_str("Enter string two:");

    longest(&msg1, &msg2); */

    /* .Create a Rust program that takes a string from the user and prints its first and
    last character using string slicing.
     */
    /* let mut input = String::new();
    println!("Enter string:");
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input.");

    let input = input.trim();

    let first_char = &input[0..1];
    println!("{} is the first character.", first_char);

    let length = input.len();
    let last_char = &input[length - 1..length];
    println!("{} is the last character.", last_char); */

    /* Implement a Rust function that takes a string and a substring as input and
    returns the number of times the substring appears in the string */

    /* let msg = "Hi, I am Daniyal, I am 22 year old";
    let sub_msg = "I am";

    let appearance = counting(&msg, &sub_msg);
    println!("{appearance}"); */

    /* Reverse a sentence */
    /* let msg = "Hi, I am Daniyal, I am 22 year old";
    let reversed = reverse_word(&msg);
    println!("Original message: {}", msg);
    println!("Reversed message: {}", reversed); */

    /* Create a Rust program that reads a text file and counts the occurrences of
    each word, ignoring punctuation and case. Use string slicing to split the text
    into words */
}

/* fn input_take_str(msg: &str) -> String {
    println!("{}", msg);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input.");

    input
} */

/* fn input_take(msg: &str) -> i32 {
    println!("{}", msg);

    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Unable to read input.");
    num.trim().parse().expect("Invalid input!")
}

fn add(num1: i32, num2: i32) {
    let result = num1 + num2;
    println!("{result}");
}

fn sub(num1: i32, num2: i32) {
    let result = num1 - num2;
    println!("{result}");
}

fn mul(num1: i32, num2: i32) {
    let result = num1 * num2;
    println!("{result}");
}

fn div(num1: i32, num2: i32) {
    let mut result = 0;
    if num2 != 0 {
        result = num1 + num2;
        println!("{result}");
    }
} */

/* Task 3
fn temperature_cal() {
    let mut unit = String::new();
    println!("Enter temperature unit:");
    io::stdin()
        .read_line(&mut unit)
        .expect("Unable to read temperature unit!!");

    match unit.trim() {
        "celsius" => {
            let mut temp = String::new();
            println!("Enter temperature:");
            io::stdin()
                .read_line(&mut temp)
                .expect("Unable to read temperature!!");
            let celsius: f64 = temp.trim().parse().expect("Invalid input.");
            let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
            println!("Temperature in fahrenheit is {:.2}", fahrenheit);
        }
        "fahrenheit" => {
            let mut temp = String::new();
            println!("Enter temperature:");
            io::stdin()
                .read_line(&mut temp)
                .expect("Unable to read temperature!!");
            let fahrenheit: f64 = temp.trim().parse().expect("Invalid input.");
            let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
            println!("Temperature in celsius is {:.2}", celsius);
        }
        _ => println!("Invalid temperature unit."),
    }
}
 */

/* Task 4
fn factorial(num: i64) -> i64 {
    if num == 0 || num == 1 {
        1
    } else {
        num * factorial(num - 1)
    }
} */

/* fn even_odd(num: i64) {
    if num % 2 == 0 {
        println!("Number is even.");
    } else {
        println!("Number is odd");
    }
} */

/* Task 6
fn sum_of_numbers(num: i64) -> i64 {
    let mut sum = 0;
    let mut i = 1;

    loop {
        sum += i;
        i += 1;

        if i > num {
            break;
        }
    }
    sum
}
 */

/* Task 8
 fn palindrome(word: &str) -> bool {
    let word = word.trim();
    let mut a: [char; 10] = ['\0'; 10];
    let mut i = 0;

    for c in word.chars() {
        a[i] = c;
        i += 1;
    }

    for j in 0..i / 2 {
        if a[j] != a[i - 1 - j] {
            return false;
        }
    }
    true
} */

/* fn guessing() {
    let random = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Unable to read guess number");
        let num: i32 = num.trim().parse().expect("Invalid input.");
        println!("You guessed {}", num);

        if num > random {
            println!("Too high!");
        } else if num < random {
            println!("Too low!");
        } else {
            println!("Hurrah, you guessed it right!");
            break;
        }
    }
} */

/* fn largest_num() -> i32 {
    let a: [i32; 8] = [1, 5, 8, 7, 10, 64, 92, 41];
    let mut largest = a[0];

    let mut count = 0;
    loop {
        if largest < a[count + 1] {
            largest = a[count + 1];
        }
        count += 1;
        if count == a.len() - 1 {
            break;
        }
    }
    largest
} */

/* fn reversed(word: &str) {
    let word = word.trim();
    let mut a: [char; 10] = ['\0'; 10];
    let mut b: [char; 10] = ['\0'; 10];
    let mut i = 0;

    for c in word.chars() {
        a[i] = c;
        i += 1;
    }

    let mut j = i;

    for i in &a {
        b[j] = *i; *i is used to access the character value pointed to by the reference i in your code.
        j -= 1;

        if j == 0 {
            break;
        }
    }

    for i in &b {
        print!("{}", i);
    }
} */

/* fn fibonacci() -> i64 {
    let mut num1 = 0;
    let mut num2 = 1;

    let mut input = String::new();
    println!("Enter number until you want to find fibonacci sequence:");
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input!");
    let input: i64 = input.trim().parse().expect("Invalid input!");

    let mut calculate = 0;
    for i in 1..=input {
        calculate = num1 + num2;
        num1 = num2;
        num2 = calculate;
    }

    num2
} */

/* fn prime_num() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input!");
    let input: i64 = input.trim().parse().expect("Invalid input!");

    let mut i = 1;
    let mut count = 0;
    loop {
        if input % i == 0 {
            count += 1;
        }
        i += 1;

        if i > input {
            if count == 2 {
                break;
            }
        }
    }
} */

/* fn owner_borrow(mut msg: String) -> String {
    msg.push_str(", Daniyal.");
    msg
} */

/* fn longest(word1: &str, word2: &str) {
    let word1 = word1.trim();
    let word2 = word2.trim();
    let mut str1: [char; 20] = ['\0'; 20];
    let mut str2: [char; 20] = ['\0'; 20];

    let mut i = 0;
    let mut j = 0;

    for c in word1.chars() {
        str1[i] = c;
        i += 1;
    }

    for c in word2.chars() {
        str2[j] = c;
        j += 1;
    }

    if i > j {
        for k in &str1 {
            print!("{}", k);
        }
    } else {
        for m in &str2 {
            print!("{}", m);
        }
    }
} */

/* fn counting(msg: &str, sub_msg: &str) -> usize {
    let equal = msg.match_indices(sub_msg);
    let count = equal.count();

    count
} */

/* fn reverse_word(word: &str) -> String {
    let mut reversed_word = String::new();
    for c in word.chars().rev() {
        reversed_word.push(c);
    }
    reversed_word
} */
