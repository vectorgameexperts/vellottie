use log::trace;
use std::fmt::{self, Display};

/// An named value type for error propagation and reflection.
#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Null,
    Bool,
    Object,
    Number,
    BoolInt,
    EnumInt,
    String,
    Array,
    Scalar2d,

    Lottie,
    Asset,
    Image,
    Precomposition,
    Layer,
    Shape,
    Transform,
    AnimatedVector,
    StaticVector,
    AnimatedNumber,
    StaticNumber,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValueType::Null => "null",
                ValueType::Bool => "boolean",
                ValueType::Object => "object",
                ValueType::Number => "number",
                ValueType::BoolInt => "0 or 1",
                ValueType::EnumInt => "0 to 255",
                ValueType::String => "string",
                ValueType::Array => "array",
                ValueType::Scalar2d => "[number, number]",
                ValueType::Asset => "Asset",
                ValueType::Layer => "Layer",
                ValueType::Shape => "Shape",
                ValueType::Lottie => "Lottie",
                ValueType::Image => "Image",
                ValueType::Precomposition => "Precomposition",
                ValueType::Transform => "Transform",
                ValueType::AnimatedVector => "AnimatedVector",
                ValueType::StaticVector => "StaticVector",
                ValueType::AnimatedNumber => "AnimatedNumber",
                ValueType::StaticNumber => "StaticNumber",
            }
        )
    }
}

/// A breadcrumb entry
#[derive(Clone, Debug)]
pub enum PathVar {
    Named {
        pos: usize,
        name: String,
        val: ValueType,
        children: usize,
    },
    Unnamed {
        pos: usize,
        val: ValueType,
        children: usize,
    },
}

impl PathVar {
    pub fn get_children(&self) -> usize {
        match self {
            PathVar::Named { children, .. }
            | PathVar::Unnamed { children, .. } => *children,
        }
    }

    pub fn increment_children(&mut self) {
        match self {
            PathVar::Named { children, .. }
            | PathVar::Unnamed { children, .. } => *children += 1,
        }
    }
}

impl Display for PathVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Named { pos, name, .. } => {
                    if *pos > 1 {
                        format!("\"{name}\"#{pos}")
                    } else {
                        format!("\"{name}\"")
                    }
                }
                Self::Unnamed { pos, val, .. } => {
                    if *pos > 1 {
                        format!("(unnamed {val})#{pos}")
                    } else {
                        format!("(unnamed {val})")
                    }
                }
            }
        )
    }
}

/// A data structure used to help unwind when parsing a Lottie Animation.
#[derive(Clone, Debug)]
pub struct Breadcrumb {
    path: Vec<PathVar>,
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self {
            path: vec![PathVar::Unnamed {
                pos: 0,
                val: ValueType::Lottie,
                children: 0,
            }],
        }
    }
}

impl Breadcrumb {
    /// Create a new breadcrumb trail. One is created for every file parse.
    pub fn new() -> Self {
        Self::default()
    }

    /// Rename the root element, if the Animation had a "nm" (name).
    pub fn rename_root(&mut self, name: String) {
        let (pos, val, children) = match self.path[0] {
            PathVar::Named {
                pos, children, val, ..
            }
            | PathVar::Unnamed {
                pos, children, val, ..
            } => (pos, val, children),
        };
        self.path[0] = PathVar::Named {
            name,
            pos,
            val,
            children,
        }
    }

    /// Enter an object field and append it to the breadcrumb trail.
    pub fn enter<S>(&mut self, val: ValueType, name: Option<S>)
    where
        S: Into<String>,
    {
        let parent = self.path.last_mut().expect("no parent in breadcrumb");
        parent.increment_children();

        let path = match name {
            Some(name) => PathVar::Named {
                pos: parent.get_children(),
                name: name.into(),
                val,
                children: 0,
            },
            None => PathVar::Unnamed {
                pos: parent.get_children(),
                val,
                children: 0,
            },
        };
        trace!("entering {path}");
        self.path.push(path);
    }

    /// Enter an unnamed object field and append it to the breadcrumb trail.
    pub fn enter_unnamed(&mut self, val: ValueType) {
        let parent = self.path.last_mut().expect("no parent in breadcrumb");
        parent.increment_children();
        let path = PathVar::Unnamed {
            pos: parent.get_children(),
            val,
            children: 0,
        };
        trace!("entering {path}");
        self.path.push(path);
    }

    /// Exit the current object, and remove it from the breadcrumb trail.
    pub fn exit(&mut self) {
        assert!(self.path.len() > 1, "request to exit with no parent");
        let path = self.path.pop().unwrap();
        trace!("exited {path}");
    }
}

impl fmt::Display for Breadcrumb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            match &self.path[0] {
                PathVar::Named { name, .. } => format!("\"{name}\""),
                PathVar::Unnamed { .. } => "(root)".to_string(),
            },
            if self.path.len() > 1 { ">" } else { "" },
            self.path
                .iter()
                .skip(1)
                .map(PathVar::to_string)
                .collect::<Vec<String>>()
                .join(">")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Breadcrumb;
    use crate::parser::breadcrumb::ValueType;

    #[test]
    fn test_empty() {
        let breadcrumb = Breadcrumb::new();
        assert_eq!("(root)", breadcrumb.to_string());
    }

    #[test]
    fn test_renamed_root() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.rename_root("Example Lottie".to_string());
        assert_eq!("\"Example Lottie\"", breadcrumb.to_string());
    }

    #[test]
    fn test_renamed_one_child() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.enter(ValueType::Shape, Some("Shape1"));
        assert_eq!("(root)>\"Shape1\"", breadcrumb.to_string());
    }

    #[test]
    fn test_renamed_two_children() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.enter(ValueType::Shape, Some("Shape"));
        breadcrumb.exit();
        breadcrumb.enter(ValueType::Shape, Some("Shape"));
        assert_eq!("(root)>\"Shape\"#2", breadcrumb.to_string());
    }

    #[test]
    fn test_renamed_level_two() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.enter(ValueType::Array, Some("Group"));
        breadcrumb.enter(ValueType::Shape, Some("Shape"));
        assert_eq!("(root)>\"Group\">\"Shape\"", breadcrumb.to_string());
    }

    #[test]
    fn test_anon() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.enter_unnamed(ValueType::Layer);
        breadcrumb.exit();
        breadcrumb.enter_unnamed(ValueType::Layer);
        assert_eq!("(root)>(unnamed Layer)#2", breadcrumb.to_string());
    }
}
