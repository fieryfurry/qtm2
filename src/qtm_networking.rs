// SPDX-License-Identifier: BSD-2-Clause-Patent

use reqwest::{header, Proxy};
use reqwest::blocking::{Body, Client};
use reqwest::header::{HeaderMap, HeaderValue};
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
        let form = Self::get_login_form(username, password);
        let request = self
            .client
            .post("https://www.gaytorrent.ru/takelogin.php")
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("multipart/form-data; boundary=--"),
            )
            .header(header::CONTENT_LENGTH, form.as_bytes().len())
            .body(Body::from(form));

        match request.send().map_err(anyhow::Error::new) {
            Ok(response) =>
                match response.url().path() {
                    "/genrelist.php" => {
                        info!("Authenticated");
                        true
                    }
                    "/takelogin.php" => {
                        info!("Not authenticated");
                        false
                    }
                    others => {
                        info!(?others, "Unmatched redirect; not authenticated");
                        false
                    }
                },
            Err(err) => {
                warn!(?err, "Error when sending request");
                false
            }
        }
    }

    fn get_form_part(value: &str) -> String {
        format!("Content-Disposition: form-data; Content-Type: text/plain; charset=utf8; name=\"{value}\"")
    }

    fn get_login_form(username: String, password: String) -> String {
        format!(
            "----\r\n{}\r\n\r\n{username}\r\n----\r\n----\r\n\
        {}\r\n\r\n{password}\r\n----\r\n----\r\n{}\r\n\r\n/genrelist.php\r\n\
        ----\r\n------\r\n",
            Self::get_form_part("username"),
            Self::get_form_part("password"),
            Self::get_form_part("returnto")
        )
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
        headers.insert(header::HOST, HeaderValue::from_static("www.gaytorrent.ru"));
        headers.insert(header::CONNECTION, HeaderValue::from_static("Keep-Alive"));
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        headers
    }

    fn get_client() -> anyhow::Result<Client> {
        let client = Client::builder()
            .user_agent(Self::get_user_agent_by_os())
            .default_headers(Self::get_default_headers())
            .cookie_store(true)
            .proxy(Proxy::https("localhost:8080").unwrap())
            .build();
        if let Err(err) = &client {
            warn!(?err, "Failed to construct client");
        }
        client.map_err(anyhow::Error::new)
    }
}
