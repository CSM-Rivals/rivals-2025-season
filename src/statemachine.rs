use std::fmt::Debug;

use phf::PhfHash;

trait StateName{}

struct State<T: 'static> {
    execute: Box<dyn Fn()>,
    next_state: Box<dyn Fn() -> &'static T>,
}

enum MyMachine {
    Foo,
    Bar,
}


impl StateName for MyMachine {}

struct Machine<T: 'static + std::cmp::Eq + PhfHash + phf_shared::PhfBorrow<T>> where T: StateName {
    states: &'static phf::Map<T, State<T>>,
    current_state: &'static T,
}

impl<T: 'static + std::cmp::Eq + PhfHash + phf_shared::PhfBorrow<T> + StateName> Machine<T> {
    fn step(&mut self) {
        let state = self.states.get(self.current_state).expect("BUG! requested state does not exist");
        state.execute.as_ref()();
        self.current_state = state.next_state.as_ref()();
    }
}

const test: Machine<MyMachine> = {

};