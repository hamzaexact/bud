use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub struct Builder<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Builder<'a> {
    pub fn init(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }
    pub fn create(&mut self, path: PathBuf) {
        let mut word = String::new();

        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\n' => {
                    self.next_char();
                }

                ';' => {
                    self.next_char();
                    break;
                }

                c if c.is_alphabetic() || c == '_' || c == '.' => {
                    while let Some(&c) = self.chars.peek() {
                        if c.is_alphanumeric() || c == '_' || c == '.' {
                            word.push(c);
                            self.next_char();
                        } else {
                            break;
                        }
                    }

                    match self.chars.peek() {
                        Some(&':') => {
                            self.next_char();
                            let next_path = path.join(&word);
                            fs::create_dir(&next_path).expect("Could not create folder");
                            self.create(next_path);
                        }
                        _ => {
                            let file_path = path.join(&word);
                            File::create(&file_path).expect("Could not create file");
                        }
                    }

                    word.clear();
                }

                _ => {
                    self.next_char();
                }
            }
        }
    }
    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }
}
