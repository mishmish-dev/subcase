const PRE_RESERVE_TREEPATH: usize = 8;

type TreePath = Vec<u32>;
type TreePathLabels = Vec<&'static str>;

pub struct State {
    depth: usize,
    path: TreePath,
    exec_path: TreePath,
    subcase_labels: TreePathLabels,
}

impl State {
    pub fn new() -> Self {
        State {
            depth: 0,
            path: Vec::with_capacity(PRE_RESERVE_TREEPATH),
            exec_path: Vec::with_capacity(PRE_RESERVE_TREEPATH),
            subcase_labels: Vec::with_capacity(PRE_RESERVE_TREEPATH),
        }
    }

    pub fn enter_subcase(&mut self) -> bool {
        self.depth += 1;
        if self.exec_path.len() < self.depth {
            self.exec_path.push(0u32);
        }

        if self.path.len() < self.depth {
            self.path.push(0u32);
        } else {
            self.path[self.depth - 1] += 1;
        }
        self.exec_path[self.depth - 1] == self.path[self.depth - 1]
    }

    pub fn set_label(&mut self, label: &'static str) {
        if self.subcase_labels.len() < self.depth {
            self.subcase_labels.push(label);
        } else {
            self.subcase_labels[self.depth - 1] = label;
        }
    }

    pub fn exit_subcase(&mut self) {
        self.depth -= 1;
    }

    pub fn prepare_for_next_run(&mut self) -> bool {
        let mut ret = false;
        while !self.exec_path.is_empty() {
            let i = self.exec_path.len() - 1;
            if self.exec_path[i] < self.path[i] {
                self.exec_path[i] += 1;
                ret = true;
                break;
            } else {
                self.exec_path.pop();
            }
        }
        self.path.clear();
        self.subcase_labels.clear();
        ret
    }

    pub fn report_exec_path(self) -> String {
        std::iter::zip(self.subcase_labels, self.exec_path)
            .map(|(label, subcase_num)| std::format!("{} (#{})", label, subcase_num))
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

pub fn run_test_case(modified_test_body: fn(&mut State)) {
    let state = std::sync::Mutex::new(State::new());
    let test_run = || modified_test_body(&mut *state.lock().unwrap());
    loop {
        if let Err(_) = std::panic::catch_unwind(test_run) {
            panic!(
                "test execution path failed executing path: {}",
                state
                    .into_inner()
                    .unwrap_or_else(|m| m.into_inner()) // Ignore poisoning
                    .report_exec_path()
            );
        }
        if !state.lock().unwrap().prepare_for_next_run() {
            return;
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __detail_macro {
    (
        @def_custom_macro $macro_name:ident $subcase_macro:ident
        [$dollar:tt] $($macro_meta:meta)*
    ) => {
        $(#[$macro_meta])*
        macro_rules! $macro_name {
            (
                $dollar(
                    $dollar( #[$meta:meta] )*
                    $vis:vis fn $name:ident() {
                        $dollar($body:tt)*
                    }
                )+
            ) => {
                $crate::__detail_macro! {
                    @transform_fn $subcase_macro
                    $dollar(
                        $dollar( #[$meta] )*
                        $vis fn $name() {
                            $dollar($body)*
                        }
                    )+
                }
            };
        }
    };
    (@def_inner_macro $name:ident $state:ident [$dollar:tt]) => {
        macro_rules! $name {
            (
                ~$label:literal
                $dollar($body:tt)*
            ) => {
                if $state.enter_subcase() {
                    $state.set_label($label);
                    $dollar($body)*
                };
                $state.exit_subcase();
            };
            ($dollar($body:tt)*) => {
                $name! { ~"[unnamed]" $dollar($body)* }
            };
        }
    };
    (
        @transform_fn $subcase_macro:ident
        $(
            $( #[$meta:meta] )*
            $vis:vis fn $name:ident() {
                $($body:tt)*
            }
        )+
    ) => {
        $(
            $( #[$meta] )*
            $vis fn $name() {
                $crate::__detail::run_test_case(|state| {
                    $crate::__detail_macro! {
                        @def_inner_macro $subcase_macro state [$]
                    };
                    $($body)*
                })
            }
        )+
    };
}
