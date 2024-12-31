use rand::distributions::Distribution;
use rand::Rng;
use std::collections::HashSet;

#[derive(Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct AllCharacters;

impl Distribution<u8> for AllCharacters {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10 + 32;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789\
                -=[]\\;',./!@#$%^&*()_+{}|:\"<>?`~";

        loop {
            let var = rng.next_u32() >> (32 - 8);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

const ALPHABET_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER_CHAR: &str = "0123456789";
const SPECIAL_CHAR: &str = "-=[]\\;',./!@#$%^&*()_+{}|:\"<>?`~";

pub fn passwd_check(s: &str, special_char: bool) -> bool {
    let hash_alphabet_lower = ALPHABET_LOWER.chars().collect::<HashSet<_>>();
    let hash_alphabet_upper = ALPHABET_UPPER.chars().collect::<HashSet<_>>();
    let hash_number = NUMBER_CHAR.chars().collect::<HashSet<_>>();
    let hash_special = SPECIAL_CHAR.chars().collect::<HashSet<_>>();

    let x = &s.chars().collect::<HashSet<_>>();

    hash_alphabet_lower.intersection(x).next().is_some()
        && hash_alphabet_upper.intersection(x).next().is_some()
        && hash_number.intersection(x).next().is_some()
        && (hash_special.intersection(x).next().is_some() == special_char)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_passwd_check() {
        let s1: &str = "a1A";
        let s2: &str = "a1A!";

        assert!(!passwd_check(s1, true), "{}", s1);
        assert!(passwd_check(s2, true), "{}", s2);

        assert!(passwd_check(s1, false), "{}", s1);
        assert!(!passwd_check(s2, false), "{}", s2);

        let s3: &str = "feoA23jiE";
        let s4: &str = "&jiA42BtA";

        assert!(!passwd_check(s3, true), "{}", s3);
        assert!(passwd_check(s4, true), "{}", s4);

        assert!(passwd_check(s3, false), "{}", s3);
        assert!(!passwd_check(s4, false), "{}", s4);
    }

    #[test]
    fn test_all_characters_distribution() {
        let charset = AllCharacters;
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..1000 {
            let c = charset.sample(&mut rng);
            assert!(
                ALL_CHARSET_BYTES.contains(&c),
                "Character {} not in charset",
                c
            );
        }
    }

    const ALL_CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789\
                                -=[]\\;',./!@#$%^&*()_+{}|:\"<>?`~";
    const ALL_CHARSET_BYTES: &[u8] = ALL_CHARSET.as_bytes();
}
