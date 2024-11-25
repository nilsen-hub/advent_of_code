fn main() {
for n in 1..101{
    println!("{} = {}", n, get_triangle(n));
}
let base = 43; 
println!("Meta triangle of {} is {}", base, get_meta_triangle(base));

}
fn get_triangle(base: usize) -> usize {
    // calculates triangular numbers from their base width
    let mut triangle: usize = 0;
    if base % 2 == 0 {
        triangle = (base + 1) * (base / 2);
    } else {
        triangle = base * ((base / 2) + 1);
    }

    return triangle;
}
fn get_meta_triangle(base: usize) -> usize{
    let mut output:usize = 0;
    let mut base = base;
    while base > 0{
        output += get_triangle(base);
        base -= 1;
    }

    output
}