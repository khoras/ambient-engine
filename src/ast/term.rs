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

#[derive(Debug)]
struct Variable<S> where S: Into<String> {
    value: S
}

#[derive(Debug)]
struct Name<S> where S: Into<String> {
    value: S
}

#[derive(Debug)]
struct Enter<M: Capability> {
    capability: M
}

#[derive(Debug)]
struct Exit<M> where M: Capability {
    capability: M
}

#[derive(Debug)]
struct Open<M> where M: Capability {
    capability: M
}

#[derive(Debug)]
struct Path<N, M>
    where N: Capability, M: Capability {
    capability_l: N,
    capability_r: M,
}

struct Empty;

// Kind of Capability

pub trait Capability {}

impl<S> Capability for Variable<S> where S: Into<String> {}

impl<S> Capability for Name<S> where S: Into<String> {}

impl<M> Capability for Enter<M> where M: Capability {}

impl<M> Capability for Exit<M> where M: Capability {}

impl<M> Capability for Open<M> where M: Capability {}

impl<N, M> Capability for Path<N, M> where N: Capability, M: Capability {}

impl Capability for Empty {}
