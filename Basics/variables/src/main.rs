fn main() {
    let add = 5 + 10;
    let sub = 95.5 - 4.3;
    let mul = 5 * 10;
    let div = 90 / 4.3;
    println!("The sum is: {add}");
    println!("The difference is: {sub}");
    println!("The product is: {mul}");
    println!("The quotient is: {div}");

    let tuple: (i32, f64, i32, 132) = (500, 6.4, 60);
    let (x, y, z) = tuple;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}
