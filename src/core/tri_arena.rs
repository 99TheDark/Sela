use std::marker::PhantomData;

use bumpalo::Bump;

use crate::core::span::Span;

// For better cache locality
pub struct TriArena<T: Sized> {
    elems: Bump,
    spans: Bump,
    misc: Bump,
    _phantom: PhantomData<T>,
}

impl<T: Sized> TriArena<T> {
    pub fn new() -> Self {
        Self {
            elems: Bump::new(),
            spans: Bump::new(),
            misc: Bump::new(),
            _phantom: PhantomData,
        }
    }

    pub fn alloc(&self, elem: T, span: Span) -> (&mut T, &mut Span) {
        let elem = self.elems.alloc(elem);
        let span = self.spans.alloc(span);
        (elem, span)
    }

    pub fn alloc_misc<U>(&self, elem: U) -> &U {
        self.misc.alloc(elem)
    }

    pub fn reset(&mut self) {
        self.elems.reset();
        self.spans.reset();
        self.elems.reset();
    }
}
