pub struct LazyInit<'a, T> {
    t: Option<T>,
    t_maker: Option<Box<dyn FnOnce() -> T + 'a>>,
}

impl<'a, T> LazyInit<'a, T> {
    pub fn new<F: FnOnce() -> T + 'a>(t_maker: F) -> Self {
        Self {
            t: None,
            t_maker: Some(Box::new(t_maker)),
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