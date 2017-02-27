use regex;

lazy_static! {
    static ref SHA_REGEX: regex::Regex = regex::Regex::new(r"\A[a-fA-F0-9]{40,40}\z").unwrap();
}

pub struct GitSha {
    pub dir_name: String,
    pub file_name: String,
    pub sha: String,
}

impl GitSha {
    pub fn from(sha: String) -> Self {

        assert!(SHA_REGEX.is_match(&sha),
                "SHA1 checksum must be exactly 40 chars from set: 0-9a-fA-F");

        GitSha {
            dir_name: String::from(&sha[0..2]), // in rust .. does not include rhs
            file_name: String::from(&sha[2..]),
            sha: sha,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_creates_gitsha_given_valid_arg() {
        let sha = super::GitSha::from(String::from("b05145df2bafb5f470b5eabc4f4e91c23850c831"));
        assert_eq!(sha.dir_name, "b0");
        assert_eq!(sha.file_name, "5145df2bafb5f470b5eabc4f4e91c23850c831");
        assert_eq!(sha.sha, "b05145df2bafb5f470b5eabc4f4e91c23850c831");
    }

    #[test]
    #[should_panic(expected = "SHA1 checksum must be exactly 40 chars from set: 0-9a-fA-F")]
    fn test_from_panics_if_sha_contains_unexpected_chars() {
        let forty_char_str = String::from("i am not your valid sha checksum thingxx");
        super::GitSha::from(forty_char_str);
    }

    #[test]
    #[should_panic(expected = "SHA1 checksum must be exactly 40 chars from set: 0-9a-fA-F")]
    fn test_from_panics_if_sha_is_too_short() {
        let too_short_str = String::from("abc123");
        super::GitSha::from(too_short_str);
    }
}
