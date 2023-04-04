#[doc(hidden)]
#[macro_export]
macro_rules! __detail_macro {
    (@def_custom_macro $name:ident $custom_subcase:ident [$dollar:tt] $($meta:meta)*) => {
        $(#[$meta])*
        macro_rules! $name {
            (
                $dollar ($body:tt)*
            ) => {
                $crate::__detail_macro! { @transform_fn $custom_subcase $dollar($body)* }
            };
        }
    };
    (@def_inner_macro $name:ident $exec_path:ident $state:ident [$dollar:tt]) => {
        macro_rules! $name {
            ($dollar ($body:tt)*) => {
                if $state.enter_subcase(&mut $exec_path) {
                    $dollar ($body)*
                };
                $state.exit_subcase();
            }
        }
    };
    (
        @transform_fn $custom_subcase:ident
        $(
            $( #[$meta:meta] )*
            $vis:vis fn $name:ident ( $( $arg:ident : $arg_t:ty ),* $(,)? )
            $( -> $ret_t:ty )?
            {
                $($body:tt)*
            }
        )+
    ) => {
        $(
            $( #[$meta] )*
            $vis fn $name ( $( $arg : $arg_t ),* ) $( -> $ret_t )? {

                let mut exec_path = $crate::__detail::TreePath::default();
                let mut state = $crate::__detail::State::default();

                $crate::__detail_macro! { @def_inner_macro $custom_subcase exec_path state [$] };

                let mut ret $(: $ret_t)? = {
                    $($body)*
                };
                state.update_exec_path(&mut exec_path);
                state.clear();

                let mut first = true;
                while !exec_path.is_empty() {
                    ret = {
                        $($body)*
                    };
                    state.update_exec_path(&mut exec_path);
                    state.clear();
                }
                ret
            }
        )+
    };
}