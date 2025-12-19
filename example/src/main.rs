use candied::*;

fn add2(a: &mut i32) -> i32 {
    *a += 2;
    *a
}

fn del2(a: &mut i32) -> i32 {
    *a /= 2;
    *a
}

fn main() {
    defer!(println!("END OF THE MAIN"));
    defer!(println!("END OF THE MAIN 2"));
    defer!({
        let mut b: i32 = -172312;
        b = b.abs();
        println!("------\nb = {b}");
    });

    let vv = vecg![for x in 0..100];
    let v = vecg![for x in 0..11; if x % 2 == 0 && x % 3 == 0];
    let vvv = vecg![x * 2 => for x in 0..10; if x % 3 == 0];
    let vvvv = vecg![x * 2, y => for x in 0..=10, for y in 0..=15; if x % 2 == 0 && y % 3 == 0];
    println!("{:?}\n{:?}\n{:?}\n{:?}", v, vv, vvv, vvvv);
    let _vvvvv = vecg![x * 3, y / 2, z + 1 => for x in 0..=10, for y in 10..=100, for z in -5i32..=-1; if x % 2 == 0 && y % 7 == 0 || z.abs() > 3];

    let mut a = 12;
    let mut b = 122;
    let mut c = -12;
    let res = for_all_ret!(del2; &mut a, &mut b, &mut c);
    println!("{:?}", res);

    for_all!(add2; &mut a, &mut b, &mut c);

    println!("a = {}\nb = {}\nc = {}", a, b, c);

    let d = 0;

    switch!(d => {
        case 0 => println!("ZERO"),
        case 1 => {
            println!("HEY");
            println!("HEYNEXTLINE");
        },
        case default => println!("DEFAULT"),
    });

    t!(d == 0 => println!("D = ZERO") , println!("D != ZERO"));

    let n: Option<i32> = None;
    println!("{}", candied_or!(n => 12));

    candied_repeat!(5, |i|, println!("{} iter", i));
    candied_repeat!(6, println!("Hello!"));
}