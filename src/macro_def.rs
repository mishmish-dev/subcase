/// Allows to fork a function's execution
/// to create multiple paths which share code, for example,
/// test case setup/teardown.
///
/// Macro body can contain one or more function definition.
/// Functions are restricted to not to return anything.
///
/// For usage, please refer to the crate doc.
#[macro_export]
macro_rules! with_subcases {
    (@def_sub $sub_name:ident [$dollar:tt] ($exec_path:ident, $state:ident)) => {
        macro_rules! $sub_name {
            ($dollar ($sub_body:tt)*) => {
                if $state.enter_subcase(&mut $exec_path) {
                    $dollar ($sub_body)*
                };
                $state.exit_subcase();
            }
        }
    };
    (
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

                let mut exec_path = $crate::detail::TreePath::default();
                let mut state = $crate::detail::State::default();

                with_subcases! { @def_sub subcase [$] (exec_path, state)};

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
    }
}
