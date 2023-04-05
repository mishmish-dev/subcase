use super::fixed_vec::FixedVec;

const MAX_SUBCASE_NESTING: usize = 16;

pub type TreePath = FixedVec<MAX_SUBCASE_NESTING>;

#[derive(Default)]
pub struct State {
    depth: usize,
    path: TreePath,
    exec_path: TreePath,
}

impl State {
    pub fn enter_subcase(&mut self) -> bool {
        if self.exec_path.len() <= self.depth {
            self.exec_path.push();
        }
        if self.path.len() <= self.depth {
            self.path.push();
        } else {
            self.path.increment_at(self.depth);
        }
        self.depth += 1;

        self.exec_path.coincides_till(&self.path, self.depth)
    }

    pub fn exit_subcase(&mut self) {
        self.depth -= 1;
    }

    pub fn update_exec_path(&mut self) {
        while !self.exec_path.is_empty() {
            let i = self.exec_path.len() - 1;
            if self.exec_path.get(i) < self.path.get(i) {
                self.exec_path.increment_at(i);
                break;
            } else {
                self.exec_path.pop();
            }
        }
        self.path.clear();
    }

    pub fn is_done(&mut self) -> bool {
        self.exec_path.is_empty()
    }
}
