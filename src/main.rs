#![allow(unused)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

type StoredSystem = Box<dyn System>;
fn main() {
    let mut sch = Scheduler {
        systems: vec![],
        resources: HashMap::default(),
    };

    sch.add_system(|| println!("{}", "Hello World!"));
    // sch.add_system(|i: u32| println!("{}", i));
    // sch.add_resource(32u32);

    sch.run();
}

struct Scheduler {
    systems: Vec<StoredSystem>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Scheduler {
    fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.run(&mut self.resources)
        }
    }

    fn add_system<S: System + 'static>(&mut self, system: impl IntoSystem<System = S>) {
        self.systems.push(Box::new(system.into_system()));
    }

    fn add_resource<R: 'static>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }
}

struct FunctionSystem<F, Input> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

trait System {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>);
}

trait IntoSystem {
    type System: System;

    fn into_system(self) -> Self::System;
}

impl<F: FnMut()> System for FunctionSystem<F, ()> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        (self.f)()
    }
}

impl<F: FnMut()> IntoSystem for F {
    type System = FunctionSystem<Self, ()>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
