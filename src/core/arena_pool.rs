use std::{cell::UnsafeCell, marker::PhantomData, ops};

use bumpalo::Bump;

use crate::core::push_get::PushGet;

// TODO: Test test test! There is so much unsafe code it really needs to be analyzed and tested
#[derive(Debug)]
struct ReusableArena {
    arena: Box<Bump>,
    in_use: bool,
}

impl ReusableArena {
    fn new() -> Self {
        Self { arena: Box::new(Bump::new()), in_use: false }
    }

    fn to_guard<'a>(&'a mut self) -> ArenaGuard<'a> {
        ArenaGuard {
            arena: &mut *self.arena as *mut Bump,
            in_use: &mut self.in_use as *mut bool,
            _pool: PhantomData,
        }
    }
}

pub struct ArenaPool {
    arenas: UnsafeCell<Vec<ReusableArena>>,
    _single_threaded: PhantomData<*const ()>,
}

impl ArenaPool {
    pub fn new() -> Self {
        Self { arenas: UnsafeCell::new(Vec::new()), _single_threaded: PhantomData }
    }

    pub fn acquire<'pool, 'arena>(&'pool self) -> ArenaGuard<'arena>
    where
        'pool: 'arena,
    {
        unsafe {
            let arenas = &mut *self.arenas.get();

            let open_arena_idx = arenas.iter().position(|a| !a.in_use);

            if let Some(idx) = open_arena_idx {
                let reuse_arena = &mut arenas[idx];
                reuse_arena.in_use = true;
                reuse_arena.to_guard()
            } else {
                let reuse_arena = ReusableArena::new();
                let reuse_arena = arenas.push_get_mut(reuse_arena);
                reuse_arena.to_guard()
            }
        }
    }
}

pub struct ArenaGuard<'a> {
    arena: *mut Bump,
    in_use: *mut bool,
    _pool: PhantomData<&'a ArenaPool>,
}

impl<'a> ops::Deref for ArenaGuard<'a> {
    type Target = Bump;

    #[inline(always)]
    fn deref(&self) -> &'a Self::Target {
        unsafe { &*self.arena }
    }
}

impl<'a> Drop for ArenaGuard<'a> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            (*self.arena).reset();
            *self.in_use = false;
        }
    }
}
