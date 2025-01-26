#![allow(unused)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

fn main() {
    let mut scheduler = Scheduler {
        systems: vec![],
        resources: HashMap::default(),
    };

    scheduler.add_resource(52);
    scheduler.add_system(|i: i32| println!("{}", i));

    scheduler.run();
}

struct Scheduler {
    systems: Vec<StoredSystem>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}
impl Scheduler {
    fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.run(&mut self.resources);
        }
    }

    fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>) {
        self.systems.push(Box::new(system.into_system()));
    }

    fn add_resource<R: 'static>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }
}

type StoredSystem = Box<dyn System>;

struct FunctionSystem<F, Input> {
    f: F,
    marker: PhantomData<Input>,
}

trait System {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>);
}

impl<F: FnMut()> System for FunctionSystem<F, ()> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        (self.f)()
    }
}

impl<F: FnMut(T1), T1: 'static> System for FunctionSystem<F, (T1,)> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        let t1 = *resources
            .remove(&TypeId::of::<T1>())
            .unwrap()
            .downcast::<T1>()
            .unwrap();
        (self.f)(t1)
    }
}

trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

impl<F: FnMut()> IntoSystem<()> for F {
    type System = FunctionSystem<Self, ()>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}

impl<F: FnMut(T1), T1: 'static> IntoSystem<(T1,)> for F {
    type System = FunctionSystem<Self, (T1,)>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
