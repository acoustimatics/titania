//! Type representations

//! Types the represent types in letpl.

use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
enum TypeTag {
    Int,
    Proc(TypeProc),
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

impl TypeTag {
    pub fn is_int(&self) -> bool {
        match self {
            TypeTag::Int => true,
            _ => false,
        }
    }

    pub fn as_proc(&self) -> Option<&TypeProc> {
        match self {
            TypeTag::Proc(t_proc) => Some(t_proc),
            _ => None,
        }
    }
}

impl TypeProc {
    pub fn new(t_return: Option<Type>) -> Self {
        Self { t_return }
    }
}

impl Type {
    pub fn new_int() -> Self {
        let tag = Rc::new(TypeTag::Int);
        Self { tag }
    }

    pub fn new_proc(t_return: Option<Type>) -> Self {
        let t_proc = TypeProc { t_return };
        let tag = TypeTag::Proc(t_proc);
        let tag = Rc::new(tag);
        Self { tag }
    }

    pub fn is_int(&self) -> bool {
        self.tag.is_int()
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
            TypeTag::Proc(proc) => write!(f, "{proc}"),
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
            (Proc(self_proc), Proc(other_proc)) => self_proc.eq(other_proc),
            _ => false,
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
