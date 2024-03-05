use std::marker::PhantomData;

pub trait Status {
    type Status;
    fn initial() -> Self::Status;
    fn is_final(status: &Self::Status) -> bool;
}

pub trait Transition {
    type Status;
    type Alphabet;
    fn next(status: &Self::Status, alphabet: &Self::Alphabet) -> Self::Status;
}

pub trait Automaton<Alphabet> {
    fn reset(&mut self);
    fn execute(&mut self, input: &[Alphabet]) -> bool;
}

pub struct FiniteStateMachine<S, A, T> {
    status: S,
    alphabet: PhantomData<A>,
    transition: PhantomData<T>,
}

impl<S, A, T> FiniteStateMachine<S, A, T>
where
    S: Status<Status = S>,
{
    pub fn new() -> Self {
        Self {
            status: S::initial(),
            alphabet: PhantomData,
            transition: PhantomData,
        }
    }
}

impl<S, A, T> Automaton<A> for FiniteStateMachine<S, A, T>
where
    S: Status<Status = S>,
    T: Transition<Status = S, Alphabet = A>,
{
    fn reset(&mut self) {
        self.status = S::initial();
    }
    fn execute(&mut self, input: &[A]) -> bool {
        for i in input {
            self.status = T::next(&self.status, i);
        }
        S::is_final(&self.status)
    }
}

#[cfg(test)]
mod fsa {
    use super::*;

    #[derive(PartialEq)]
    enum A {
        Zero,
        One,
    }

    fn encode(n: usize) -> Vec<A> {
        let mut v = Vec::new();
        let mut n = n;
        while n > 0 {
            v.push(if n % 2 == 0 { A::Zero } else { A::One });
            n /= 2;
        }
        v
    }

    #[test]
    fn power_of_2() {
        #[derive(PartialEq)]
        enum S {
            S1,
            S2,
            S3,
        }
        impl Status for S {
            type Status = S;
            fn initial() -> S {
                S::S1
            }
            fn is_final(status: &S) -> bool {
                *status == S::S2
            }
        }
        struct T;
        impl Transition for T {
            type Status = S;
            type Alphabet = A;
            fn next(status: &S, input: &A) -> S {
                match (status, input) {
                    (S::S1, A::Zero) => S::S1,
                    (S::S1, A::One) => S::S2,
                    (S::S2, A::Zero) => S::S3,
                    (S::S2, A::One) => S::S3,
                    (S::S3, A::Zero) => S::S3,
                    (S::S3, A::One) => S::S3,
                }
            }
        }
        for i in 1..=16 {
            let expect = (i & (i - 1)) == 0;
            let input = encode(i);
            let mut fsa: FiniteStateMachine<S, A, T> = FiniteStateMachine::new();
            let output = fsa.execute(&input);
            assert_eq!(output, expect);
        }
    }
}
