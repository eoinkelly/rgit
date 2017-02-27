use std;

pub enum GitObject {
    GitCommit { size: u64, content: Vec<u8> },
    GitTree { size: u64, content: Vec<u8> },
    GitBlob { size: u64, content: Vec<u8> },
}

impl std::fmt::Display for GitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::from("\n");

        match self {
            &GitObject::GitCommit { ref size, ref content } => {
                output.push_str(&format!("\n  Type: commit\n"));
                output.push_str(&format!("  Size: {} bytes\n", size));
                output.push_str(&format!("  Content:\n{}\n", present_content(&content)));
                write!(f, "{}", output)
            }
            &GitObject::GitTree { ref size, ref content } => {
                output.push_str(&format!("\n  Type: tree\n"));
                output.push_str(&format!("  Size: {} bytes\n", size));
                output.push_str(&format!("  Content: {:?}\n", content));
                output.push_str(&format!("  Content:\n{}\n", present_content(&content)));
                write!(f, "{}", output)
            }
            &GitObject::GitBlob { ref size, ref content } => {
                output.push_str(&format!("\n  Type: blob\n"));
                output.push_str(&format!("  Size: {} bytes\n", size));
                output.push_str(&format!("  Content:\n{}\n", present_content(&content)));
                write!(f, "{}", output)
            }
        }
    }
}

// Private helper functions

fn present_content(content: &Vec<u8>) -> String {
    let mut output = String::new();
    for line in std::str::from_utf8(content).unwrap().lines() {
        output.push_str(&format!("    {}\n", line));
    }
    output
}
