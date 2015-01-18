#![feature(unsafe_destructor)]
#![allow(unstable)]

use std::marker;
use std::mem::transmute;
use std::thread::{ Thread, JoinGuard };
use std::raw;

mod test;

pub type TaskBody<'s> = &'s mut (FnMut() + Sync + 's);

pub struct Section<'s> {
    marker: marker::ContravariantLifetime<'s>,
    tasks: Vec<JoinGuard<'s, ()>>
}

pub struct RawClosurePtr(*mut raw::Closure);
impl Copy for RawClosurePtr {}
unsafe impl Send for RawClosurePtr {}

pub fn execute<'s>(closures: &'s mut [TaskBody<'s>]) {
    let mut join = Section::new();
    for closure in closures.iter_mut() {
        join.fork(closure);
    }
    join.sync();
}

impl<'s> Section<'s> {
    pub fn new() -> Section<'s> {
        Section { marker: marker::ContravariantLifetime,
                  tasks: Vec::new() }
    }

    pub fn fork(&mut self, body: &'s mut TaskBody<'s>) {
        unsafe {
            let body: RawClosurePtr = transmute(body);

            // really don't want the `push` below to fail
            // after task has been spawned
            self.tasks.reserve(1);

            let future = Thread::scoped(move || {
                let body: &mut TaskBody = transmute(body);
                (*body)()
            });

            // due to reserve above, should be infallible
            self.tasks.push(future);
        }
    }

    pub fn sync(&mut self) {
        loop {
            match self.tasks.pop() {
                None => { break; }
                Some(task) => {
                    // propagate any failure
                    match task.join() {
                        Ok(()) => { }
                        Err(e) => { panic!(e) }
                    }
                }
            }
        }
    }
}

#[unsafe_destructor]
impl<'s> Drop for Section<'s> {
    fn drop(&mut self) {
        self.sync();
    }
}
