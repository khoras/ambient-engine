# Ambient AST

This crate provides intermediate Abstract Syntax Tree (AST) for the compilation process.
These definitions came from the 
[Cardellis' Paper](http://lucacardelli.name/Papers/MobileAmbients.A4.pdf).

## Capabilities

```
M ::=              -- Capability
   x                -- variable
    n               -- name
    in M            -- enter
    out M           -- exit
    open M          -- open
    ε               -- empty
    M.M′            -- path
```

## Processes 

```
P,Q ::=
    (νn)P           -- restriction
    0               -- inactivity
    P|Q             -- composition
    !P              -- replication
    M[P]            -- ambient
    M.P             -- capability action
    (x).P           -- input action
    <M>             -- async output action
```
