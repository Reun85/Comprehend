//! # Comprehension
//!
//! Macro rules starting with @ are internal

#[macro_export]
macro_rules! c_internal {

    (@param $i:ident) => {$i};
    (@param _) => {_};
    (@param ($($i:tt),*)) => {( $($crate::c_internal!(@param $i)),*)};
    [@inn $ex:expr, for $i:tt in $iter:expr] => {
        $iter.clone().into_iter().map(|$crate::c_internal!(@param $i)| $ex).collect::<Vec<_>>()
    };
    [@inn $ex:expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$crate::c_internal!(@param $i)| $crate::c_internal![@inn $ex $(, for $i2 in $iter2)*])
    };
    [@innif $ex:expr, if $cond:expr, for $i:tt in $iter:expr] => {
        $iter.clone().into_iter().filter(|$crate::c_internal!(@param $i)| $cond).map(|$crate::c_internal!(@param $i)| $ex).collect::<Vec<_>>()
    };
    [@innif $ex:expr, if $cond: expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$crate::c_internal!(@param $i)| $crate::c_internal![@innif $ex, if $cond $(, for $i2 in $iter2)*])
    };
    [@innif $ex:expr, if $cond:expr, else $alt:expr, for $i:tt in $iter:expr] => {
        $iter.iter().map(|$crate::c_internal!(@param $i)|
        if $cond{
            $ex
        } else{
            $alt
        }
        ).collect::<Vec<_>>()
    };
    [@innif $ex:expr, if $cond: expr, else $alt: expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$crate::c_internal!(@param $i)| $crate::c_internal![@innif $ex, if $cond, else $alt $(, for $i2 in $iter2)*])
    };
}
#[macro_export]
macro_rules! c {

    // ------------------------------------------------------------------------------------
    //          Arrays
    // ------------------------------------------------------------------------------------


    // Basic
    // ------------------------------------------------------------------------------------
    [$ex:expr, for $i:tt in $iter:expr] => {
        $iter.into_iter().map(|$crate::c_internal!(@param $i)| $ex)
    };
    [$ex:expr, if $cond:expr, for $i:tt in $iter:expr] => {
        $iter.into_iter().filter(|$crate::c_internal!(@param $i)| $cond).map(|$i| $ex)
    };
    [$ex:expr, if $cond:expr, else $alt: expr, for $i:tt in $iter:expr] => {
        $iter.into_iter().map(|$crate::c_internal!(@param $i)|
            if $cond {
                $ex
            } else {
                $alt
            }
        )
    };


    // Handle nested arrays
    // ------------------------------------------------------------------------------------
    [$ex:expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$crate::c_internal!(@param $i)| $crate::c_internal![@inn $ex $(, for $i2 in $iter2)*])

    };
    // ------------------------------------------------------------------------------------

    // Handle nested array ifs.
    // ------------------------------------------------------------------------------------
    [$ex:expr, if $cond: expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$crate::c_internal!(@param $i)| $crate::c_internal!(@innif $ex, if $cond $(, for $i2 in $iter2)*))
    };
    // ------------------------------------------------------------------------------------

    // Handle nested array if elses.
    // ------------------------------------------------------------------------------------
    [$ex:expr, if $cond: expr, else $alt:expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*]=>{
        $iter.into_iter().flat_map(|$crate::c_internal!(@param $i)| $crate::c_internal!(@innif $ex, if $cond, else $alt $(, for $i2 in $iter2)*))
    };
    // ------------------------------------------------------------------------------------


    // ------------------------------------------------------------------------------------
    //         Sets
    // ------------------------------------------------------------------------------------
    // NOTE: THese are essentially the same as arrays. Its up to you to collect as a set.
    // Use co to automatically collect as a set.

    // ------------------------------------------------------------------------------------


    // ------------------------------------------------------------------------------------
    //          HashMaps
    // ------------------------------------------------------------------------------------
    // NOTE: THese are essentially the same as arrays. Its up to you to collect as a map.
    // Use co to automatically collect as a hashmap.

    // Basic
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:tt in $iter:expr} => {
        $crate::c![($key, $ex), for $i in $iter]
    };
    {$key:expr, $ex:expr, for $i:tt in $iter:expr} => {
        $crate::c![($key, $ex), for $i in $iter]
    };
    {{$key:expr, $ex:expr}, for $i:tt in $iter:expr} => {
        $crate::c![($key, $ex), for $i in $iter]
    };
    {$key:expr => $ex:expr, if $cond:expr, for $i:tt in $iter:expr} => {
        $crate::c![($key,$ex), if $cond, for $i in $iter]
    };
    {$key:expr, $ex:expr, if $cond:expr, for $i:tt in $iter:expr} => {
        $crate::c![($key,$ex), if $cond, for $i in $iter]
    };
{{$key:expr, $ex:expr}, if $cond:expr, for $i:tt in $iter:expr} => {
        $crate::c![($key,$ex), if $cond, for $i in $iter]
    };
    {$key:expr => $ex:expr, if $cond:expr, else $alt: expr, for $i:tt in $iter:expr} => {
        $crate::c![($key,$ex), if $cond, else $alt, for $i in $iter]
    };
    {$key:expr, ex:expr, if $cond:expr, else $alt: expr, for $i:tt in $iter:expr} => {
        $crate::c![($key,$ex), if $cond, else $alt, for $i in $iter]
    };
    {{$key:expr, ex:expr}, if $cond:expr, else $alt: expr, for $i:tt in $iter:expr} => {
        $crate::c![($key,$ex), if $cond, else $alt, for $i in $iter]
    };


    // Handle nested maps
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::c![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*]
    };
    {$key:expr, $ex:expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::c![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*]
    };
    {{$key:expr, $ex:expr}, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::c![($key, $ex), for $i in $iter $(, for $i2 in $iter2)*]
    };
    // ------------------------------------------------------------------------------------

    // Handle nested map ifs.
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, if $cond: expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::$crate::c![($key, $ex), if $cond, for $i in $iter $(, for $i2 in $iter2)*]
    };
    {$key:expr, $ex:expr, if $cond: expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::c![($key, $ex), if $cond, for $i in $iter $(, for $i2 in $iter2)*]
    };
    {{$key:expr, $ex:expr}, if $cond: expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::c![($key, $ex), if $cond, for $i in $iter $(, for $i2 in $iter2)*]
    };
    // ------------------------------------------------------------------------------------

    // Handle nested map if elses.
    // ------------------------------------------------------------------------------------
    {$key:expr => $ex:expr, if $cond: expr, else $alt:expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::c![($key, $ex), if $cond, else $alt, for $i in $iter $(, for $i2 in $iter2)*]
    };
    {$key:expr, $ex:expr, if $cond: expr, else $alt:expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::c![($key, $ex), if $cond, else $alt, for $i in $iter $(, for $i2 in $iter2)*]
    };
    {{$key:expr, $ex:expr}, if $cond: expr, else $alt:expr, for $i:tt in $iter:expr $(,for $i2:tt in $iter2:expr)*}=>{
        $crate::c![($key, $ex), if $cond, else $alt, for $i in $iter $(, for $i2 in $iter2)*]
    };

    // ------------------------------------------------------------------------------------

}

#[cfg(test)]
mod array_tests {
    use super::*;
    #[test]
    fn basic() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<i32> = c![x, for (_,x) in v.into_iter().enumerate()].collect();

        assert_eq!(y, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    #[test]
    fn basic2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<_> = c![(ind*x), for (ind,x) in v.into_iter().enumerate()].collect();

        assert_eq!(y, vec![0, 2, 6, 12, 20, 30, 42, 56, 72, 90]);
    }
    #[test]
    fn basic_expr() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<_> = c![2*x, for x in v].collect();

        assert_eq!(y, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    }
    #[test]
    fn basic_expr2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<_> = c![if x > 5 {x*2} else{x}, for x in v].collect();

        assert_eq!(y, vec![1, 2, 3, 4, 5, 12, 14, 16, 18, 20]);
    }
    #[test]
    fn filter() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: Vec<_> = c![x, if x %2 == 0, for x in v].collect();

        assert_eq!(y, vec![2, 4, 6, 8, 10]);
    }
    #[test]
    fn filter2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Pass all
        let y: Vec<_> = c![x, if x %2 == 1 || x%2 ==0, for x in v].collect();

        assert_eq!(y, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    #[test]
    fn nested_basic() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

        let y: Vec<_> = c![2*p, for x in v, for y in x, for p in y].collect();

        assert_eq!(y, vec![2, 4, 6, 8, 10, 12, 14, 16]);
    }
    #[test]
    fn nested_ranges() {
        let y: Vec<(i32, i32)> =
            c![(i,j), if (i+j) % 3 == 0, for i in 0..3, for j in 0..3].collect();
        assert_eq!(y, vec![(0, 0), (1, 2), (2, 1)]);
    }
    #[test]
    fn nested_multiarray() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

        // Using .iter therefore we do actually clone.
        let y: Vec<_> = c![p*y[0], for x in v.iter(), for y in x, for p in y].collect();

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

#[cfg(test)]
mod map_tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn basic() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: HashMap<i32, i32> = c! {x=>x, for x in v}.collect();

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
    fn basic_expr() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let y: HashMap<i32, i32> = c! {2*x=>x, for x in v}.collect();

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

        let y: HashMap<i32, bool> = c! {x=> x>5, for x in v}.collect();

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

        let y: HashMap<i32, i32> = c! {x=>x%3, if x %2 == 0, for x in v}.collect();

        assert_eq!(y, HashMap::from([(2, 2), (4, 1), (6, 0), (8, 2), (10, 1)]));
    }
    #[test]
    fn filter2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Pass all
        let y: HashMap<i32, i32> = c! {x=>x%3, if x %2 == 1 || x%2 ==0, for x in v}.collect();

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
    fn nested_basic() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

        let y: HashMap<i32, i32> = c! {2*p=>p%3, for x in v, for y in x, for p in y}.collect();

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
    fn nested_multiarray() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

        let y: HashMap<i32, i32> = c! {2*p=>y[0], for x in v, for y in x, for p in y}.collect();

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
