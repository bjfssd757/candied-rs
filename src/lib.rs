#[macro_export]
/// Generates a vector based on given conditions
/// ## Versions
/// * Generating an array from a range:
/// ```Rust
/// let vector = vecg![for x in 0..=100]; // [0, 1, 2, ..., 98, 99, 100]
/// ```
/// 
/// * Generating an array from a range with a condition
/// ```Rust
/// let vector = vecg![for x in 0..=10; if x % 2 == 0 && x % 3 == 0]; // [0, 6]
/// ```
/// 
/// * Generating an array from a range with or without a condition and changing the appropriate element
/// ```Rust
/// let vector = vecg![x * 2 => for x in 0..10; if x % 3 == 0]; // [0, 6, 12, 18]
/// ```
/// 
/// # Example
/// ```Rust
/// let vv = vecg![for x in 0..100];
/// let v = vecg![for x in 0..11; if x % 2 == 0 && x % 3 == 0];
/// let vvv = vecg![x * 2 => for x in 0..10; if x % 3 == 0];
/// println!("{:?}\n{:?}\n{:?}", v, vv, vvv);
/// ```
macro_rules! vecg {
    (for $x:ident in $exp:expr; if $cond:expr) => {
        vecg![$x => for $x in $exp; if $cond]
    };

    (for $x:ident in $exp:expr) => {
        vecg![$x => for $x in $exp]
    };

    ($res:expr => for $x:ident in $exp:expr; if $cond:expr) => {
        {
            let mut v = Vec::new();
            for $x in $exp {
                if $cond {
                    v.push($res);
                }
            }
            v
        }
    };

    ($res:expr => for $x:ident in $exp:expr) => {
        {
            let mut v = Vec::new();
            for $x in $exp {
                v.push($res);
            }
            v
        }
    };
}

#[macro_export]
/// Executes the specified function for each passed parameter. Does not return the function's result
/// # Example
/// ```Rust
/// fn add2(a: &mut i32) -> i32 {
///     *a += 2;
///     *a
/// }
/// 
/// int main() {
///     let mut a = 12;
///     let mut b = 122;
///     let mut c = -12;
///     for_all!(add2; &mut a, &mut b, &mut c);
///     println!("a = {}\nb = {}\nc = {}", a, b, c);
///     // a = 14
///     // b = 124
///     // c = -10
/// }
/// ```
macro_rules! for_all {
    ($func_name:ident; $($all:expr),* $(,)?) => {
        $( $func_name($all); )*
    };
}

#[macro_export]
/// Executes the specified function for each parameter passed and returns the result of that function for each as a tuple
/// # Example
/// ```Rust
/// fn del2(a: &mut i32) -> i32 {
///     *a /= 2;
///     *a
/// }
/// 
/// int main() {
///     let mut a = 12;
///     let mut b = 122;
///     let mut c = -12;
///     let res = for_all_ret!(del2; &mut a, &mut b, &mut c);
///     println!("{:?}", res); // (6, 61, -6)
/// }
/// ```
macro_rules! for_all_ret {
    ($func_name:ident; $($all:expr),* $(,)?) => {
        ( $( $func_name($all) ),* )
    };
}

#[macro_export]
/// The macro provides a switch-case construct
/// # Example
/// ```Rust
/// let d = 0;
/// switch!(d => {
///     case 0 => println!("ZERO"),
///     case 1 => {
///         println!("HEY");
///         println!("HEYNEXTLINE");
///     },
///     case default => println!("DEFAULT"), // or `case _ => ...`
/// });
/// ```
macro_rules! switch {
    ($exp:expr => { $( case $cond:tt => $body:expr ),* $(,)? }) => {
        match $exp {
            $( 
                switch!(@PARSE_COND $cond) => $body, 
            )*
        }
    };

    (@PARSE_COND default) => { _ };
    (@PARSE_COND _) => { _ };
    (@PARSE_COND $cond:expr) => { $cond };
}

#[macro_export]
/// Ternary operator
/// # Example
/// ```Rust
/// let d = 0;
/// t!(d == 0 => println!("D = ZERO") , println!("D != ZERO"));
/// ```
macro_rules! t {
    ($cond:expr => $t:expr , $f:expr) => {
        if $cond { $t } else { $f }
    };
}

#[macro_export]
/// Executes code when the scope of a function or block is exited.
/// 
/// Block calls occur from the last macro definition to the first
/// # Example
/// ```Rust
/// defer!(println!("END OF THE MAIN")); // third
/// defer!(println!("END OF THE MAIN 2")); // second
/// defer!({
///     let mut b: i32 = -172312;
///     b = b.abs();
///     println!("------\nb = {b}");
/// }); // first
/// ```
macro_rules! defer {
    ($($body:tt)*) => {
        let _deferred = {
            struct Defer<F: FnOnce()>(Option<F>);
            impl<F: FnOnce()> Drop for Defer<F> {
                fn drop(&mut self) { (self.0.take().unwrap())(); }
            }
            Defer(Some(|| { $($body)* }))
        };
    };
}
