mod macros;
pub mod visitors;

pub use visitors::{JsonVisitor, TextVisitor};

/// Message is single piece of information to show to user.
/// Message can be presented in two ways: as JSON
/// or as text.
pub struct Message<'a> {
    text: &'a dyn std::fmt::Display,
    structured: &'a dyn erased_serde::Serialize,
}

impl<'a> Message<'a> {
    /// Creates new Message from its text and structured representation
    pub fn new(
        text: &'a dyn std::fmt::Display,
        structured: &'a dyn erased_serde::Serialize,
    ) -> Message<'a> {
        Message { text, structured }
    }
}
/// Types that can be converted to message
///
/// This trait has two impls:
///  - `Message` can be converted to itself
///  - Value of type, implementing `Serialize` and `Display`.
pub trait AsMessage {
    fn as_message<'a>(&'a self) -> Message<'a>;
}

impl AsMessage for Message<'_> {
    fn as_message<'a>(&'a self) -> Message<'a> {
        Message {
            text: self.text,
            structured: self.structured,
        }
    }
}

impl<T: std::fmt::Display + serde::Serialize> AsMessage for T {
    fn as_message<'a>(&'a self) -> Message<'a> {
        Message {
            structured: self,
            text: self,
        }
    }
}

/// Visitor of Message stream: it receives each message and displays to user
pub trait Visitor: Send + Sync {
    /// Processes the message somehow
    fn visit_message(&self, message: Message<'_>);
}

pub trait ScopedVisitor: Visitor {
    /// Creates new visitor that can use `scope` to change its configuration
    fn scoped(&self, scope: &str) -> Self;
}

/// Thread-safe version of `ScopedVisitor`
pub trait DynScopedVisitor: Visitor {
    fn scoped(&self, scope: &str) -> Box<dyn DynScopedVisitor>;
}

impl<V: ScopedVisitor + 'static> DynScopedVisitor for V {
    fn scoped(&self, scope: &str) -> Box<dyn DynScopedVisitor> {
        Box::new(self.scoped(scope))
    }
}

impl DynScopedVisitor for Box<dyn DynScopedVisitor> {
    fn scoped(&self, scope: &str) -> Box<dyn DynScopedVisitor> {
        (**self).scoped(scope)
    }
}

impl<V: Visitor> Visitor for &V {
    fn visit_message(&self, message: Message<'_>) {
        (**self).visit_message(message)
    }
}

impl<V: Visitor + ?Sized> Visitor for Box<V> {
    fn visit_message(&self, message: Message<'_>) {
        (**self).visit_message(message)
    }
}
