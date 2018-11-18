include!(concat!(env!("OUT_DIR"), "/count.rs"));

#[doc(hidden)]
#[macro_export]
macro_rules! dispatch {
    (() $($bang:tt)*) => {
        $crate::count!($($bang)*)
    };
    ((($($first:tt)*) $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($first)* $($rest)*) $($bang)*)
    };
    (([$($first:tt)*] $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($first)* $($rest)*) $($bang)*)
    };
    (({$($first:tt)*} $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($first)* $($rest)*) $($bang)*)
    };
    ((! $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($rest)*) $($bang)* !)
    };
    (($first:tt $($rest:tt)*) $($bang:tt)*) => {
        $crate::dispatch!(($($rest)*) $($bang)*)
    };
}
