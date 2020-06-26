/*-
 * Copyright 2020 James P. Howard, II <jh@jameshoward.us>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 *   The above copyright notice and this permission notice shall be included in
 *   all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

mod lein;
pub use lein::Lein;

mod utils;

/// Signals an error has been encountered by one of the encoders implementing
/// the [`PhonicsEncoder`] trait.
#[derive(Debug)]
pub enum PhonicsError {
    /// Signals that an unknown character was found and could not be processed.  Many
    /// phonetic spelling algorithms only accept a limited range of inputs.  For
    /// instance, certain French characters ("ç") may not be accepted by an English
    /// language encoder.  Each encoder may chose to handle these characters by
    /// ignoring them, treating them as equivalent to some other letter, or throwing
    /// an error.
    UnknownCharactersFound,
}

/// A trait for phonetic encoding of a string.
///
/// Instances of [`PhonicsEncoder`] should provide an encoder for strings.  It is
/// not expected to maintain state as phonetic encoders do not typically include
/// an updating mechanism.
///
/// ### Example
///
/// ```
/// use phonics::PhonicsEncoder;
///
/// fn foo<E: PhonicsEncoder + ?Sized>(e: &mut E) -> String {
///     return(e.encode("Scully").unwrap());
/// }
/// ```
pub trait PhonicsEncoder {
    /// Return a new encoder.
    ///
    /// This function should return a new instance of the phonetic encoder the
    /// the that implements this trait.
    ///
    /// ### Example
    ///
    /// ```
    /// use phonics::{Lein, PhonicsEncoder};
    ///
    /// let mut enc = Lein::new();
    /// ```
    fn new() -> Self;

    /// Encode a string and return the result or error.
    ///
    /// This function should encode a string according to the algorithm that
    /// implements this trait and return that string.  Under certain
    /// circumstances, an error may return so the result is returned in a
    /// [`Result`].
    ///
    /// ### Example
    ///
    /// ```
    /// use phonics::{Lein, PhonicsEncoder};
    ///
    /// let mut enc = Lein::new();
    /// enc.encode("Mulder");
    /// ```
    fn encode(&self, word: &str) -> Result<String, PhonicsError>;
}

/// A generic factory for phonetic encoders.
///
/// Instances of [`PhonicsEncoder`] should provide an encoder for strings.  It is
/// not expected to maintain state as phonetic encoders do not typically include
/// an updating mechanism.
///
/// ### Example
///
/// ```
/// use phonics::Phonics;
/// ```
pub struct Phonics<P: PhonicsEncoder> {
    encoder: P,
}

impl<P: PhonicsEncoder> PhonicsEncoder for Phonics<P> {
    fn new() -> Phonics<P> {
        Phonics { encoder: P::new() }
    }

    fn encode(&self, word: &str) -> Result<String, PhonicsError> {
        self.encoder.encode(word)
    }
}
