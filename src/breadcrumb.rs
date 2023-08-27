use log::trace;
use std::fmt;

#[derive(Clone)]
pub struct Breadcrumb {
    path: Vec<String>,
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self { path: vec![] }
    }

    pub fn enter(&mut self, path: impl Into<String>) {
        let path = path.into();
        trace!("entering {path}");
        self.path.push(path);
    }

    pub fn exit(&mut self) {
        if let Some(path) = self.path.pop() {
            trace!("exited {path}");
        }
    }
}

impl fmt::Debug for Breadcrumb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Breadcrumb")
            .field("path", &format!("{}", self))
            .finish()
    }
}

impl fmt::Display for Breadcrumb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.path.is_empty() {
                "(root)".to_string()
            } else {
                self.path.join(">")
            }
        )
    }
}
