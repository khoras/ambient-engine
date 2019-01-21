//! This is the documentation to capabilities module

/// The `Capability` type. See [Cardellis' Paper](http://lucacardelli.name/Papers/MobileAmbients.A4.pdf) page 20.
///
/// ```text
/// M ::=              -- Capability
///   x                -- variable
///    n               -- name
///    in M            -- enter
///    out M           -- exit
///    open M          -- open
///    ε               -- empty
///    M.M′            -- path
/// ```
///
pub trait Capability {}

/// `Variable` capability for dynamic purposes
#[derive(Debug)]
pub struct Variable<S>
where
    S: Into<String>,
{
    pub value: S,
}

/// `Name` for action target or ambient namespace definition
#[derive(Debug)]
pub struct Name<S>
where
    S: Into<String>,
{
    pub value: S,
}

/// `In` enter a sibling named ambient
#[derive(Debug)]
pub struct In<M: Capability> {
    pub capability: M,
}

/// `Out` exit a parent named ambient
#[derive(Debug)]
pub struct Out<M>
where
    M: Capability,
{
    pub capability: M,
}

/// `Open` dissolve a sibling named abient
#[derive(Debug)]
pub struct Open<M>
where
    M: Capability,
{
    pub capability: M,
}

/// `Path` composition of two sequential capabilities. The left one should be done before.
#[derive(Debug)]
pub struct Path<N, M>
where
    N: Capability,
    M: Capability,
{
    pub capability_l: N,
    pub capability_r: M,
}

/// `Empty` capability meaning nothing to be done
pub struct Empty;

// Kind of Capability

impl<S> Capability for Variable<S> where S: Into<String> {}

impl<S> Capability for Name<S> where S: Into<String> {}

impl<M> Capability for In<M> where M: Capability {}

impl<M> Capability for Out<M> where M: Capability {}

impl<M> Capability for Open<M> where M: Capability {}

impl<N, M> Capability for Path<N, M>
where
    N: Capability,
    M: Capability,
{
}

impl Capability for Empty {}

// Factories

impl<S> Variable<S>
where
    S: Into<String>,
{
    fn new(value: S) -> Self {
        Self { value }
    }
}

impl<S> Name<S>
where
    S: Into<String>,
{
    fn new(value: S) -> Self {
        Self { value }
    }
}

impl<M> In<M>
where
    M: Capability,
{
    fn new(capability: M) -> Self {
        Self { capability }
    }
}

impl<M> Out<M>
where
    M: Capability,
{
    fn new(capability: M) -> Self {
        Self { capability }
    }
}

impl<M> Open<M>
where
    M: Capability,
{
    fn new(capability: M) -> Self {
        Self { capability }
    }
}

impl<N, M> Path<N, M>
where
    N: Capability,
    M: Capability,
{
    fn new(capability_l: N, capability_r: M) -> Self {
        Self {
            capability_l,
            capability_r,
        }
    }
}

impl Empty {
    fn new() -> Self {
        Self
    }
}
