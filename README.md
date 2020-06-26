# Phonetic Spelling Algorithms in Rust

[![Download](https://img.shields.io/crates/d/phonics)](https://crates.io/crates/phonics)
[![License](https://img.shields.io/crates/l/phonics)](https://github.com/Hoverbear/getset/blob/master/LICENSE)
[![Docs](https://docs.rs/phonics/badge.svg)](https://docs.rs/phonics/)
[![Build Status](https://img.shields.io/travis/k3jph/phonics-in-rust.svg)](https://travis-ci.org/k3jph/phonics-in-rust)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.3908742.svg)](https://doi.org/10.5281/zenodo.3908742)

This is a package to port the [Phonics in
R](https://howardjp.github.io/phonics/) package to Rust.  In large
part, this package is being developed to learn how to program in
Rust.  Accordingly, there is a substantial effort to translate the
phonetic tools into _idiomatic_ Rust.  Each algorithm is implemented
as a separate struct based on a `PhoneticEncoder` trait, providing
a uniform interface.  This package is not a high priority for the
author and will likely be developed slowly.

## Algorithms included

* Lein

_More to come, as they are reimplemented from the R package._

## Dependencies

* [getset](https://crates.io/crates/getset)
* [regex](https://crates.io/crates/regex)

## Contribution guidelines

* Use [GitFlow](http://nvie.com/posts/a-successful-git-branching-model/)
* Write unit tests
* Document your functions

## For more information

* [Phonics in R website](https://howardjp.github.io/phonics/)
* James P. Howard, II <<jh@jameshoward.us>>
