fn main() {
    let mut a = 1;
    let mut b = 1;

    f(&mut a, &mut b);

    println!("{} {}", a, b)
}

fn f(a: &mut i32, b: &mut i32) {
    *a += 2;
    *b += 1;
    
}