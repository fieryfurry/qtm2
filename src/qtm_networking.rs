// SPDX-License-Identifier: BSD-2-Clause-Patent

use reqwest::blocking::{Client};
use reqwest::blocking::multipart::Form;
use reqwest::{header, StatusCode};
use reqwest::header::HeaderMap;
use tracing::{info, warn};

#[derive(Debug)]
pub(crate) struct QtmNetworking {
    pub(crate) client: Client,
}

impl QtmNetworking {
    pub(crate) fn try_new() -> anyhow::Result<Self> {
        Ok(Self {
            client: Self::get_client()?,
        })
    }

    pub(crate) fn login(&self, username: String, password: String) -> bool {
        let form = Self::get_login_multipart(username, password);
        dbg!(&form);
        let response = self.client.post("https://www.gaytorrent.ru/takelogin.php")
            .multipart(form)
            .send()
            .map_err(anyhow::Error::new);
        match response {
            Ok(response) => {
                match response.status() {
                    StatusCode::FOUND => {
                        info!("Authenticated");
                        true
                    },
                    StatusCode::OK => {
                        info!("Not authenticated");
                        false
                    },
                    others => {
                        info!(?others, "Unmatched status code; not authenticated");
                        false
                    }
                }
            }
            Err(err) => {
                warn!(?err, "Error when sending request");
                false
            }
        }
    }

    pub fn get_login_multipart(username: String, password: String) -> Form {
        Form::new()
            .text("username", username)
            .text("password", password)
            .text("returnto", "/genrelist.php")
    }

    fn get_user_agent_by_os() -> &'static str {
        if cfg!(target_os = "windows") {
            "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/47.0.2526.111 Safari/537.36"
        } else if cfg!(target_os = "macos") {
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_2) AppleWebKit/601.3.9 (KHTML, like Gecko) Version/9.0.2 Safari/601.3.9"
        } else if cfg!(target_os = "linux") {
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1"
        } else {
            "Mozilla/5.0 (X11; CrOS x86_64 8172.45.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.64 Safari/537.36"
        }
    }

    fn get_default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::HOST,
            header::HeaderValue::from_static("www.gaytorrent.ru"),
        );
        headers.insert(
            header::CONNECTION,
            header::HeaderValue::from_static("Keep-Alive"),
        );
        headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-cache"),
        );
        headers
    }

    fn get_client() -> anyhow::Result<Client> {
        let client = Client::builder()
            .user_agent(Self::get_user_agent_by_os())
            .default_headers(Self::get_default_headers())
            .cookie_store(true)
            .build();
        if let Err(err) = &client {
            warn!(?err, "Failed to construct client");
        }
        client.map_err(anyhow::Error::new)
    }
}
