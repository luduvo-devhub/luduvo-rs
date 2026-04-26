#![allow(unused)]

use std::result::Result;

use luduvo_api::users::profile;
use rand::{RngExt, rngs::ThreadRng};
use thiserror::Error;

use crate::codegen::{CodeComplexity, generate_code};

pub mod codegen;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ProfileError(#[from] profile::Error),

    #[error("user not found")]
    UserNotFound,

    #[error("verification code mismatch")]
    CodeMismatch,
}

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Bio,
}

#[derive(Copy, Clone, Debug)]
pub struct DiscordUser {
    pub id: usize,
}

#[derive(Debug)]
pub struct Settings {
    pub mode: Mode,
    pub code_length: usize,
}

impl Settings {
    pub fn new(mode: Option<Mode>, code_length: Option<usize>) -> Self {
        let default = Settings::default();

        Settings {
            mode: mode.unwrap_or(default.mode),
            code_length: code_length.unwrap_or(default.code_length),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            mode: Mode::default(),
            code_length: 10,
        }
    }
}

pub struct Client {
    pub rng: ThreadRng,
    pub client: profile::Client,

    pub settings: Settings,
}

impl Client {
    pub fn new(settings: Option<Settings>) -> Self {
        Client {
            rng: rand::rng(),
            client: profile::Client::new(None),

            settings: settings.unwrap_or_default(),
        }
    }

    pub fn generate_code(&mut self, code_complexity: Option<CodeComplexity>) -> String {
        generate_code(self, code_complexity)
    }

    pub async fn is_verified(
        &mut self,
        luduvo_username: String,
        discord_user: DiscordUser,
        code: String,
    ) -> Result<bool, Error> {
        let user = self.client.get_user_by_username(luduvo_username).await?;

        match self.settings.mode {
            Mode::Bio => {
                let bio = user.bio.unwrap_or("".to_string());

                if bio.contains(&code) {
                    return Ok(true);
                }
            }
        };

        Ok(false)
    }
}
