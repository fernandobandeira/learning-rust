#![feature(test)]
#![allow(dead_code)]

extern crate test;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// 1 - Arrays & Hashing
mod contains_duplicate;
mod group_anagrams;
mod two_sum;
mod valid_anagram;

fn main() {}
