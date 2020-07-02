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

/// Remove the first character a string
///
/// This function removes the first character of a string and returns the
/// remaining characters as a string.
pub fn strip_first_char(string: &str) -> String {
    String::from(
        string
            .char_indices()
            .nth(1)
            .and_then(|(i, _)| string.get(i..))
            .unwrap_or(""),
    )
}

/// Strip repeat characters
///
/// This function removes duplicated consecutive characters.  For instance,
/// many phonetic spelling algorithms replace a string of consecutive
/// identical characters with a single instance.  Therefore, this support
/// function provides that function, generically.
pub fn remove_duplicate_characters(string: &str) -> String {
    let mut return_string = String::new();
    let mut last_char = '0';

    for c in string.chars() {
        if c != last_char {
            return_string.push(c);
            last_char = c;
        }
    }

    return_string
}

pub fn transform_characters(string: &str, src: &str, dst: &str) -> String {
    let mut return_string = String::new();

    for c in string.chars() {
        if src.contains(c) {
            return_string.push_str(dst);
        } else {
            return_string.push(c);
        }
    }

    return_string
}

#[cfg(test)]
mod tests {
    use crate::utils;

    #[test]
    fn strip_first_char_works() {
        assert_eq!(utils::strip_first_char("Hilbert"), "ilbert");
        assert_eq!(utils::strip_first_char("H"), "");
        assert_eq!(utils::strip_first_char(""), "");
    }

    #[test]
    fn remove_duplicate_characters_works() {
        assert_eq!(
            utils::remove_duplicate_characters("abracadabra"),
            "abracadabra"
        );
        assert_eq!(utils::remove_duplicate_characters("bananna"), "banana");
        assert_eq!(utils::remove_duplicate_characters("baaaaa"), "ba");
        assert_eq!(utils::remove_duplicate_characters("baaaaab"), "bab");
        assert_eq!(utils::remove_duplicate_characters("baaaaabbbbbbb"), "bab");
        assert_eq!(utils::remove_duplicate_characters("H"), "H");
        assert_eq!(utils::remove_duplicate_characters(""), "");
    }

    #[test]
    fn transform_characters() {
        assert_eq!(
            utils::transform_characters("abracadabra", "ab", "1"),
            "11r1c1d11r1"
        );
        assert_eq!(utils::transform_characters("bananna", "n", "2"), "ba2a22a");
        assert_eq!(utils::transform_characters("baaaaa", "a", "T"), "bTTTTT");
        assert_eq!(utils::transform_characters("H", "H", "5"), "5");
        assert_eq!(utils::transform_characters("H", "F", "5"), "H");
        assert_eq!(utils::transform_characters("", "H", "5"), "");
    }
}
