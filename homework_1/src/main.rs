// assignments.rs
// Runs Assignment 1, 2, and 3 in sequence.

// ---------- Assignment 1: Temperature Converter ----------
const FREEZING_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_F) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + FREEZING_F
}

fn run_assignment_1() {
    println!("=== Assignment 1: Temperature Converter ===");
    let mut f: f64 = 32.0;

    let c = fahrenheit_to_celsius(f);
    println!("{f:.1}°F = {c:.1}°C");

    for _ in 0..5 {
        f += 1.0;
        let c_next = fahrenheit_to_celsius(f);
        println!("{f:.1}°F = {c_next:.1}°C");
    }

    // quick back-check
    let back_to_f = celsius_to_fahrenheit(c);
    println!("Back-check: {c:.1}°C = {back_to_f:.1}°F");
    println!();
}

// ---------- Assignment 2: Number Analyzer ----------
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn run_assignment_2() {
    println!("=== Assignment 2: Number Analyzer ===");
    let nums: [i32; 10] = [3, 5, 15, 2, 7, 9, 10, 12, 25, 30];

    for n in nums {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{n}: FizzBuzz");
        } else if n % 3 == 0 {
            println!("{n}: Fizz");
        } else if n % 5 == 0 {
            println!("{n}: Buzz");
        } else {
            if is_even(n) {
                println!("{n}: even");
            } else {
                println!("{n}: odd");
            }
        }
    }

    // sum with while loop
    let mut sum = 0;
    let mut i = 0;
    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum = {sum}");

    // find largest with loop
    let mut max_val = nums[0];
    let mut j = 1;
    loop {
        if j >= nums.len() {
            break;
        }
        if nums[j] > max_val {
            max_val = nums[j];
        }
        j += 1;
    }
    println!("Largest = {max_val}");
    println!();
}

// ---------- Assignment 3: Guessing Game ----------
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn run_assignment_3() {
    println!("=== Assignment 3: Guessing Game ===");
    let secret: i32 = 42;

    // Simulated guesses (includes the correct one)
    let guesses: [i32; 6] = [10, 70, 40, 43, 41, 42];

    let mut attempts = 0;
    let mut idx = 0;

    loop {
        let mut guess: i32 = guesses[idx];
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess #{attempts}: {guess} — correct!");
            break;
        } else if result == 1 {
            println!("Guess #{attempts}: {guess} — too high.");
        } else {
            println!("Guess #{attempts}: {guess} — too low.");
        }

        idx += 1;
    }

    println!("You guessed the number in {attempts} attempts.");
    println!();
}

// ---------- Main: run all ----------
fn main() {
    run_assignment_1();
    run_assignment_2();
    run_assignment_3();
}
