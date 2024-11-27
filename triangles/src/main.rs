use std::ops::Index;

fn main() {
    //let base:usize = 6;
    //let order: usize = 3;
    //let output = get_meta_meta_triangle(base);
    //let mut single: Vec<usize> = Vec::new();
    //let mut meta: Vec<usize> = Vec::new();
    //let mut metameta: Vec<usize> = Vec::new();
    //let mut metametameta: Vec<usize> = Vec::new();

    // for n in 1..101{
    //     single.push(get_triangle(n));
    //     meta.push(get_meta_triangle(n));
    //     metameta.push(get_meta_meta_triangle(n));
    //     metametameta.push(get_meta_meta_meta_triangle(n));
    // }
    //
    // for (index, el) in single.iter().enumerate(){
    //     println!("input: {}  single: {}  meta: {}  metameta: {}  metametameta: {}", index + 1, el, meta[index], metameta[index], metametameta[index]);
    // }

    get_n_order_triangle();
}
fn get_n_order_triangle() {
    // base:usize, order: usize
    // -> usize
    let mut order: usize = 2;
    let order_const: usize = order;
    let base: usize = 6;
    println!("setup complete");
    let mut source: Vec<usize> = (1..base + 1).collect();
    let mut target: Vec<usize> = Vec::with_capacity(base);
    for el in source {
        target.push(get_triangle(el));
    }
    source = target.clone();
    while order > 1 {
        for (index, el) in source.iter().enumerate() {
            if index == 0 {
                target[index] = 1;
                continue;
            }
            target[index] = sum_vector_from_index(&source, &index);
        }
        source = target.clone();
        order -= 1;
    }

    println!("{} & {} = {}", order_const, base, source[base - 1]);
}
fn sum_vector_from_index(source: &Vec<usize>, index: &usize) -> usize {
    let mut sum: usize = 0;
    let source = source.clone();
    let mut counter = *index;
    loop {
        sum += source[counter];
        if counter == 0 {
            break;
        }
        counter -= 1;
    }

    sum
}
fn get_triangle(base: usize) -> usize {
    // calculates triangular numbers from their base width
    let mut triangle: usize = 0;
    if base % 2 == 0 {
        triangle = (base + 1) * (base / 2);
    } else {
        triangle = base * ((base / 2) + 1);
    }
    //println!("triangle   = {}", triangle);
    return triangle;
}
