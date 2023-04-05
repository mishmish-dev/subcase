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
    (@def_inner_macro $name:ident $state:ident [$dollar:tt]) => {
        macro_rules! $name {
            ($dollar ($body:tt)*) => {
                if $state.enter_subcase() {
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
            $vis:vis fn $name:ident( $( $arg:ident : $arg_t:ty ),* $(,)? )
            $( -> $ret_t:ty )?
            {
                $($body:tt)*
            }
        )+
    ) => {
        $(
            $( #[$meta] )*
            $vis fn $name( $( $arg : $arg_t ),* ) $( -> $ret_t )? {
                use $crate::__detail::State;
                let mut state = State::default();

                fn run_one_exec_path( state: &mut State, $( $arg : $arg_t ),* ) $( -> $ret_t )? {
                    $crate::__detail_macro! { @def_inner_macro $custom_subcase state [$] };
                    $($body)*
                }

                let mut ret = run_one_exec_path(&mut state, $( $arg ),*);
                state.update_exec_path();

                while !state.is_done() {
                    ret = run_one_exec_path(&mut state, $( $arg ),*);
                    state.update_exec_path();
                }
                ret
            }
        )+
    };
}