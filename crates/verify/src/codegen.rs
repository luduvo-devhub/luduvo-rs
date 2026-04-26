use rand::{
    RngExt,
    distr::{Alphabetic, Alphanumeric},
    rngs::ThreadRng,
};

use super::Client;

#[derive(Default, Debug)]
pub enum CodeComplexity {
    #[default]
    Low,

    Medium,
    High,
}

pub fn generate_code(mut client: &mut Client, code_complexity: Option<CodeComplexity>) -> String {
    let charset: &[u8] = match code_complexity.unwrap_or(CodeComplexity::Low) {
        CodeComplexity::Low => {
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" as &[u8]
        }

        CodeComplexity::Medium => {
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789" as &[u8]
        }

        CodeComplexity::High => {
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:,.<>?" as &[u8]
        }
    };

    let mut out = String::with_capacity(client.settings.code_length);

    for _ in 0..client.settings.code_length {
        let index = client.rng.random_range(0..charset.len());

        out.push(charset[index] as char);
    }

    out
}
