mod capability {
    /*
    M ::=               -- Capability
        in n            -- enter
        out n           -- exit
        open n          -- open
    */

    trait Name: Into<String> {}

    #[derive(Debug)]
    struct In<M: Name> {
        m: M,
    }

    #[derive(Debug)]
    struct Out<M: Name> {
        m: M,
    }

    #[derive(Debug)]
    struct Open<M: Name> {
        m: M,
    }

    struct Empty;

    // Capability trait implementation

    pub trait Capability {}

    impl<M: Name> Capability for In<M> {}

    impl<M: Name> Capability for Out<M> {}

    impl<M: Name> Capability for Open<M> {}


}

mod process {



}