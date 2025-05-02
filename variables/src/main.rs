fn main() {
    let programming_language = "Rust"; // &str

    println!("`programming_language`: {:?}", programming_language);

    // programming_language = "Zig" // error

    let mut fib = 1; // i32

    println!("`fib`: {}", fib);

    fib = 1;

    println!("`fib`: {}", fib);

    fib = 2;

    println!("`fib`: {}", fib);

    fib = 3;

    println!("`fib`: {}", fib);

    fib = 5;

    println!("`fib`: {}", fib);

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // [i32; 10]

    println!("`numbers`: {:?}", numbers);

    println!("`&numbers[4..]`: {:?}", &numbers[4..]);
}
