//! # Comprehension
//!
//! Python like list, set and hashmap comprehensions for Rust as efficiently as possible.
//!
//! Due to how Python handles comprehensions, a one to one translation is not possible, atleast not without resorting to cloning
//! everything. When using this library, its up to the user to clone if
//! necessary.
//!
//! Upholds Rust ownerships and borrow rules, passing by value WILL consume the collection.
//!
//! `comp!` returns a iterator over the elements. `compco!` includes a .collect() call.
//!
//! ## Examples
//!
//! Basic array comprehension
//! ```
//! # use comprehend::comp;
//! let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//! let y: Vec<_> = comp![2*x, for x in v].collect();
//! assert_eq!(y, vec![2, 4,6, 8, 10, 12, 14, 16, 18, 20]);
//! ```
//!
//! Nested array comprehension
//! ```
//! # use comprehend::comp;
//! let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
//! let y: Vec<_> = comp![2*p, for x in v, for p in x].collect();
//! assert_eq!(y, vec![2, 4, 6, 8, 10, 12, 14, 16, 18]);
//! ```
//!
//! Creating combinations of two arrays, must use clone!
//! ```
//! # use comprehend::comp;
//! let v = vec![1, 2];
//! let z = vec![3,4];
//! let y: Vec<_> = comp![(x,y), for x in v, for y in z.clone()].collect();
//! assert_eq!(y,vec![(1,3),(1,4),(2,3),(2,4)]);
//! ```
//!
//! Use with filters
//! Create a vector of vectors of 3 indices which are all less than 3, and add up to
//! 4.
//! ```
//! # use comprehend::comp;
//! let y: Vec<_> = comp![vec![i, j, k], for i in 0..3, for j in 0..3, for k in 0..3, if i+j+k == 4].collect();
//! assert_eq!(y,vec![[0,2,2],[1,1,2],[1,2,1],[2,0,2],[2,1,1],[2,2,0]]);
//! ```
//!
//! Basic hashmap comprehension
//! ```
//! # use std::collections::HashMap;
//! # use comprehend::comp;
//! let v = vec![1, 2, 3, 4];
//! let y: HashMap<_,_> = comp!{x=>2*x, for x in v}.collect();
//! assert_eq!(y, HashMap::from([(1, 2), (2, 4), (3, 6), (4, 8)]));
//! ```
//!  
//! Hashmap comprehension can be also nested used with filters or the special => syntax.
//!
//!
//! Special '=>' syntax when using nested comprehensions, to do an operation on an inner loop.
//! For example assign the first variable as value to the other numbers as keys in the array.
//!```
//! # use std::collections::HashMap;
//! # use comprehend::comp;
//! let v: Vec<Vec<_>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
//! #[allow(redundant_semicolons)]
//! let y: HashMap<i32,i32> =
//! comp![p=>z.unwrap(), for x in v => {let mut y = x.into_iter(); let z = y.next();}, for p in y].collect();
//! assert_eq!(y, HashMap::from([(2, 1), (3, 1), (5, 4), (6, 4), (8, 7), (9, 7)]));
//!
//! ```
//!
//! Would be equivalent to this:
//! ```
//! # use std::collections::HashMap;
//! # use comprehend::comp;
//! let v: Vec<Vec<_>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
//! #[allow(redundant_semicolons)]
//! let y: HashMap<i32,i32> =
//! comp![{let mut y = x.into_iter(); let z = y.next(); comp![p=>z.unwrap(),for p in y]}, for x in v].flatten().collect();
//! assert_eq!(y, HashMap::from([(2, 1), (3, 1), (5, 4), (6, 4), (8, 7), (9, 7)]));
//! ```

#[macro_export]
macro_rules! comp {

    // ------------------------------------------------------------------------------------
    //          Arrays / Sets
    // ------------------------------------------------------------------------------------


    // Basic
    [$ex:expr, for $i:pat in $iter:expr $(, if $cond:expr)?] => {
        $iter.into_iter()$(.filter(move |$i| $cond))?.map(move |$i| $ex)
    };

    // Handle nested arrays
    [$ex:expr, for $i:pat in $iter:expr $(=> {$($then:stmt;)+})? $(,for $i2:pat in $iter2:expr $(=> {$($then2:stmt;)+})?)+ $(, if $cond:expr)?]=>{
        $iter.into_iter().flat_map(move |$i| {$($($then;)+)? $crate::comp![$ex $(, for $i2 in $iter2 $(=>  {$($then2;)+})?)* $(, if $cond)?]})


    };
    // ------------------------------------------------------------------------------------
    //          HashMaps
    // ------------------------------------------------------------------------------------
    // NOTE: These are essentially the same as arrays with a tuple as value. Its up to you to collect as a map.
    // Use co to automatically collect as a hashmap.

    // Basic
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr $(, if $cond:expr)?} => {
        $crate::comp![($key, $ex), for $i in $iter $(, if $cond)?]
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr $(, if $cond:expr)?} => {
        $crate::comp![($key, $ex), for $i in $iter $(, if $cond)?]
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr $(, if $cond:expr)?} => {
        $crate::comp![($key, $ex), for $i in $iter $(, if $cond)?]
    };


    // Handle nested maps
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr $(=> {$($then:stmt;)+})? $(,for $i2:pat in $iter2:expr $(=> {$($then2:stmt;)+})?)+ $(, if $cond:expr)?}=>{
        $crate::comp![($key, $ex), for $i in $iter $(=> {$($then;)+})? $(, for $i2 in $iter2 $(=> {$($then2;)+})?)* $(, if $cond)?]
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr $(=> {$($then:stmt;)+})? $(,for $i2:pat in $iter2:expr $(=> {$($then2:stmt;)+})?)* $(, if $cond:expr)?}=>{
        $crate::comp![($key, $ex), for $i in $iter $(=> {$($then;)+})? $(, for $i2 in $iter2 $(=> {$($then2;)+})?)* $(, if $cond)?]
    };

    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr $(=> {$($then:stmt;)+})? $(,for $i2:pat in $iter2:expr $(=> {$($then2:stmt;)+})?)* $(, if $cond:expr)?}=>{
        $crate::comp![($key, $ex), for $i in $iter $(=> {$($then;)+})? $(, for $i2 in $iter2 $(=> {$($then2;)+})?)* $(, if $cond)?]
    };
    // ------------------------------------------------------------------------------------

}

#[macro_export]
macro_rules! compco {

    // ------------------------------------------------------------------------------------
    //          Arrays / Sets
    // ------------------------------------------------------------------------------------


    // Basic
    [$ex:expr, for $i:pat in $iter:expr $(, if $cond:expr)?] => {
        $iter.into_iter()$(.filter(move |$i| $cond))?.map(move |$i| $ex).collect()
    };

    // Handle nested arrays
    [$ex:expr, for $i:pat in $iter:expr $(=> {$($then:stmt;)+})? $(,for $i2:pat in $iter2:expr $(=> {$($then2:stmt;)+})?)+ $(, if $cond:expr)?]=>{
        #[allow(redundant_semicolons)]
        $iter.into_iter().flat_map(move |$i| {$($($then;)+)? $crate::comp![$ex $(, for $i2 in $iter2 $(=> {$($then2;)+})?)* $(, if $cond)?]}).collect()


    };
    // ------------------------------------------------------------------------------------
    //          HashMaps
    // ------------------------------------------------------------------------------------
    // NOTE: These are essentially the same as arrays with a tuple as value. Its up to you to collect as a map.
    // Use co to automatically collect as a hashmap.

    // Basic
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr $(, if $cond:expr)?} => {
        $crate::comp![($key, $ex), for $i in $iter $(, if $cond)?].collect()
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr $(, if $cond:expr)?} => {
        $crate::comp![($key, $ex), for $i in $iter $(, if $cond)?].collect()
    };
    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr $(, if $cond:expr)?} => {
        $crate::comp![($key, $ex), for $i in $iter $(, if $cond)?].collect()
    };


    // Handle nested maps
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:pat in $iter:expr $(=> {$($then:stmt;)+})? $(,for $i2:pat in $iter2:expr $(=> {$($then2:stmt;)+})?)+ $(, if $cond:expr)?}=>{
        $crate::comp![($key, $ex), for $i in $iter $(=> {$($then;)+})? $(, for $i2 in $iter2 $(=> {$($then2;)+})?)* $(, if $cond)?].collect()
    };
    {$key:expr, $ex:expr, for $i:pat in $iter:expr $(=> {$($then:stmt;)+})? $(,for $i2:pat in $iter2:expr $(=> {$($then2:stmt;)+})?)* $(, if $cond:expr)?}=>{
        $crate::comp![($key, $ex), for $i in $iter $(=> {$($then;)+})? $(, for $i2 in $iter2 $(=> {$($then2;)+})?)* $(, if $cond)?].collect()
    };

    {{$key:expr, $ex:expr}, for $i:pat in $iter:expr $(=> {$($then:stmt;)+})? $(,for $i2:pat in $iter2:expr $(=> {$($then2:stmt;)+})?)* $(, if $cond:expr)?}=>{
        $crate::comp![($key, $ex), for $i in $iter $(=> {$($then;)+})? $(, for $i2 in $iter2 $(=> {$($then2;)+})?)* $(, if $cond)?].collect()
    };
    // ------------------------------------------------------------------------------------

}

#[cfg(test)]
mod tests {
    use super::*;

    mod array {
        use super::*;

        #[test]
        fn basic() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: Vec<_> = comp![x, for x in v].collect();

            assert_eq!(y, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        }
        #[test]
        fn complex_collection() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: Vec<_> =
                comp![(ind as i32*x), for (ind,x) in v.into_iter().enumerate()].collect();

            assert_eq!(y, vec![0, 2, 6, 12, 20, 30, 42, 56, 72, 90]);
        }
        #[test]
        fn basic_expr() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            // v is passed by reference, we have to clone x-s.
            let y: Vec<_> = comp![2*(x.clone()), for x in v.iter()].collect();

            assert_eq!(y, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
        }
        #[test]
        fn basic_expr2() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: Vec<_> = comp![if x > 5 {x*2} else {x}, for x in v].collect();

            assert_eq!(y, vec![1, 2, 3, 4, 5, 12, 14, 16, 18, 20]);
        }
        #[test]
        fn filter() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: Vec<_> = comp![x, for x in v, if x % 2 == 0].collect();

            assert_eq!(y, vec![2, 4, 6, 8, 10]);
        }
        #[test]
        fn filter2() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            // Pass all
            let y: Vec<_> = comp![x, for x in v, if x %2 == 1 || x%2 ==0].collect();

            assert_eq!(y, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        }
        #[test]
        fn nested_basic() {
            let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

            let y: Vec<_> = comp![2i32*p, for x in v, for y in x, for p in y].collect();

            assert_eq!(y, vec![2, 4, 6, 8, 10, 12, 14, 16]);
        }
        #[test]
        fn nested_ranges() {
            let y: Vec<(i32, i32)> =
                comp![(i,j), for i in 0..3, for j in 0..3, if (i+j) % 3 == 0].collect();
            assert_eq!(y, vec![(0, 0), (1, 2), (2, 1)]);
        }
        #[test]
        fn nested_multiarray() {
            let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
            let v2 = v.clone();

            // This is an edge case. The `y` inner variable is used 2 times. We need to clone it.
            // Since the expression is evaluated at the end, we need to clone in 'for' loop.
            // There is no way to automatically clone the collection, without cloning in other
            // (unnecessary) cases.
            // Thankfully, the Rust LSP will tell you to clone it.
            // This edge case will also not work if the type does not impl Copy.
            let y: Vec<_> = comp![p*y[0], for x in v, for y in x, for p in y.clone()].collect();

            let v = v2;
            assert_eq!(
                y,
                vec![
                    1 * v[0][0][0],
                    2 * v[0][0][0],
                    3 * v[0][1][0],
                    4 * v[0][1][0],
                    5 * v[1][0][0],
                    6 * v[1][0][0],
                    7 * v[1][1][0],
                    8 * v[1][1][0],
                ]
            );
        }
    }

    mod map_tests {
        use std::collections::HashMap;

        use super::*;
        #[test]
        fn basic() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: HashMap<_, _> = comp! {x=>x, for x in v}.collect();

            assert_eq!(
                y,
                HashMap::from([
                    (1, 1),
                    (2, 2),
                    (3, 3),
                    (4, 4),
                    (5, 5),
                    (6, 6),
                    (7, 7),
                    (8, 8),
                    (9, 9),
                    (10, 10)
                ])
            );
        }
        #[test]
        fn basic2() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: HashMap<_, _> = comp! {x=>x, for x in v}.collect();
            let y: HashMap<_, _> = comp! {y=>2*x, for (y,x) in y}.collect();

            assert_eq!(
                y,
                HashMap::from([
                    (1, 2),
                    (2, 4),
                    (3, 6),
                    (4, 8),
                    (5, 10),
                    (6, 12),
                    (7, 14),
                    (8, 16),
                    (9, 18),
                    (10, 20)
                ])
            );
        }
        #[test]
        fn basic_expr() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: HashMap<i32, i32> = comp! {2*x=>x, for x in v}.collect();

            assert_eq!(
                y,
                HashMap::from([
                    (2, 1),
                    (4, 2),
                    (6, 3),
                    (8, 4),
                    (10, 5),
                    (12, 6),
                    (14, 7),
                    (16, 8),
                    (18, 9),
                    (20, 10)
                ])
            );
        }
        #[test]
        fn basic_expr2() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: HashMap<i32, bool> = comp! {x=> x>5, for x in v}.collect();

            assert_eq!(
                y,
                HashMap::from([
                    (1, false),
                    (2, false),
                    (3, false),
                    (4, false),
                    (5, false),
                    (6, true),
                    (7, true),
                    (8, true),
                    (9, true),
                    (10, true)
                ])
            );
        }
        #[test]
        fn filter() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let y: HashMap<i32, i32> = comp! {x=>x%3, for x in v, if x %2 == 0}.collect();

            assert_eq!(y, HashMap::from([(2, 2), (4, 1), (6, 0), (8, 2), (10, 1)]));
        }
        #[test]
        fn filter2() {
            let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            // Pass all
            let y: HashMap<i32, i32> =
                comp! {x=>x%3, for x in v, if x %2 == 1 || x%2 ==0}.collect();

            assert_eq!(
                y,
                HashMap::from([
                    (1, 1),
                    (2, 2),
                    (3, 0),
                    (4, 1),
                    (5, 2),
                    (6, 0),
                    (7, 1),
                    (8, 2),
                    (9, 0),
                    (10, 1)
                ])
            );
        }
        #[test]
        fn nestedthensyntax() {
            let v: Vec<Vec<_>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

            #[allow(redundant_semicolons)]
            let y: HashMap<i32,i32> =
            comp![p=>z.unwrap(), for x in v => {let mut y = x.into_iter(); let z = y.next();}, for p in y]
                .collect();
            assert_eq!(
                y,
                HashMap::from([(2, 1), (3, 1), (5, 4), (6, 4), (8, 7), (9, 7)])
            );
        }
        #[test]
        fn nested_basic() {
            let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

            let y: HashMap<i32, i32> =
                comp! {2*p=>p%3, for x in v, for y in x, for p in y}.collect();

            assert_eq!(
                y,
                HashMap::from([
                    (2, 1),
                    (4, 2),
                    (6, 0),
                    (8, 1),
                    (10, 2),
                    (12, 0),
                    (14, 1),
                    (16, 2)
                ])
            );
        }
        #[test]
        fn nested_basic2() {
            let v = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];

            let y: HashMap<_, _> =
                comp! {z=>ind, for x in v, for (ind,z) in comp![2*z, for z in x].enumerate()}
                    .collect();

            assert_eq!(
                y,
                HashMap::from([
                    (2, 0),
                    (4, 1),
                    (6, 0),
                    (8, 1),
                    (10, 0),
                    (12, 1),
                    (14, 0),
                    (16, 1)
                ])
            );
        }
        #[test]
        fn nested_multiarray() {
            let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

            let y: HashMap<_, _> =
                comp! {2*p=>y[0], for x in v, for y in x, for p in y.clone()}.collect();

            assert_eq!(
                y,
                HashMap::from([
                    (2, 1),
                    (4, 1),
                    (6, 3),
                    (8, 3),
                    (10, 5),
                    (12, 5),
                    (14, 7),
                    (16, 7)
                ])
            );
        }
    }
}
