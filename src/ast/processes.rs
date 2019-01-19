// Reference http://lucacardelli.name/Papers/MobileAmbients.A4.pdf page 20

use crate::ast::capabilities::*;

//
// Processes
//

/*
P,Q ::=
    (νn)P           -- restriction
    0               -- inactivity
    P|Q             -- composition
    !P              -- replication
    M[P]            -- ambient
    M.P             -- capability action
    (x).P           -- input action
    <M>             -- async output action
*/

pub trait Process {}

#[derive(Debug)]
pub struct Restriction<S, P>
where
    S: Into<String>,
    P: Process,
{
    pub name: S,
    pub process: P,
}

#[derive(Debug)]
pub struct Inactivity;

#[derive(Debug)]
pub struct Composition<P, Q>
where
    P: Process,
    Q: Process,
{
    pub process_l: P,
    pub process_r: Q,
}

#[derive(Debug)]
pub struct Replication<P>
where
    P: Process,
{
    pub process: P,
}

#[derive(Debug)]
pub struct Ambient<M, P>
where
    M: Capability,
    P: Process,
{
    pub capability: M,
    pub process: P,
}

#[derive(Debug)]
pub struct Action<M, P>
where
    M: Capability,
    P: Process,
{
    pub capability: M,
    pub process: P,
}

#[derive(Debug)]
pub struct Input<X, P>
where
    X: Into<String>,
    P: Process,
{
    pub variable: X,
    pub process: P,
}

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

// EOF
