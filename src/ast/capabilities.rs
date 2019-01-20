// Reference http://lucacardelli.name/Papers/MobileAmbients.A4.pdf page 20

//
// Capabilities
//

/*
M ::=               -- Capability
    x               -- variable
    n               -- name
    in M            -- enter
    out M           -- exit
    open M          -- open
    ε               -- empty
    M.M′            -- path
*/

pub trait Capability {}

#[derive(Debug)]
pub struct Variable<S>
where
    S: Into<String>,
{
    pub value: S,
}

#[derive(Debug)]
pub struct Name<S>
where
    S: Into<String>,
{
    pub value: S,
}

#[derive(Debug)]
pub struct In<M: Capability> {
    pub capability: M,
}

#[derive(Debug)]
pub struct Out<M>
where
    M: Capability,
{
    pub capability: M,
}

#[derive(Debug)]
pub struct Open<M>
where
    M: Capability,
{
    pub capability: M,
}

#[derive(Debug)]
pub struct Path<N, M>
where
    N: Capability,
    M: Capability,
{
    pub capability_l: N,
    pub capability_r: M,
}

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
