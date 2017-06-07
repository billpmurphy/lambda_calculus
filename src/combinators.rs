//! [Standard terms](https://en.wikipedia.org/wiki/Lambda_calculus#Standard_terms) and
//! [combinators](https://en.wikipedia.org/wiki/Combinatory_logic#Combinatory_calculi)
//!
//! * [SKI](https://en.wikipedia.org/wiki/SKI_combinator_calculus)
//! * [Iota](https://en.wikipedia.org/wiki/Iota_and_Jot)
//! * [BCKW](https://en.wikipedia.org/wiki/B,_C,_K,_W_system)
// //! * the recursion combinator U - needs more research
//! * the self-application combinator ω
//! * the divergent combinator Ω
//! * [the fixed-point combinators Y and Z](https://en.wikipedia.org/wiki/Fixed-point_combinator)
//! * the reverse application combinator T

#![allow(non_snake_case)]

use term::{Term, abs, app};
use term::Term::*;

/// I - the identity combinator.
///
/// I := λx.x = λ 1
///
/// # Example
/// ```
/// use lambda_calculus::combinators::I;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(I().app(Var(1)), NOR, 0, false), Var(1));
/// ```
pub fn I() -> Term { abs(Var(1)) }

/// K - the constant / discarding combinator.
///
/// K := λxy.x = λ λ 2 = TRUE
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::K;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(app!(K(), Var(1), Var(2)), NOR, 0, false), Var(1));
/// # }
/// ```
pub fn K() -> Term { abs(abs(Var(2))) }

/// S - the substitution combinator.
///
/// S := λxyz.x z (y z) = λ λ λ 3 1 (2 1)
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::S;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(app!(S(), Var(1), Var(2), Var(3)              ), NOR, 0, false),
///            beta(app!(     Var(1), Var(3), app!(Var(2), Var(3))), NOR, 0, false)
/// );
/// # }
/// ```
pub fn S() -> Term {
    abs(abs(abs(
        app!(Var(3), Var(1), app(Var(2), Var(1)))
    )))
}

/// Iota - the universal combinator.
///
/// ι := λx.x S K = λ 1 S K
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::{iota, I, K, S};
/// use lambda_calculus::*;
///
/// assert_eq!(beta(app!(iota(), iota()), NOR, 0, false), I());
/// assert_eq!(beta(app!(iota(), app!(iota(), app!(iota(), iota()))), NOR, 0, false), K());
/// assert_eq!(beta(app!(iota(), app!(iota(), app!(iota(), app!(iota(), iota())))), NOR, 0, false), S());
/// # }
/// ```
// TODO[issue #28979]: change to ι
pub fn iota() -> Term { abs(app!(Var(1), S(), K())) }

/// B - the composition combinator.
///
/// B := λxyz.x (y z) = λ λ λ 3 (2 1)
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::B;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(app!(B(), Var(1),      Var(2), Var(3) ), NOR, 0, false),
///            beta(app!(     Var(1), app!(Var(2), Var(3))), NOR, 0, false)
/// );
/// # }
/// ```
pub fn B() -> Term {
    abs(abs(abs(
        app(Var(3), app(Var(2), Var(1)))
    )))
}

/// C - the swapping combinator.
///
/// C := λxyz.x z y = λ λ λ 3 1 2
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::C;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(app!(C(), Var(1), Var(2), Var(3)), NOR, 0, false),
///            beta(app!(     Var(1), Var(3), Var(2)), NOR, 0, false)
/// );
/// # }
/// ```
pub fn C() -> Term {
    abs(abs(abs(
        app!(Var(3), Var(1), Var(2))
    )))
}

/// W - the duplicating combinator.
///
/// W := λxy.x y y = λ λ 2 1 1
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::W;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(app!(   W(), Var(1), Var(2)), NOR, 0, false),
///            beta(app!(Var(1), Var(2), Var(2)), NOR, 0, false)
/// );
/// # }
/// ```
pub fn W() -> Term {
    abs(abs(
        app!(Var(2), Var(1), Var(1))
    ))
}
/*
/// U - the recursion combinator.
///
/// U := λxy.y (x x y) = λ λ 1 (2 2 1)
pub fn U() -> Term { abs(abs(Var(1).app(Var(2).app(Var(2)).app(Var(1))))) }
*/
/// ω - the self-application combinator.
///
/// ω := λx.x x = λ 1 1
///
/// # Example
/// ```
/// use lambda_calculus::combinators::om;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(  om().app(Var(1)), NOR, 0, false),
///            beta(Var(1).app(Var(1)), NOR, 0, false)
/// );
/// ```
// TODO[issue #28979]: change to ω
pub fn om() -> Term { abs(Var(1).app(Var(1))) }

/// Ω - the divergent combinator.
///
/// Ω := ω ω
///
/// # Example
/// ```
/// use lambda_calculus::combinators::OM;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(OM(), NOR, 3, false), OM());
/// ```
// TODO[issue #28979]: change to Ω
pub fn OM() -> Term { om().app(om()) }

/// Y - the lazy fixed-point combinator.
///
/// It is suitable for `NOR` (normal), `HNO` (hybrid normal), `CBN` (call-by-name) and `HSP`
/// (head spine) reduction `Order`s.
///
/// Y := λf.(λx.f (x x)) (λx.f (x x)) = λ (λ 2 (1 1)) (λ 2 (1 1))
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::Y;
/// use lambda_calculus::*;
///
/// fn dummy() -> Term { abs(Var(2)) } // a dummy term that won't easily reduce
///
/// assert_eq!(beta(              app!(Y(), dummy() ), NOR, 0, false),
///            beta(app!(dummy(), app!(Y(), dummy())), NOR, 0, false)
/// );
/// # }
/// ```
pub fn Y() -> Term {
    abs(app(
        abs(app(Var(2), app(Var(1), Var(1)))),
        abs(app(Var(2), app(Var(1), Var(1))))
    ))
}

/// Z - the strict fixed-point combinator.
///
/// It will work with all the reduction orders suitable for its lazy counterpart (the `Y`
/// combinator). In addition, it will also work with `CBV` (call-by-value) and `HAP` (hybrid
/// applicative) reduction `Order`s, but with them it's not a drop-in replacement for the `Y`
/// combinator - in order for such expressions to work, they need to be modified so that the
/// evaluation of arguments of conditionals and other terms that need to be lazy is delayed.
///
/// Z := λf.(λx.f (λv.x x v)) (λx.f (λv.x x v)) = λ (λ 2 (λ 2 2 1)) (λ 2 (λ 2 2 1))
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::Z;
/// use lambda_calculus::*;
///
/// fn dummy() -> Term { abs(Var(2)) } // a dummy term that won't easily reduce
///
/// assert_eq!(beta(              app!(Z(), dummy() ), CBV, 0, false),
///            beta(app!(dummy(), app!(Z(), dummy())), CBV, 0, false)
/// );
/// # }
/// ```
pub fn Z() -> Term {
    abs(app(
        abs(app(Var(2), abs(app!(Var(2), Var(2), Var(1))))),
        abs(app(Var(2), abs(app!(Var(2), Var(2), Var(1)))))
    ))
}

/// T - the reverse application combinator.
///
/// T := λxf. f x = λ λ 1 2
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::combinators::T;
/// use lambda_calculus::*;
///
/// assert_eq!(beta(app!(T(), Var(1), Var(2)), NOR, 0, false),
///                 app!(     Var(2), Var(1))
/// );
/// # }
/// ```
pub fn T() -> Term {
    abs(abs(
        app(Var(1), Var(2))
    ))
}
