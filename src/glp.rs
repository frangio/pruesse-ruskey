use std::{rc::Rc, ops::Deref};

pub trait GLPSubProc {
    type Delta;
    fn size(&self) -> usize;
    fn execute(&mut self, i: usize) -> (bool, Self::Delta);
}

trait GLPIterator {
    fn start(n: usize) -> Self;
    fn next<SP: GLPSubProc>(&mut self, proc: &mut SP) -> Option<SP::Delta>;
}

struct GLPLoopFree {
    p: Vec<usize>,
}

impl GLPIterator for GLPLoopFree {
    fn start(n: usize) -> Self {
        let p = vec![0; n];
        GLPLoopFree { p }
    }

    fn next<SP: GLPSubProc>(&mut self, proc: &mut SP) -> Option<SP::Delta> {
        let n = self.p.len();
        let i = self.p.get(0).copied().filter(|&i| i < n)?;
        let (vi, d) = proc.execute(i);
        if vi {
            self.p[i] = 0;
        } else if i == n - 1 {
            self.p[i] = n;
        } else if self.p[i + 1] == 0 {
            self.p[i] = i + 1;
        } else {
            self.p[i] = self.p[i + 1];
            self.p[i + 1] = 0;
        }
        if i > 0 {
            self.p[0] = 0;
        }
        Some(d)
    }
}

struct GLPIter<SP: GLPSubProc> {
    proc: SP,
    iter: GLPLoopFree,
}

impl<SP: GLPSubProc> GLPIter<SP> {
    fn run(proc: SP) -> Self {
        let n = proc.size();
        let iter = GLPLoopFree::start(n);
        GLPIter { proc, iter }
    }
}

impl<SP: GLPSubProc> Iterator for GLPIter<SP> {
    type Item = SP::Delta;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next(&mut self.proc)
    }
}

struct GLPState<SP: GLPSubProc>(Rc<SP>);

impl<SP: GLPSubProc> GLPState<SP> {
    fn new(proc: SP) -> Self {
        GLPState(Rc::new(proc))
    }
}

impl<SP: GLPSubProc> Clone for GLPState<SP> {
    fn clone(&self) -> Self {
        GLPState(self.0.clone())
    }
}

impl<SP: GLPSubProc> Deref for GLPState<SP> {
    type Target = SP;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<SP: GLPSubProc> GLPSubProc for GLPState<SP> {
    type Delta = SP::Delta;

    fn size(&self) -> usize {
        self.0.size()
    }

    fn execute(&mut self, i: usize) -> (bool, Self::Delta) {
        Rc::get_mut(&mut self.0).unwrap().execute(i)
    }
}

struct GLPIterStates<SP: GLPSubProc> {
    started: bool,
    inner: GLPIter<GLPState<SP>>,
}

impl<SP: GLPSubProc> GLPIterStates<SP> {
    fn run(proc: SP) -> Self {
        let inner = GLPIter::run(GLPState::new(proc));
        GLPIterStates { inner, started: false }
    }
}

impl<SP: GLPSubProc> Iterator for GLPIterStates<SP> {
    type Item = GLPState<SP>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            self.inner.next()?;
        } else {
            self.started = true;
        }
        Some(self.inner.proc.clone())
    }
}

pub fn deltas<SP: GLPSubProc>(proc: SP) -> impl Iterator<Item = SP::Delta> {
    GLPIter::run(proc)
}

pub fn states<SP: GLPSubProc>(proc: SP) -> impl Iterator<Item = impl Deref<Target = SP>> {
    GLPIterStates::run(proc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_codes() {
        struct GrayCode(Vec<bool>);

        impl GrayCode {
            fn new(n: usize) -> Self {
                GrayCode(vec![false; n])
            }

            fn bits(&self) -> String {
                self.0.iter().rev().map(|&b| if b { "1" } else { "0" }).collect::<String>()
            }
        }

        impl GLPSubProc for GrayCode {
            type Delta = ();

            fn size(&self) -> usize {
                self.0.len()
            }

            fn execute(&mut self, i: usize) -> (bool, Self::Delta) {
                self.0[i] = !self.0[i];
                (false, ())
            }
        }

        fn gray_codes(n: usize) -> Vec<String> {
            states(GrayCode::new(n)).map(|g| g.bits()).collect()
        }

        assert_eq!(
            gray_codes(2),
            vec!["00", "01", "11", "10"],
        );

        assert_eq!(
            gray_codes(3),
            vec!["000", "001", "011", "010", "110", "111", "101", "100"],
        );
    }
}
