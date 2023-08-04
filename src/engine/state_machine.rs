use raylib::RaylibHandle;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub struct StateMachine<T, O> {
    pub update: HashMap<T, fn(&mut O, &mut RaylibHandle)>,
    pub enter: HashMap<T, fn(&mut O, &mut RaylibHandle)>,
    pub exit: HashMap<T, fn(&mut O, &mut RaylibHandle)>,
}

impl<T, O> StateMachine<T, O>
where
    T: PartialEq + Eq + Hash + Copy + Default + Debug,
{
    pub fn new() -> Self {
        StateMachine {
            update: HashMap::<T, fn(&mut O, &mut RaylibHandle)>::new(),
            enter: HashMap::<T, fn(&mut O, &mut RaylibHandle)>::new(),
            exit: HashMap::<T, fn(&mut O, &mut RaylibHandle)>::new(),
        }
    }

    pub fn add_state(
        &mut self,
        state: T,
        update_fn: fn(&mut O, &mut RaylibHandle),
        enter_fn: fn(&mut O, &mut RaylibHandle),
        exit_fn: fn(&mut O, &mut RaylibHandle),
    ) {
        self.update.insert(state, update_fn);
        self.enter.insert(state, enter_fn);
        self.exit.insert(state, exit_fn);
    }
}

impl<T, O> Default for StateMachine<T, O> {
    fn default() -> Self {
        Self {
            update: HashMap::new(),
            enter: HashMap::new(),
            exit: HashMap::new(),
        }
    }
}
