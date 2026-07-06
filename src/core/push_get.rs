pub trait PushGet<T> {
    fn push_get(&mut self, value: T) -> &T;
    fn push_get_mut(&mut self, value: T) -> &mut T;
}

impl<T> PushGet<T> for Vec<T> {
    #[inline(always)]
    fn push_get(&mut self, value: T) -> &T {
        self.push(value);
        let last_idx = self.len() - 1;
        unsafe { self.get_unchecked(last_idx) }
    }

    #[inline(always)]
    fn push_get_mut(&mut self, value: T) -> &mut T {
        self.push(value);
        let last_idx = self.len() - 1;
        unsafe { self.get_unchecked_mut(last_idx) }
    }
}
