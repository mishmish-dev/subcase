use super::fixed_vec::FixedVec;

const MAX_SUBCASE_NESTING: usize = 16;

pub type TreePath = FixedVec<MAX_SUBCASE_NESTING>;

#[derive(Default)]
pub struct State {
    depth: usize,
    path: TreePath,
}

impl State {
    pub fn enter_subcase(&mut self, exec_path: &mut TreePath) -> bool {
        if exec_path.len() <= self.depth {
            exec_path.push();
        }
        if self.path.len() <= self.depth {
            self.path.push();
        } else {
            self.path.increment_at(self.depth);
        }
        self.depth += 1;

        exec_path.coincides_till(&self.path, self.depth)
    }

    pub fn exit_subcase(&mut self) {
        self.depth -= 1;
    }

    pub fn update_exec_path(&mut self, exec_path: &mut TreePath) {
        while !exec_path.is_empty() {
            let i = exec_path.len() - 1;
            if exec_path.get(i) < self.path.get(i) {
                exec_path.increment_at(i);
                return;
            } else {
                exec_path.pop();
            }
        }
    }

    pub fn clear(&mut self) {
        self.depth = 0;
        self.path.clear();
    }
}
