#[macro_export]
macro_rules! with_subcases {
    (
        $(
            $( #[$meta:meta] )*
            $vis:vis fn $name:ident ( $( $arg:ident : $arg_t:ty ),* $(,)? ) {
                $($body:tt)*
            }
        )+
    ) => {
        $(
            $( #[$meta] )*
            $vis fn $name ( $( $arg : $arg_t ),* ) {

                let mut exec_path = $crate::detail::TreePath::new();
                let mut state = $crate::detail::SubcasesState::default();

                macro_rules! subcase {
                    ($block:block) => {
                        if state.enter_subcase(&mut exec_path)
                            $block
                        ;
                        state.exit_subcase();
                    }
                }

                let mut first = true;
                while first || !exec_path.is_empty() {
                    {
                        $($body)*
                    }

                    state.update_exec_path(&mut exec_path);
                    state.clear();
                    first = false;
                }
            }
        )+
    }
}
