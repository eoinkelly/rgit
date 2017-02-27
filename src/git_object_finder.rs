use std;
use compress;
use git_sha;
use git_object;

use std::io::Read; // brings in a trait we need for read_to_end()

pub struct GitObjectFinder {}

impl GitObjectFinder {
    pub fn find(git_dir_path: std::path::PathBuf, sha: git_sha::GitSha) -> git_object::GitObject {
        // Create an instance of PathBuf which represents the absolute path to our object ...
        let object_path: std::path::PathBuf = git_dir_path.join("objects")
            .join(sha.dir_name)
            .join(sha.file_name);

        // ... and perform some basic run-time checks on it
        assert!(object_path.is_file(), "The given object does not exist");


        // Open the object file and zlib decode it ...
        let stream = std::fs::File::open(object_path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        let num_bytes = compress::zlib::Decoder::new(stream).read_to_end(&mut buffer).unwrap();

        // ... and perform some basic sanity checks
        assert!(num_bytes > 0, "Git objects cannot be zero bytes");

        println!("num bytes after decompression: {:?}", num_bytes);

        // Extract the type
        let (type_buffer, rest_1) = split_vec_on_byte(&buffer, b' ');
        let content_type = String::from_utf8(type_buffer).unwrap();
        println!("type : {:?}", content_type);

        // Extract the size
        let (size_buffer, content_buffer) = split_vec_on_byte(&rest_1, 0);
        let size: u64 = String::from_utf8(size_buffer).unwrap().parse().unwrap();
        println!("size buffer: {:?}", size);

        println!("content buffer: {:?}",
                 String::from_utf8(content_buffer.clone()));
        // After extracting type and size from the buffer what we have left is the content. Return
        // the appropriate GitObject enum value for whatever type we found.
        match content_type.as_str() {
            "commit" => {
                git_object::GitObject::GitCommit {
                    size: size,
                    content: content_buffer,
                }
            }
            "tree" => {
                git_object::GitObject::GitTree {
                    size: size,
                    content: content_buffer,
                }
            }
            "blob" => {
                git_object::GitObject::GitBlob {
                    size: size,
                    content: content_buffer,
                }
            }
            _ => panic!("Unrecognised git object type"),
        }
    }
}

// splits only once
// Take a vector of T and a single instance of T and split the vector into two vectors around the
// first instance of that value but not
//
// ```
// let a = 1, b = 2, c = 3, d = 4, e = 5;
// let v = vec![a, b, c, d, e];
// let (left, right) = split_vec_on_byte(v, c);
// assert_eq!(left, vec![a, b]);
// assert_eq!(right, vec![d, e]);
// ```
//
// TODO: this can probably be made generic
fn split_vec_on_byte(v: &Vec<u8>, byte: u8) -> (Vec<u8>, Vec<u8>) {
    let pos: usize = v.iter().position(|x| *x == byte).unwrap();
    let left = &v[0..pos];
    let right = &v[(pos + 1)..];
    (left.to_vec(), right.to_vec())
}

#[cfg(test)]
mod tests {
    use git_sha;
    use git_object;
    use fixtures;
    use git_object_finder;

    #[test]
    fn test_find_finds_object_which_exists() {
        let sha = git_sha::GitSha::from(fixtures::Fixtures::git_commit_sha());
        let git_object = git_object_finder::GitObjectFinder::find(fixtures::Fixtures::git_dir(),
                                                                  sha);

        match git_object {
            git_object::GitObject::GitCommit { size, content } => {
                assert_eq!(size, content.len() as u64);
            }
            git_object::GitObject::GitBlob { size, content } => {
                assert_eq!(size, content.len() as u64);
            }
            git_object::GitObject::GitTree { size, content } => {
                assert_eq!(size, content.len() as u64);
            }
        }
    }

    #[test]
    fn test_split_vec_on_byte_works_with_one_matching_byte() {
        let v = vec![11, 22, 0, 33, 44];
        let (left, right) = super::split_vec_on_byte(&v, 0);
        assert_eq!(left, vec![11, 22]);
        assert_eq!(right, vec![33, 44]);
    }

    #[test]
    fn test_split_vec_on_byte_works_with_multiple_matching_bytes() {
        let v = vec![11, 22, 0, 33, 44, 0, 55, 66];
        let (left, right) = super::split_vec_on_byte(&v, 0);
        assert_eq!(left, vec![11, 22]);
        assert_eq!(right, vec![33, 44, 0, 55, 66]);
    }
}
