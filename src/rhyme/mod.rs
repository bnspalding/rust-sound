//! Tools for measuring similarity between collections of phonemes
//!
//! Rhyme provides two approaches to measuring similarity: strict and approximate. Strict rhyme
//! is an equality relation between syllables or collections of phonemes — two collections either
//! *do* or *don't* match. Approximate rhyme is a continuous measure of similarity — it allows us to say
//! *how* similar two collections are.

pub mod approx;
pub mod strict;
