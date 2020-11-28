#[derive(Debug)]
/// A JS Keyword
///
/// # Standard
/// await
/// break
/// case
/// catch
/// class
/// const
/// continue
/// debugger
/// default
/// delete (10)
/// do
/// else
/// export
/// extends
/// finally
/// for
/// function
/// if
/// import
/// in (20)
/// instanceof
/// new
/// return
/// super
/// switch
/// this
/// throw
/// try
/// typeof
/// var (30)
/// void
/// while
/// with
/// yield
/// # Future Reserved
/// enum
/// # Strict Mode Future Reserved
/// implements
/// package
/// protected
/// interface
/// private (40)
/// public
pub enum Keyword<T> {
    Await(T),
    Break(T),
    Case(T),
    Catch(T),
    Class(T),
    Const(T),
    Continue(T),
    Debugger(T),
    Default(T),
    Delete(T),
    Do(T),
    Else(T),
    Enum(T),
    Export(T),
    Extends(T),
    Finally(T),
    For(T),
    Function(T),
    If(T),
    Implements(T),
    Import(T),
    In(T),
    InstanceOf(T),
    Interface(T),
    Let(T),
    New(T),
    Package(T),
    Private(T),
    Protected(T),
    Public(T),
    Return(T),
    Static(T),
    Super(T),
    Switch(T),
    This(T),
    Throw(T),
    Try(T),
    TypeOf(T),
    Var(T),
    Void(T),
    While(T),
    With(T),
    Yield(T),
}

impl<T> Clone for Keyword<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Await(i) => Self::Await(i.clone()),
            Self::Break(i) => Self::Break(i.clone()),
            Self::Case(i) => Self::Case(i.clone()),
            Self::Catch(i) => Self::Catch(i.clone()),
            Self::Class(i) => Self::Class(i.clone()),
            Self::Const(i) => Self::Const(i.clone()),
            Self::Continue(i) => Self::Continue(i.clone()),
            Self::Debugger(i) => Self::Debugger(i.clone()),
            Self::Default(i) => Self::Default(i.clone()),
            Self::Delete(i) => Self::Delete(i.clone()),
            Self::Do(i) => Self::Do(i.clone()),
            Self::Else(i) => Self::Else(i.clone()),
            Self::Enum(i) => Self::Enum(i.clone()),
            Self::Export(i) => Self::Export(i.clone()),
            Self::Extends(i) => Self::Extends(i.clone()),
            Self::Finally(i) => Self::Finally(i.clone()),
            Self::For(i) => Self::For(i.clone()),
            Self::Function(i) => Self::Function(i.clone()),
            Self::If(i) => Self::If(i.clone()),
            Self::Implements(i) => Self::Implements(i.clone()),
            Self::Import(i) => Self::Import(i.clone()),
            Self::In(i) => Self::In(i.clone()),
            Self::InstanceOf(i) => Self::InstanceOf(i.clone()),
            Self::Interface(i) => Self::Interface(i.clone()),
            Self::Let(i) => Self::Let(i.clone()),
            Self::New(i) => Self::New(i.clone()),
            Self::Package(i) => Self::Package(i.clone()),
            Self::Private(i) => Self::Private(i.clone()),
            Self::Protected(i) => Self::Protected(i.clone()),
            Self::Public(i) => Self::Public(i.clone()),
            Self::Return(i) => Self::Return(i.clone()),
            Self::Static(i) => Self::Static(i.clone()),
            Self::Super(i) => Self::Super(i.clone()),
            Self::Switch(i) => Self::Switch(i.clone()),
            Self::This(i) => Self::This(i.clone()),
            Self::Throw(i) => Self::Throw(i.clone()),
            Self::Try(i) => Self::Try(i.clone()),
            Self::TypeOf(i) => Self::TypeOf(i.clone()),
            Self::Var(i) => Self::Var(i.clone()),
            Self::Void(i) => Self::Void(i.clone()),
            Self::While(i) => Self::While(i.clone()),
            Self::With(i) => Self::With(i.clone()),
            Self::Yield(i) => Self::Yield(i.clone()),
        }
    }
}

impl<T, U> PartialEq<Keyword<T>> for Keyword<U> {
    fn eq(&self, other: &Keyword<T>) -> bool {
        use Keyword::*;
        matches!((self, other), (Await(_), Await(_))
            | (Break(_), Break(_))
            | (Case(_), Case(_))
            | (Catch(_), Catch(_))
            | (Class(_), Class(_))
            | (Const(_), Const(_))
            | (Continue(_), Continue(_))
            | (Debugger(_), Debugger(_))
            | (Default(_), Default(_))
            | (Delete(_), Delete(_))
            | (Do(_), Do(_))
            | (Else(_), Else(_))
            | (Enum(_), Enum(_))
            | (Export(_), Export(_))
            | (Extends(_), Extends(_))
            | (Finally(_), Finally(_))
            | (For(_), For(_))
            | (Function(_), Function(_))
            | (If(_), If(_))
            | (Implements(_), Implements(_))
            | (Import(_), Import(_))
            | (In(_), In(_))
            | (InstanceOf(_), InstanceOf(_))
            | (Interface(_), Interface(_))
            | (Let(_), Let(_))
            | (New(_), New(_))
            | (Package(_), Package(_))
            | (Private(_), Private(_))
            | (Protected(_), Protected(_))
            | (Public(_), Public(_))
            | (Return(_), Return(_))
            | (Static(_), Static(_))
            | (Super(_), Super(_))
            | (Switch(_), Switch(_))
            | (This(_), This(_))
            | (Throw(_), Throw(_))
            | (Try(_), Try(_))
            | (TypeOf(_), TypeOf(_))
            | (Var(_), Var(_))
            | (Void(_), Void(_))
            | (While(_), While(_))
            | (With(_), With(_))
            | (Yield(_), Yield(_)))
    }
}

impl Keyword<()> {
    pub fn with_str(self, s: &str) -> Keyword<&str> {
        match self {
            Keyword::Await(_) => Keyword::Await(s),
            Keyword::Break(_) => Keyword::Break(s),
            Keyword::Case(_) => Keyword::Case(s),
            Keyword::Catch(_) => Keyword::Catch(s),
            Keyword::Class(_) => Keyword::Class(s),
            Keyword::Const(_) => Keyword::Const(s),
            Keyword::Continue(_) => Keyword::Continue(s),
            Keyword::Debugger(_) => Keyword::Debugger(s),
            Keyword::Default(_) => Keyword::Default(s),
            Keyword::Delete(_) => Keyword::Delete(s),
            Keyword::Do(_) => Keyword::Do(s),
            Keyword::Else(_) => Keyword::Else(s),
            Keyword::Enum(_) => Keyword::Enum(s),
            Keyword::Export(_) => Keyword::Export(s),
            Keyword::Extends(_) => Keyword::Extends(s),
            Keyword::Finally(_) => Keyword::Finally(s),
            Keyword::For(_) => Keyword::For(s),
            Keyword::Function(_) => Keyword::Function(s),
            Keyword::If(_) => Keyword::If(s),
            Keyword::Implements(_) => Keyword::Implements(s),
            Keyword::Import(_) => Keyword::Import(s),
            Keyword::In(_) => Keyword::In(s),
            Keyword::InstanceOf(_) => Keyword::InstanceOf(s),
            Keyword::Interface(_) => Keyword::Interface(s),
            Keyword::Let(_) => Keyword::Let(s),
            Keyword::New(_) => Keyword::New(s),
            Keyword::Package(_) => Keyword::Package(s),
            Keyword::Private(_) => Keyword::Private(s),
            Keyword::Protected(_) => Keyword::Protected(s),
            Keyword::Public(_) => Keyword::Public(s),
            Keyword::Return(_) => Keyword::Return(s),
            Keyword::Static(_) => Keyword::Static(s),
            Keyword::Super(_) => Keyword::Super(s),
            Keyword::Switch(_) => Keyword::Switch(s),
            Keyword::This(_) => Keyword::This(s),
            Keyword::Throw(_) => Keyword::Throw(s),
            Keyword::Try(_) => Keyword::Try(s),
            Keyword::TypeOf(_) => Keyword::TypeOf(s),
            Keyword::Var(_) => Keyword::Var(s),
            Keyword::Void(_) => Keyword::Void(s),
            Keyword::While(_) => Keyword::While(s),
            Keyword::With(_) => Keyword::With(s),
            Keyword::Yield(_) => Keyword::Yield(s),
        }
    }
}

impl<T> ToString for Keyword<T> {
    /// Convert a keyword into a string
    fn to_string(&self) -> String {
        self.as_str().into()
    }
}

impl<T> PartialEq<str> for Keyword<T> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<T> Keyword<T> {
    /// Is this keyword one of the future reserved words
    ///
    /// - enum
    /// - export
    /// - implements
    /// - super
    pub fn is_future_reserved(&self) -> bool {
        match self {
            Keyword::Enum(_) => true,
            Keyword::Export(_) => true,
            Keyword::Implements(_) => true,
            Keyword::Super(_) => true,
            _ => false,
        }
    }
    /// Is this keyword a reserved word when the context
    /// has a 'use strict' directive.
    ///
    /// ## Keywords
    /// - implements
    /// - interface
    /// - package
    /// - private
    /// - protected
    /// - public
    /// - static
    /// - yield
    /// - let
    pub fn is_strict_reserved(&self) -> bool {
        match self {
            Keyword::Implements(_) => true,
            Keyword::Interface(_) => true,
            Keyword::Package(_) => true,
            Keyword::Private(_) => true,
            Keyword::Protected(_) => true,
            Keyword::Public(_) => true,
            Keyword::Static(_) => true,
            Keyword::Yield(_) => true,
            Keyword::Let(_) => true,
            _ => false,
        }
    }
    /// Is this keyword a reserved word
    ///
    /// ## Keywords
    /// - break
    /// - case
    /// - catch
    /// - continue
    /// - debugger
    /// - default
    /// - delete
    /// - do
    /// - else
    /// - for
    /// - function
    /// - if
    /// - instanceof
    /// - in
    /// - new
    /// - return
    /// - switch
    /// - this
    /// - throw
    /// - try
    /// - typeof
    /// - var
    /// - void
    /// - while
    /// - with
    pub fn is_reserved(&self) -> bool {
        matches!(self, Keyword::Break(_)
            | Keyword::Case(_)
            | Keyword::Catch(_)
            | Keyword::Class(_)
            | Keyword::Continue(_)
            | Keyword::Debugger(_)
            | Keyword::Default(_)
            | Keyword::Delete(_)
            | Keyword::Do(_)
            | Keyword::Else(_)
            | Keyword::Export(_)
            | Keyword::Extends(_)
            | Keyword::Finally(_)
            | Keyword::For(_)
            | Keyword::Function(_)
            | Keyword::If(_)
            | Keyword::Import(_)
            | Keyword::In(_)
            | Keyword::InstanceOf(_)
            | Keyword::New(_)
            | Keyword::Return(_)
            | Keyword::Switch(_)
            | Keyword::Super(_)
            | Keyword::This(_)
            | Keyword::Throw(_)
            | Keyword::Try(_)
            | Keyword::TypeOf(_)
            | Keyword::Var(_)
            | Keyword::Void(_)
            | Keyword::While(_)
            | Keyword::With(_))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Keyword::Await(_) => "await",
            Keyword::Break(_) => "break",
            Keyword::Case(_) => "case",
            Keyword::Catch(_) => "catch",
            Keyword::Class(_) => "class",
            Keyword::Const(_) => "const",
            Keyword::Continue(_) => "continue",
            Keyword::Debugger(_) => "debugger",
            Keyword::Default(_) => "default",
            Keyword::Import(_) => "import",
            Keyword::Delete(_) => "delete",
            Keyword::Do(_) => "do",
            Keyword::Else(_) => "else",
            Keyword::Enum(_) => "enum",
            Keyword::Export(_) => "export",
            Keyword::Extends(_) => "extends",
            Keyword::Finally(_) => "finally",
            Keyword::For(_) => "for",
            Keyword::Function(_) => "function",
            Keyword::If(_) => "if",
            Keyword::In(_) => "in",
            Keyword::Implements(_) => "implements",
            Keyword::InstanceOf(_) => "instanceof",
            Keyword::Interface(_) => "interface",
            Keyword::Let(_) => "let",
            Keyword::New(_) => "new",
            Keyword::Package(_) => "package",
            Keyword::Private(_) => "private",
            Keyword::Protected(_) => "protected",
            Keyword::Public(_) => "public",
            Keyword::Static(_) => "static",
            Keyword::Return(_) => "return",
            Keyword::Super(_) => "super",
            Keyword::Switch(_) => "switch",
            Keyword::This(_) => "this",
            Keyword::Throw(_) => "throw",
            Keyword::Try(_) => "try",
            Keyword::TypeOf(_) => "typeof",
            Keyword::Var(_) => "var",
            Keyword::Void(_) => "void",
            Keyword::While(_) => "while",
            Keyword::With(_) => "with",
            Keyword::Yield(_) => "yield",
        }
    }

    pub fn to_empty(&self) -> Keyword<()> {
        match self {
            Keyword::Await(_) => Keyword::Await(()),
            Keyword::Break(_) => Keyword::Break(()),
            Keyword::Case(_) => Keyword::Case(()),
            Keyword::Catch(_) => Keyword::Catch(()),
            Keyword::Class(_) => Keyword::Class(()),
            Keyword::Const(_) => Keyword::Const(()),
            Keyword::Continue(_) => Keyword::Continue(()),
            Keyword::Debugger(_) => Keyword::Debugger(()),
            Keyword::Default(_) => Keyword::Default(()),
            Keyword::Import(_) => Keyword::Import(()),
            Keyword::Delete(_) => Keyword::Delete(()),
            Keyword::Do(_) => Keyword::Do(()),
            Keyword::Else(_) => Keyword::Else(()),
            Keyword::Enum(_) => Keyword::Enum(()),
            Keyword::Export(_) => Keyword::Export(()),
            Keyword::Extends(_) => Keyword::Extends(()),
            Keyword::Finally(_) => Keyword::Finally(()),
            Keyword::For(_) => Keyword::For(()),
            Keyword::Function(_) => Keyword::Function(()),
            Keyword::If(_) => Keyword::If(()),
            Keyword::In(_) => Keyword::In(()),
            Keyword::Implements(_) => Keyword::Implements(()),
            Keyword::InstanceOf(_) => Keyword::InstanceOf(()),
            Keyword::Interface(_) => Keyword::Interface(()),
            Keyword::Let(_) => Keyword::Let(()),
            Keyword::New(_) => Keyword::New(()),
            Keyword::Package(_) => Keyword::Package(()),
            Keyword::Private(_) => Keyword::Private(()),
            Keyword::Protected(_) => Keyword::Protected(()),
            Keyword::Public(_) => Keyword::Public(()),
            Keyword::Static(_) => Keyword::Static(()),
            Keyword::Return(_) => Keyword::Return(()),
            Keyword::Super(_) => Keyword::Super(()),
            Keyword::Switch(_) => Keyword::Switch(()),
            Keyword::This(_) => Keyword::This(()),
            Keyword::Throw(_) => Keyword::Throw(()),
            Keyword::Try(_) => Keyword::Try(()),
            Keyword::TypeOf(_) => Keyword::TypeOf(()),
            Keyword::Var(_) => Keyword::Var(()),
            Keyword::Void(_) => Keyword::Void(()),
            Keyword::While(_) => Keyword::While(()),
            Keyword::With(_) => Keyword::With(()),
            Keyword::Yield(_) => Keyword::Yield(()),
        }
    }
}

impl<'a> Keyword<&'a str> {
    pub fn has_unicode_escape(&self) -> bool {
        match self {
            Keyword::Await(s) => s,
            Keyword::Break(s) => s,
            Keyword::Case(s) => s,
            Keyword::Catch(s) => s,
            Keyword::Class(s) => s,
            Keyword::Const(s) => s,
            Keyword::Continue(s) => s,
            Keyword::Debugger(s) => s,
            Keyword::Default(s) => s,
            Keyword::Import(s) => s,
            Keyword::Delete(s) => s,
            Keyword::Do(s) => s,
            Keyword::Else(s) => s,
            Keyword::Enum(s) => s,
            Keyword::Export(s) => s,
            Keyword::Extends(s) => s,
            Keyword::Finally(s) => s,
            Keyword::For(s) => s,
            Keyword::Function(s) => s,
            Keyword::If(s) => s,
            Keyword::In(s) => s,
            Keyword::Implements(s) => s,
            Keyword::InstanceOf(s) => s,
            Keyword::Interface(s) => s,
            Keyword::Let(s) => s,
            Keyword::New(s) => s,
            Keyword::Package(s) => s,
            Keyword::Private(s) => s,
            Keyword::Protected(s) => s,
            Keyword::Public(s) => s,
            Keyword::Static(s) => s,
            Keyword::Return(s) => s,
            Keyword::Super(s) => s,
            Keyword::Switch(s) => s,
            Keyword::This(s) => s,
            Keyword::Throw(s) => s,
            Keyword::Try(s) => s,
            Keyword::TypeOf(s) => s,
            Keyword::Var(s) => s,
            Keyword::Void(s) => s,
            Keyword::While(s) => s,
            Keyword::With(s) => s,
            Keyword::Yield(s) => s,
        }
        .contains("\\u")
    }
}
