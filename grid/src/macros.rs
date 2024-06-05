#[macro_export]
macro_rules! grid {
    () => {};

    ( $type:ty, $rows:literal, $cols:literal $(, $extras:ty)? $(, $sides:literal)? ) => {
        <grid!(@type_brack $type, $($extras)?)> ::new_with_specs($rows, $cols, None,  grid!(@sides $($sides)?))
    };

    ( $val:expr => $type:ty, $rows:literal, $cols:literal $(, $extras:ty)? $(, $sides:literal)?) => {
        <grid!(@type_brack $type, $($extras)?)> ::new_with_specs($rows, $cols, Some($val),  grid!(@sides $($sides)?))
    };

    ( $val:expr, $rows:literal, $cols:literal $(, $extras:ty)? $(, $sides:literal)?) => {
        <grid!(@type_brack _, $($extras)?)> ::new_with_specs($rows, $cols, Some($val), grid!(@extras $($extras)?), grid!(@sides $($sides)?))
    };

    (@type_brack $type:ty, $extras:ty) => ( $crate::grid::Grid::<$type, $extras> );

    (@type_brack $type:ty, ) => ( $crate::grid::Grid::<$type, ()> );

    (@type_brack $type:pat, $extras:ty ) => ($crate::grid::Grid::<$type, $extras>);

    (@sides ) => ( None );

    (@sides $sides:literal ) => {
        {
            match $sides {
                4 => Some($crate::grid::Sided::Four),
                8 => Some($crate::grid::Sided::Eight),
                _ => None
            }
        }
    };

    (@error) => ({ compile_error!("something") });
}
