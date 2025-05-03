fn main() {
    let p1 = (0.0, 0.0); // (f64, f64)

    println!("{:?}", p1);

    let mut p2 = (0.0, 0.0, 0.0); // (f64, f64, f64)

    println!("{:?}", p2);

    p2.0 = 1.0;

    p2.1 = 1.0;

    p2.2 = 1.0;

    println!("{:?}", p2);

    let a1 = [0; 5]; // [i32; 5]

    println!("{:?}", a1);

    let mut a2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];  // [i32; 10]

    for i in 0..a2.len() {
        a2[i] *= 2;
    }

    println!("{:?}", a2);
}
