//! Type representations

//! Types the represent types in letpl.

use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum TypeTag {
    Int,
}

/// Represents a procedure type.
#[derive(Debug)]
pub struct TypeProc {
    /// The procedure's return type.
    t_return: Option<Type>,
}

#[derive(Debug)]
pub struct Type {
    tag: Rc<TypeTag>,
}

impl TypeProc {
    /// Creates a procedure type.
    pub fn new(t_return: Option<Type>) -> Self {
        Self { t_return }
    }
}

impl Type {
    pub fn new_int() -> Self {
        let tag = Rc::new(TypeTag::Int);
        Self { tag }
    }

    pub fn tag(&self) -> &TypeTag {
        self.tag.as_ref()
    }
}

impl Clone for Type {
    fn clone(&self) -> Self {
        let tag = Rc::clone(&self.tag);
        Self { tag }
    }
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "int"),
        }
    }
}

impl fmt::Display for TypeProc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "procedure;")?;
        if let Some(t_return) = &self.t_return {
            write!(f, " {t_return}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}

impl PartialEq for TypeTag {
    fn eq(&self, other: &Self) -> bool {
        use TypeTag::*;

        match (self, other) {
            (Int, Int) => true,
        }
    }
}

impl PartialEq for TypeProc {
    fn eq(&self, other: &Self) -> bool {
        self.t_return.eq(&other.t_return)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.tag.as_ref().eq(other.tag.as_ref())
    }
}
