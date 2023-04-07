const PRE_RESERVE_TREEPATH: usize = 8;

pub type TreePath = Vec<u32>;
use crate::ErrTestable;

pub struct State {
    depth: usize,
    path: TreePath,
    exec_path: TreePath,
}

impl State {
    pub fn new() -> Self {
        State {
            depth: 0,
            path: TreePath::with_capacity(PRE_RESERVE_TREEPATH),
            exec_path: TreePath::with_capacity(PRE_RESERVE_TREEPATH),
        }
    }

    pub fn enter_subcase(&mut self) -> bool {
        if self.exec_path.len() <= self.depth {
            self.exec_path.push(0u32);
        }
        if self.path.len() <= self.depth {
            self.path.push(0u32);
        } else {
            self.path[self.depth] += 1;
        }
        let ret = self.exec_path[self.depth] == self.path[self.depth];
        self.depth += 1;
        ret
    }

    pub fn exit_subcase(&mut self) {
        self.depth -= 1;
    }

    pub fn prepare_for_next_run(&mut self) -> bool {
        while !self.exec_path.is_empty() {
            let i = self.exec_path.len() - 1;
            if self.exec_path[i] < self.path[i] {
                self.exec_path[i] += 1;
                self.path.clear();
                return true;
            } else {
                self.exec_path.pop();
            }
        }
        false
    }

    pub fn report_exec_path(self) -> TreePath {
        self.exec_path
    }
}

pub fn run_test_case<Ret: ErrTestable>(modified_test_body: fn(&mut State) -> Ret) {
    let state = std::sync::Mutex::new(State::new());
    let test_run = || modified_test_body(&mut *state.lock().unwrap());
    loop {
        let ret = std::panic::catch_unwind(test_run);
        if ret.map_or(true, |r| ErrTestable::is_err(&r)) {
            panic!(
                "test execution path failed executing path {:?}",
                state
                    .into_inner()
                    .unwrap_or_else(|m| m.into_inner()) // Ignore poisoning
                    .report_exec_path()
            );
        }
        if !state.lock().unwrap().prepare_for_next_run() {
            return
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __detail_macro {
    (@def_custom_macro $name:ident $subcase_macro:ident [$dollar:tt] $($meta:meta)*) => {
        $(#[$meta])*
        macro_rules! $name {
            (
                $dollar ($body:tt)*
            ) => {
                $crate::__detail_macro! { @transform_fn $subcase_macro $dollar($body)* }
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
        @transform_fn $subcase_macro:ident
        $(
            $( #[$meta:meta] )*
            $vis:vis fn $name:ident() $( -> $ret_t:ty )? {
                $($body:tt)*
            }
        )+
    ) => {
        $(
            $( #[$meta] )*
            $vis fn $name() {
                $crate::__detail::run_test_case(|state| $( -> $ret_t )? {
                    $crate::__detail_macro! {
                        @def_inner_macro $subcase_macro state [$]
                    };
                    $($body)*
                })
            }
        )+
    };
}
