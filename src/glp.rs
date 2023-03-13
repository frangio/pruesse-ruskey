use std::{rc::Rc, ops::Deref};

pub trait GLPSubProc {
    type Input;
    type Delta;
    fn start(input: Self::Input) -> (usize, Self);
    fn execute(&mut self, i: usize) -> (bool, Self::Delta);
}

struct GLPState<SP: GLPSubProc>(Rc<SP>);

impl<SP: GLPSubProc> Clone for GLPState<SP> {
    fn clone(&self) -> Self {
        GLPState(self.0.clone())
    }
}

impl<SP: GLPSubProc> GLPSubProc for GLPState<SP> {
    type Input = SP::Input;
    type Delta = SP::Delta;

    fn start(input: Self::Input) -> (usize, Self) {
        let (n, proc) = SP::start(input);
        (n, GLPState(Rc::new(proc)))
    }

    fn execute(&mut self, i: usize) -> (bool, Self::Delta) {
        Rc::get_mut(&mut self.0).unwrap().execute(i)
    }
}

trait GLPIterator {
    fn start(n: usize) -> Self;
    fn next<SP: GLPSubProc>(&mut self, proc: &mut SP) -> Option<SP::Delta>;
}

pub struct GLPLoop {
    v: Vec<bool>,
}

impl GLPIterator for GLPLoop {
    fn start(n: usize) -> Self {
        let v = vec![true; n];
        GLPLoop { v }
    }

    fn next<SP: GLPSubProc>(&mut self, proc: &mut SP) -> Option<SP::Delta> {
        let i = self.v.iter().position(|&vi| vi)?;
        let (vi, d) = proc.execute(i);
        self.v[i] = vi;
        for j in 0..i {
            self.v[j] = true;
        }
        Some(d)
    }
}

pub struct GLPLoopFree {
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

struct GLPIter<SP: GLPSubProc, I: GLPIterator = GLPLoopFree> {
    proc: SP,
    iter: I,
}

impl<SP: GLPSubProc, I: GLPIterator> GLPIter<SP, I> {
    fn start(input: SP::Input) -> Self {
        let (n, proc) = SP::start(input);
        let iter = I::start(n);
        GLPIter { proc, iter }
    }
}

impl<SP: GLPSubProc, I: GLPIterator> Iterator for GLPIter<SP, I> {
    type Item = SP::Delta;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next(&mut self.proc)
    }
}

struct GLPIterStates<SP: GLPSubProc, I: GLPIterator = GLPLoopFree> {
    started: bool,
    inner: GLPIter<GLPState<SP>, I>,
}

impl<SP: GLPSubProc, I: GLPIterator> GLPIterStates<SP, I> {
    fn start(input: SP::Input) -> Self {
        let inner = GLPIter::start(input);
        GLPIterStates { inner, started: false }
    }
}

impl<SP: GLPSubProc> Deref for GLPState<SP> {
    type Target = SP;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<SP: GLPSubProc, I: GLPIterator> Iterator for GLPIterStates<SP, I> {
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


pub fn deltas<SP: GLPSubProc>(input: SP::Input) -> impl Iterator<Item = SP::Delta> {
    GLPIter::<SP>::start(input)
}

pub fn states<SP: GLPSubProc>(input: SP::Input) -> impl Iterator<Item = impl Deref<Target = SP>> {
    GLPIterStates::<SP, GLPLoop>::start(input)
}
