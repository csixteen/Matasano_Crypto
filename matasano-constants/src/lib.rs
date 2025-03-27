//! Frequencies generated from around 4.5 billion characters of English
//! text ([source]).
//!
//! [source]: <http://practicalcryptography.com/cryptanalysis/letter-frequencies-various-languages/english-letter-frequencies/>

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    /// Frequency of English single letters
    pub static ref MONOGRAM_FREQ: HashMap<char, f64> = HashMap::from_iter([
        ('a', 0.0855),
        ('b', 0.016),
        ('c', 0.0316),
        ('d', 0.0387),
        ('e', 0.121),
        ('f', 0.0218),
        ('g', 0.0209),
        ('h', 0.0496),
        ('i', 0.0733),
        ('j', 0.0022),
        ('k', 0.0081),
        ('l', 0.0421),
        ('m', 0.0253),
        ('n', 0.0717),
        ('o', 0.0747),
        ('p', 0.0207),
        ('q', 0.001),
        ('r', 0.0633),
        ('s', 0.0673),
        ('t', 0.0894),
        ('u', 0.0268),
        ('v', 0.0106),
        ('w', 0.0183),
        ('x', 0.0019),
        ('y', 0.0172),
        ('z', 0.0011),
    ]);

    /// Most common words in a 'news' text corpus containing around 900 million words.
    pub static ref WORD_FREQ: HashMap<&'static str, f64> = HashMap::from_iter([
        ("the", 0.0642),
        ("of", 0.0276),
        ("and", 0.0275),
        ("to", 0.0267),
        ("a", 0.0243),
        ("in", 0.0231),
        ("is", 0.0112),
        ("for", 0.0101),
        ("that", 0.0092),
        ("was", 0.0088),
        ("on", 0.0078),
        ("with", 0.0075),
        ("he", 0.0075),
        ("it", 0.0074),
        ("as", 0.0071),
        ("at", 0.0058),
        ("his", 0.0055),
        ("by", 0.0051),
        ("be", 0.0048),
        ("from", 0.0047),
        ("are", 0.0047),
        ("this", 0.0042),
        ("i", 0.0041),
        ("but", 0.004),
        ("have", 0.0039),
        ("an", 0.0037),
        ("has", 0.0035),
        ("not", 0.0034),
        ("they", 0.0033),
        ("or", 0.003),
    ]);
}

/// Calculates the similarity between two probability distributions. The lower the value, the more
/// similar they are. More info [`here`].
///
/// [`here`]: <https://en.wikipedia.org/wiki/Bhattacharyya_distance>
pub fn bhattacharyya_distance(a: &HashMap<char, f64>, b: &HashMap<char, f64>) -> f64 {
    let mut bc = 0_f64;

    for letter in MONOGRAM_FREQ.keys() {
        bc += f64::sqrt(
            a.get(letter).copied().unwrap_or_default() * b.get(letter).copied().unwrap_or_default(),
        );
    }

    -bc.ln()
}
