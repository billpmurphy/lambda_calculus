//! [Church pairs](https://en.wikipedia.org/wiki/Church_encoding#Church_pairs)

use term::{Term, abs};
use term::Term::*;
use church::booleans::{tru, fls};
use church::ChurchError;
use church::ChurchError::*;

/// Produces a Church-encoded pair; applying it to two other terms puts them inside it.
///
/// PAIR := λxyz.z x y = λ λ λ 1 3 2
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::church::pairs::pair;
/// use lambda_calculus::church::numerals::{zero, one};
///
/// let pair01 = app!(pair(), zero(), one());
///
/// assert_eq!(pair01.fst_ref(), Ok(&zero()));
/// assert_eq!(pair01.snd_ref(), Ok(&one()));
/// # }
/// ```
pub fn pair() -> Term {
    abs(abs(abs(
        app!(Var(1), Var(3), Var(2))
    )))
}

/// Applied to a Church-encoded pair `(a, b)` it yields `a`.
///
/// FST := λp.p TRUE = λ 1 TRUE
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::church::pairs::{pair, fst};
/// use lambda_calculus::church::numerals::{zero, one};
/// use lambda_calculus::reduction::beta;
/// use lambda_calculus::reduction::Order::*;
///
/// let pair_0_1 = app!(pair(), zero(), one());
///
/// assert_eq!(beta(fst().app(pair_0_1), NOR, 0, false), zero());
/// # }
/// ```
pub fn fst() -> Term { abs(Var(1).app(tru())) }

/// Applied to a Church-encoded pair `(a, b)` it yields `b`.
///
/// SND := λp.p FALSE = λ 1 FALSE
///
/// # Example
/// ```
/// # #[macro_use] extern crate lambda_calculus;
/// # fn main() {
/// use lambda_calculus::church::pairs::{pair, snd};
/// use lambda_calculus::church::numerals::{zero, one};
/// use lambda_calculus::reduction::beta;
/// use lambda_calculus::reduction::Order::*;
///
/// let pair_0_1 = app!(pair(), zero(), one());
///
/// assert_eq!(beta(snd().app(pair_0_1), NOR, 0, false), one());
/// # }
/// ```
pub fn snd() -> Term { abs(Var(1).app(fls())) }

impl Term {
    /// Checks whether `self` is a Church-encoded pair.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let pair01 = app!(pair(), zero(), one());
    ///
    /// assert!(pair01.is_pair());
    /// # }
    /// ```
    pub fn is_pair(&self) -> bool {
        self.unpair_ref().is_ok()
    }

    /// Splits a Church-encoded pair into a pair of terms, consuming `self`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.unpair(), Ok((zero(), one())));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn unpair(self) -> Result<(Term, Term), ChurchError> {
        let candidate = if let Abs(abstracted) = self { *abstracted } else { self };

        if let Ok((wrapped_a, b)) = candidate.unapp() {
            if let Ok(a) = wrapped_a.rhs() {
                Ok((a, b))
            } else {
                Err(NotAPair)
            }
        } else {
            Err(NotAPair)
        }
    }

    /// Splits a Church-encoded pair into a pair of references to its underlying terms.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.unpair_ref(), Ok((&zero(), &one())));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn unpair_ref(&self) -> Result<(&Term, &Term), ChurchError> {
        let candidate = if let Abs(ref abstracted) = *self { abstracted } else { self };

        if let Ok((wrapped_a, b)) = candidate.unapp_ref() {
            if let Ok(a) = wrapped_a.rhs_ref() {
                Ok((a, b))
            } else {
                Err(NotAPair)
            }
        } else {
            Err(NotAPair)
        }
    }

    /// Splits a Church-encoded pair into a pair of mutable references to its underlying terms.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let mut pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.unpair_mut(), Ok((&mut zero(), &mut one())));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn unpair_mut(&mut self) -> Result<(&mut Term, &mut Term), ChurchError> {
        let mut candidate = if let Abs(ref mut abstracted) = *self { abstracted } else { self };

        if let Ok((wrapped_a, b)) = candidate.unapp_mut() {
            if let Ok(a) = wrapped_a.rhs_mut() {
                Ok((a, b))
            } else {
                Err(NotAPair)
            }
        } else {
            Err(NotAPair)
        }
    }

    /// Returns the first term from a Church-encoded pair, consuming `self`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.fst(), Ok(zero()));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn fst(self) -> Result<Term, ChurchError> {
        Ok(try!(self.unpair()).0)
    }

    /// Returns a reference to the first term of a Church-encoded pair.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.fst_ref(), Ok(&zero()));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn fst_ref(&self) -> Result<&Term, ChurchError> {
        Ok(try!(self.unpair_ref()).0)
    }

    /// Returns a mutable reference to the first term of a Church-encoded pair.
    /// Returns a reference to the first term of a Church-encoded pair.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let mut pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.fst_mut(), Ok(&mut zero()));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn fst_mut(&mut self) -> Result<&mut Term, ChurchError> {
        Ok(try!(self.unpair_mut()).0)
    }

    /// Returns the second term from a Church-encoded pair, consuming `self`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.snd(), Ok(one()));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn snd(self) -> Result<Term, ChurchError> {
        Ok(try!(self.unpair()).1)
    }

    /// Returns a reference to the second term of a Church-encoded pair.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.snd_ref(), Ok(&one()));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn snd_ref(&self) -> Result<&Term, ChurchError> {
        Ok(try!(self.unpair_ref()).1)
    }

    /// Returns a mutable reference to the second term of a Church-encoded pair.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lambda_calculus;
    /// # fn main() {
    /// use lambda_calculus::church::pairs::pair;
    /// use lambda_calculus::church::numerals::{zero, one};
    ///
    /// let mut pair01 = app!(pair(), zero(), one());
    ///
    /// assert_eq!(pair01.snd_mut(), Ok(&mut one()));
    /// # }
    /// ```
    /// # Errors
    ///
    /// The function will return an error if `self` is not a Church pair.
    pub fn snd_mut(&mut self) -> Result<&mut Term, ChurchError> {
        Ok(try!(self.unpair_mut()).1)
    }
}

impl From<(Term, Term)> for Term {
    fn from((t1, t2): (Term, Term)) -> Self {
        abs(app!(Var(1), t1, t2))
    }
}
