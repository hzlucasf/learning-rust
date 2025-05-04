fn main() {
    println!("`is_valid_triangle((3.0, 4.0, 5.0))`: {}", is_valid_triangle((3.0, 4.0, 5.0)));

    println!("`is_valid_triangle((0.0, 4.0, 5.0))`: {}", is_valid_triangle((0.0, 4.0, 5.0)));

    match triangle_area((3.0, 4.0, 5.0)) {
        Ok(r) => println!("{:.2}", r),
        _ => println!("invalid triangle")
    }

    println!("`factorial(5)`: {}", factorial(5));
}

fn is_valid_triangle(triangle: (f64, f64, f64)) -> bool {
    let (x, y, z) = triangle;

    x + y > z && x + z > y && y + z > x
}

fn triangle_area(triangle: (f64, f64, f64)) -> Result<f64, ()> {
    if !is_valid_triangle(triangle) {
        return Err(());
    }

    let (x, y, z) = triangle;

    let semi_perimeter = (x + y + z) / 2.0;
        
    Ok((semi_perimeter * (semi_perimeter - x) * (semi_perimeter - y) * (semi_perimeter - z)).sqrt())
}

fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1)
    }
}
