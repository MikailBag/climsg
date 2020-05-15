use std::sync::Arc;

/// Visitor that renders messages with features like folding
pub struct TextVisitor {
    cfg: Arc<Configuration>,
    path: ScopePath,
}

impl TextVisitor {
    pub fn new() -> TextVisitor {
        TextVisitor {
            cfg: Arc::new(Configuration::new()),
            path: ScopePath::new(),
        }
    }

    /// When scope with this path will be finished, all its contents
    /// will be hidden
    pub fn collapse_on_finish(&mut self, path: &[impl AsRef<str>]) {
        self.cfg.edit(|cfg| {
            cfg.collapse_on_finish.push(ScopePath::from_slice(path));
        })
    }

    /// Inner data of this scope will never be displayed.
    /// Root still will be rendered
    pub fn collapse_on_start(&mut self, path: &[impl AsRef<str>]) {
        self.cfg.edit(|cfg| {
            cfg.collapse_on_start.push(ScopePath::from_slice(path));
        })
    }
}

impl crate::Visitor for TextVisitor {
    fn visit_message(&self, message: crate::Message<'_>) {
        println!("{}", message.text);
    }
}

impl crate::ScopedVisitor for TextVisitor {
    fn scoped(&self, scope: &str,) -> Self {
        let mut new_path = self.path.clone();
        new_path.items.push(scope.to_string());
        TextVisitor {
            path: new_path,
            cfg: self.cfg.clone(),
        }
    }
}

#[derive(Clone)]
struct ScopePath {
    items: Vec<String>,
}

impl ScopePath {
    fn new() -> ScopePath {
        ScopePath { items: Vec::new() }
    }

    fn from_slice(slice: &[impl AsRef<str>]) -> ScopePath {
        ScopePath {
            items: slice
                .iter()
                .map(AsRef::as_ref)
                .map(ToString::to_string)
                .collect(),
        }
    }
}

#[derive(Clone)]
struct Configuration {
    collapse_on_finish: Vec<ScopePath>,
    collapse_on_start: Vec<ScopePath>,
}

impl Configuration {
    fn new() -> Configuration {
        Configuration {
            collapse_on_finish: Vec::new(),
            collapse_on_start: Vec::new()
        }
    }

    fn edit(self: &mut Arc<Self>, f: impl FnOnce(&mut Configuration)) {
        f(Arc::make_mut(self))
    }
}
