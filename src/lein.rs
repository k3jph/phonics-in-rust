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

use regex::Regex;

use crate::utils;
use crate::PhonicsEncoder;
use crate::PhonicsError;

/// The Lein name coding procedure.
///
/// The Lein algorithm is only defined for inputs over the
/// standard English alphabet, _i.e._, "A-Z". Non-alphabetical
/// characters are removed from the string in a locale-dependent fashion.
/// This strips spaces, hyphens, and numbers.  Other letters, such as
/// 'Ü', may be permissible in the current locale but are unknown to
/// Lein.  For inputs outside of its known range, an error is returned
/// If `clean` is `false`, the encoder attempts to process the
/// strings.  The default value of `clean` is `true`.
///
/// # References
///
/// Billy T. Lynch and William L. Arends. "Selection of surname coding
/// procedure for the SRS record linkage system." United States
/// Department of Agriculture, Sample Survey Research Branch, Research
/// Division, Washington, 1977.
///
/// # Example
///
/// ```
/// use phonics::{Lein, PhonicsEncoder};
///
/// let mut enc = Lein::new();
/// enc.encode("Mulder");
/// ```
pub struct Lein {
    /// The special characters check regular expression is precompiled at instance instantiation
    /// to speed execution at run time.
    special_characters_re: Regex,

    /// The white space removal regular expression is precompiled at instance instantiation to
    /// speed execution at run time.
    white_space_re: Regex,

    /// The Lein specification has a standard maximum length.  However, it may be reasonable to
    /// use either a shorter or longer length, depending on the application.  The default value
    /// for `max_code_len` is is in [`Lein::MAX_CODE_LEN_DEFAULT`].
    pub max_code_len: usize,

    /// If `clean` is `true`, then `encode` will return
    /// [`PhonicsError::UnknownCharactersFound`]. If `clean` is `false`, then unknown characters
    /// are ignored.  The default value is [`Lein::CLEAN_DEFAULT`].
    pub clean: bool,
}

impl PhonicsEncoder for Lein {
    fn new() -> Lein {
        Lein {
            special_characters_re: Regex::new(r"[^A-Z]").unwrap(),
            white_space_re: Regex::new(r"[^A-Z]*").unwrap(),
            max_code_len: Lein::MAX_CODE_LEN_DEFAULT,
            clean: Lein::CLEAN_DEFAULT,
        }
    }

    fn encode(&self, source_string: &str) -> Result<String, PhonicsError> {
        let mut return_string = String::from(source_string);

        // First, uppercase it and test for unprocessable characters
        return_string = return_string.to_uppercase();
        if self.special_characters_re.is_match(&return_string) && self.clean {
            return Err(PhonicsError::UnknownCharactersFound);
        }
        let return_string = self.white_space_re.replace_all(&return_string, "");

        // Check if the return string is empty yet
        if return_string == "" {
            return Ok("".to_string());
        }

        // First character of key = first character of name
        let first_char = return_string.chars().next().unwrap();
        let return_string = utils::strip_first_char(&return_string);

        // Delete vowels and Y, W, and H
        let return_string = utils::transform_characters(&return_string, "AEIOUYWH", "");

        // Remove duplicate consecutive characters
        let return_string = utils::remove_duplicate_characters(&return_string);

        // D, T -> 1; M, N -> 2; L, R -> 3; B, F, P, V -> 4;
        // C, J, K, G, Q, S, X, Z -> 5
        let return_string = utils::transform_characters(&return_string, "DT", "1");
        let return_string = utils::transform_characters(&return_string, "MN", "2");
        let return_string = utils::transform_characters(&return_string, "LR", "3");
        let return_string = utils::transform_characters(&return_string, "BFPV", "4");
        let return_string = utils::transform_characters(&return_string, "CJKGQSXZ", "5");

        // Append word except for first character to first
        let mut return_string = format!("{}{}", first_char, return_string);

        // Zero-pad and truncate to requested length
        for _ in 1..self.max_code_len {
            return_string.push('0');
        }
        let mut return_string = return_string.as_str()[0..(self.max_code_len)].to_string();

        // Handle a critical edge case
        if return_string == "0000" {
            return_string = String::new();
        }

        Ok(return_string)
    }
}

impl Lein {
    /// The default value of the maximum allowable return length.
    pub const MAX_CODE_LEN_DEFAULT: usize = 4;

    /// The default value on the handling of special characters.
    pub const CLEAN_DEFAULT: bool = false;
}

#[cfg(test)]
mod tests {
    use crate::{Lein, PhonicsEncoder};

    #[test]
    fn test_lein_default_max_code_len() {
        let e = Lein::new();

        assert_eq!(e.max_code_len, Lein::MAX_CODE_LEN_DEFAULT);
    }

    #[test]
    fn test_lein_set_max_code_len() {
        let mut e = Lein::new();

        for i in 0..10 {
            e.max_code_len = i;
            assert_eq!(e.max_code_len, i);
        }
    }

    #[test]
    fn test_lein_default_clean() {
        let e = Lein::new();

        assert_eq!(e.clean, Lein::CLEAN_DEFAULT);
    }

    #[test]
    fn test_lein_set_clean() {
        let mut e = Lein::new();

        for i in &[false, true] {
            e.clean = *i;
            assert_eq!(e.clean, *i);
        }
    }
}
