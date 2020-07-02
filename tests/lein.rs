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

use phonics::{Lein, Phonics, PhonicsEncoder};

#[test]
fn test_lein_phonics() {
    let e = Phonics::<Lein>::new();

    assert_eq!(e.encode("Euler").unwrap(), "E330");
    assert_eq!(e.encode("Ellery").unwrap(), "E330");
    assert_eq!(e.encode("Gauss").unwrap(), "G500");
    assert_eq!(e.encode("Ghosh").unwrap(), "G500");
    assert_eq!(e.encode("Hilbert").unwrap(), "H343");
    assert_eq!(e.encode("Knuth").unwrap(), "K210");
    assert_eq!(e.encode("Kant").unwrap(), "K210");
    assert_eq!(e.encode("Lloyd").unwrap(), "L310");
    assert_eq!(e.encode("Ladd").unwrap(), "L100");
    assert_eq!(e.encode("Lukasiewicz").unwrap(), "L555");
    assert_eq!(e.encode("Lissajous").unwrap(), "L555");
    assert_eq!(e.encode("J").unwrap(), "J000");
    assert_eq!(e.encode("A").unwrap(), "A000");
    assert_eq!(e.encode("").unwrap(), "");
    assert_eq!(e.encode("Euler3.1415").unwrap(), "E330");
    assert_eq!(e.encode("12345").unwrap(), "");
    assert_eq!(e.encode("Müller-Lü denscheidt").unwrap(), "M333");
    assert_eq!(e.encode("Wikipedia").unwrap(), "W541");
    assert_eq!(e.encode("garçon").unwrap(), "G320");
    assert_eq!(e.encode("Breschnew").unwrap(), "B355");
}

#[test]
fn test_lein_directly() {
    let e = Lein::new();

    assert_eq!(e.encode("Euler").unwrap(), "E330");
    assert_eq!(e.encode("Ellery").unwrap(), "E330");
    assert_eq!(e.encode("Gauss").unwrap(), "G500");
    assert_eq!(e.encode("Ghosh").unwrap(), "G500");
    assert_eq!(e.encode("Hilbert").unwrap(), "H343");
    assert_eq!(e.encode("Knuth").unwrap(), "K210");
    assert_eq!(e.encode("Kant").unwrap(), "K210");
    assert_eq!(e.encode("Lloyd").unwrap(), "L310");
    assert_eq!(e.encode("Ladd").unwrap(), "L100");
    assert_eq!(e.encode("Lukasiewicz").unwrap(), "L555");
    assert_eq!(e.encode("Lissajous").unwrap(), "L555");
    assert_eq!(e.encode("J").unwrap(), "J000");
    assert_eq!(e.encode("A").unwrap(), "A000");
    assert_eq!(e.encode("").unwrap(), "");
    assert_eq!(e.encode("Euler3.1415").unwrap(), "E330");
    assert_eq!(e.encode("12345").unwrap(), "");
    assert_eq!(e.encode("Müller-Lü denscheidt").unwrap(), "M333");
    assert_eq!(e.encode("Wikipedia").unwrap(), "W541");
    assert_eq!(e.encode("garçon").unwrap(), "G320");
    assert_eq!(e.encode("Breschnew").unwrap(), "B355");
}

#[test]
fn test_lein_max_code_len() {
    let mut e = Lein::new();

    e.max_code_len = 3;
    assert_eq!(e.encode("Euler").unwrap(), "E33");
    assert_eq!(e.encode("Ellery").unwrap(), "E33");
    assert_eq!(e.encode("Gauss").unwrap(), "G50");
    assert_eq!(e.encode("Ghosh").unwrap(), "G50");
    assert_eq!(e.encode("Hilbert").unwrap(), "H34");
    assert_eq!(e.encode("Knuth").unwrap(), "K21");
    assert_eq!(e.encode("Kant").unwrap(), "K21");
    assert_eq!(e.encode("Lloyd").unwrap(), "L31");
    assert_eq!(e.encode("Ladd").unwrap(), "L10");
    assert_eq!(e.encode("Lukasiewicz").unwrap(), "L55");
    assert_eq!(e.encode("Lissajous").unwrap(), "L55");
    assert_eq!(e.encode("J").unwrap(), "J00");
    assert_eq!(e.encode("A").unwrap(), "A00");
    assert_eq!(e.encode("").unwrap(), "");
    assert_eq!(e.encode("Euler3.1415").unwrap(), "E33");
    assert_eq!(e.encode("12345").unwrap(), "");
    assert_eq!(e.encode("Müller-Lü denscheidt").unwrap(), "M33");
    assert_eq!(e.encode("Wikipedia").unwrap(), "W54");
    assert_eq!(e.encode("garçon").unwrap(), "G32");
    assert_eq!(e.encode("Breschnew").unwrap(), "B35");

    e.max_code_len = 6;
    assert_eq!(e.encode("Euler").unwrap(), "E33000");
    assert_eq!(e.encode("Ellery").unwrap(), "E33000");
    assert_eq!(e.encode("Gauss").unwrap(), "G50000");
    assert_eq!(e.encode("Ghosh").unwrap(), "G50000");
    assert_eq!(e.encode("Hilbert").unwrap(), "H34310");
    assert_eq!(e.encode("Knuth").unwrap(), "K21000");
    assert_eq!(e.encode("Kant").unwrap(), "K21000");
    assert_eq!(e.encode("Lloyd").unwrap(), "L31000");
    assert_eq!(e.encode("Ladd").unwrap(), "L10000");
    assert_eq!(e.encode("Lukasiewicz").unwrap(), "L55550");
    assert_eq!(e.encode("Lissajous").unwrap(), "L55500");
    assert_eq!(e.encode("J").unwrap(), "J00000");
    assert_eq!(e.encode("A").unwrap(), "A00000");
    assert_eq!(e.encode("").unwrap(), "");
    assert_eq!(e.encode("Euler3.1415").unwrap(), "E33000");
    assert_eq!(e.encode("12345").unwrap(), "");
    assert_eq!(e.encode("Müller-Lü denscheidt").unwrap(), "M33312");
    assert_eq!(e.encode("Wikipedia").unwrap(), "W54100");
    assert_eq!(e.encode("garçon").unwrap(), "G32000");
    assert_eq!(e.encode("Breschnew").unwrap(), "B35520");
}

#[test]
fn test_lein_clean() {
    let mut e = Lein::new();

    e.clean = true;
    assert_eq!(e.encode("Euler").unwrap(), "E330");
    assert_eq!(e.encode("Ellery").unwrap(), "E330");
    assert_eq!(e.encode("Gauss").unwrap(), "G500");
    assert_eq!(e.encode("Ghosh").unwrap(), "G500");
    assert_eq!(e.encode("Hilbert").unwrap(), "H343");
    assert_eq!(e.encode("Knuth").unwrap(), "K210");
    assert_eq!(e.encode("Kant").unwrap(), "K210");
    assert_eq!(e.encode("Lloyd").unwrap(), "L310");
    assert_eq!(e.encode("Ladd").unwrap(), "L100");
    assert_eq!(e.encode("Lukasiewicz").unwrap(), "L555");
    assert_eq!(e.encode("Lissajous").unwrap(), "L555");
    assert_eq!(e.encode("J").unwrap(), "J000");
    assert_eq!(e.encode("A").unwrap(), "A000");
    assert_eq!(e.encode("").unwrap(), "");
    assert!(e.encode("Euler3.1415").is_err());
    assert!(e.encode("12345").is_err());
    assert!(e.encode("Müller-Lü denscheidt").is_err());
    assert_eq!(e.encode("Wikipedia").unwrap(), "W541");
    assert!(e.encode("garçon").is_err());
    assert_eq!(e.encode("Breschnew").unwrap(), "B355");
}
