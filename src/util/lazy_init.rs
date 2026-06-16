use std::marker::PhantomData;

pub struct LazyInit<'a, T, F: FnOnce() -> T + 'a> {
    t: Option<T>,
    t_maker: Option<F>,
    _phantom: PhantomData<&'a F>,
}

impl<'a, T, F: FnOnce() -> T + 'a> LazyInit<'a, T, F> {
    pub fn new(t_maker: F) -> Self {
        Self {
            t: None,
            t_maker: Some(t_maker),
            _phantom: PhantomData,
        }
    }

    pub fn get_or_init(&mut self) -> &mut T {
        let t = &mut self.t;
        if let Some(t) = t {
            t
        } //
        else {
            *t = Some(self.t_maker.take().unwrap()());
            t.as_mut().unwrap()
        }
    }

    pub fn get(self) -> Option<T> {
        self.t
    }
}