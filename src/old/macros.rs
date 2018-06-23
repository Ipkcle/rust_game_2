macro_rules! for_impl {
    {
        $($type:ty),*;

        impl {
            $($function:tt)*
        }

        $(impl $trait:ty {
            $($trait_function:tt)*
        })*

    } => {
        for_impl!(@call_tuple $($type),* | ($($function),*));
        for_impl!(@call_tuple_trait $($trait),* | ($($type),*) | ($(($($trait_function),*)),*));
    };

    {
        $($type:ty),*;

        $(impl $trait:ty {
            $($trait_function:tt)*
        })*

    } => {
        for_impl!(@call_tuple_trait $($trait),* | ($($type),*) | ($(($($trait_function),*)),*));
    };

    (@call_tuple $($type:ty),* | $function_tuple:tt) => {
        $(for_impl!(@call $type | $function_tuple);)*
    };
    (@call $type:ty | ($($function:tt),*)) => {
        impl $type {
            $($function)*
        }
    };

    (@call_tuple_trait $($trait:ty),* | $type_tuple:tt | ($($function_tuple:tt),*)) => {
        $(for_impl!(@call_trait $trait | $type_tuple | $function_tuple);)*
    };

    (@call_trait $trait: ty | ($($type:ty),*) | $function_tuple:tt) => {
        $(for_impl!(@call_trait_final $trait | $type | $function_tuple);)*
    };
    
    (@call_trait_final $trait: ty| $type:ty | ($($function:tt),*)) => {
        impl $trait for $type {
            $($function)*
        }
    };
}
