pub type TreePath = Vec<usize>;

#[derive(Default)]
pub struct SubcasesState {
    depth: usize,
    path: TreePath,
}

impl SubcasesState {
    pub fn enter_subcase(&mut self, exec_path: &mut TreePath) -> bool {
        if exec_path.len() <= self.depth {
            exec_path.push(0);
        }
        if self.path.len() <= self.depth {
            self.path.push(0);
        } else {
            self.path[self.depth] += 1;
        }
        self.depth += 1;

        exec_path[0..self.depth] == self.path[0..self.depth]
    }

    pub fn exit_subcase(&mut self) {
        self.depth -= 1;
    }

    pub fn update_exec_path(&mut self, exec_path: &mut TreePath) {
        while !exec_path.is_empty() {
            let i = exec_path.len() - 1;
            if exec_path[i] < self.path[i] {
                exec_path[i] += 1;
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
