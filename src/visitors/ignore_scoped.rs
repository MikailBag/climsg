
pub struct IgnoreScoped<T>(T);

impl<T: crate::Visitor + Clone> IgnoreScoped<T> {
    pub fn new(inner: T) -> IgnoreScoped<T> {
        IgnoreScoped(inner)
    }
}

impl<T> IgnoreScoped<T> {
    pub fn get_ref(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: crate::Visitor> crate::Visitor for IgnoreScoped<T> {
    fn visit_message(&self, message: crate::Message<'_>) {
        self.get_ref().visit_message(message)
    }
}

impl<T: crate::Visitor + Clone> crate::ScopedVisitor for IgnoreScoped<T> {
    fn scoped(&self, _scope: &str,) -> Self {
        Self(self.get_ref().clone())
    }
}