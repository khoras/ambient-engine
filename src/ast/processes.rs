//! This is the documentation to processes module

use crate::ast::capabilities::*;

/// The `Process` type. See [Cardellis' Paper](http://lucacardelli.name/Papers/MobileAmbients.A4.pdf) page 20.
///
/// ```text
/// P,Q ::=
///    (νn)P           -- restriction
///    0               -- inactivity
///    P|Q             -- composition
///    !P              -- replication
///    M[P]            -- ambient
///    M.P             -- capability action
///    (x).P           -- input action
///    <M>             -- async output action
///```
///
pub trait Process {}

/// `Restriction` defines a process with a private name
#[derive(Debug)]
pub struct Restriction<S, P>
where
    S: Into<String>,
    P: Process,
{
    pub name: S,
    pub process: P,
}

/// `Inactivity` defines a process doing nothing
#[derive(Debug)]
pub struct Inactivity;

/// `Composition` defines a two processes running in parallel
#[derive(Debug)]
pub struct Composition<P, Q>
where
    P: Process,
    Q: Process,
{
    pub process_l: P,
    pub process_r: Q,
}

/// `Replication` defines a perpetual process where `!P` is isomorphic to `P | !P`
#[derive(Debug)]
pub struct Replication<P>
where
    P: Process,
{
    pub process: P,
}

/// `Ambient` defines a named bounded space containing processes
#[derive(Debug)]
pub struct Ambient<M, P>
where
    M: Capability,
    P: Process,
{
    pub capability: M,
    pub process: P,
}

/// `Action` attach a capability to a given process defining an action to be done
#[derive(Debug)]
pub struct Action<M, P>
where
    M: Capability,
    P: Process,
{
    pub capability: M,
    pub process: P,
}

/// `Input` defines the function able to capture a message.
#[derive(Debug)]
pub struct Input<X, P>
where
    X: Into<String>,
    P: Process,
{
    pub variable: X,
    pub process: P,
}

/// `Output` defines the message able to be captured by an `Input`
#[derive(Debug)]
pub struct Output<M>
where
    M: Capability,
{
    pub message: M,
}

// Kind of process

impl<N, P> Process for Restriction<N, P>
where
    N: Into<String>,
    P: Process,
{
}

impl Process for Inactivity {}

impl<P, Q> Process for Composition<P, Q>
where
    P: Process,
    Q: Process,
{
}

impl<P> Process for Replication<P> where P: Process {}

impl<M, P> Process for Ambient<M, P>
where
    M: Capability,
    P: Process,
{
}

impl<M, P> Process for Action<M, P>
where
    M: Capability,
    P: Process,
{
}

impl<X, P> Process for Input<X, P>
where
    X: Into<String>,
    P: Process,
{
}

impl<M> Process for Output<M> where M: Capability {}

// Factories

impl<S, P> Restriction<S, P>
where
    S: Into<String>,
    P: Process,
{
    fn new(name: S, process: P) -> Self {
        Self { name, process }
    }
}

impl Inactivity {
    fn new() -> Self {
        Inactivity
    }
}

impl<P, Q> Composition<P, Q>
where
    P: Process,
    Q: Process,
{
    fn new(process_l: P, process_r: Q) -> Self {
        Self {
            process_l,
            process_r,
        }
    }
}

impl<P> Replication<P>
where
    P: Process,
{
    fn new(process: P) -> Self {
        Self { process }
    }
}

impl<M, P> Ambient<M, P>
where
    M: Capability,
    P: Process,
{
    fn new(capability: M, process: P) -> Self {
        Self {
            capability,
            process,
        }
    }
}

impl<M, P> Action<M, P>
where
    M: Capability,
    P: Process,
{
    fn new(capability: M, process: P) -> Self {
        Self {
            capability,
            process,
        }
    }
}

impl<X, P> Input<X, P>
where
    X: Into<String>,
    P: Process,
{
    fn new(variable: X, process: P) -> Self {
        Self { variable, process }
    }
}

impl<M> Output<M>
where
    M: Capability,
{
    fn new(message: M) -> Self {
        Self { message }
    }
}
