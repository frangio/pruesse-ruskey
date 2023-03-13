use crate::glp::GLPSubProc;

#[derive(Debug)]
pub struct GrayCode(Vec<bool>);

impl GrayCode {
    pub fn bits(&self) -> String {
        self.0.iter().map(|&b| if b { "1" } else { "0" }).collect::<String>()
    }
}

impl GLPSubProc for GrayCode {
    type Input = usize;
    type Delta = ();

    fn start(n: usize) -> (usize, GrayCode) {
        (n, GrayCode(vec![false; n]))
    }

    fn execute(&mut self, i: usize) -> (bool, Self::Delta) {
        self.0[i] = !self.0[i];
        (false, ())
    }
}

