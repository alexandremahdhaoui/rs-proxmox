use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use reqwest::{Client, ClientBuilder, Method, RequestBuilder, Response, Url};
use reqwest::cookie::{Cookie, Jar};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};

const PROXMOX_USER_KEY: &str = "PROXMOX_USER";
const PROXMOX_PASS_KEY: &str = "PROXMOX_PASS";
const PROXMOX_REALM_KEY: &str = "PROXMOX_REALM";
const PROXMOX_BASEURL_KEY: &str = "PROXMOX_BASEURL";
const PROXMOX_PORT_KEY: &str = "PROXMOX_PORT";

const PROXMOX_REST_API_PATH: &str = "api2/json";
const PROXMOX_AUTHENTICATION_PATH: &str = "access/ticket";

const PVE_AUTH_COOKIE_KEY: &str = "PVEAuthCookie";
const CSRF_PREVENTION_TOKEN_KEY: &str = "CSRFPreventionToken";

const HTTP_METHOD_ERROR: &str = "http method should be GET, POST, PUT or DELETE";

#[derive(Serialize, Deserialize)]
struct ProxmoxAuthenticationResponse {
    csrf_prevention_token: String,
    ticket: String,
}

/// # Proxmox Client
///
pub struct ProxmoxClient {
    user: Option<String>,
    password: Option<String>,
    realm: Option<String>,
    base_url: Option<String>,
    port: Option<String>,
}

impl ProxmoxClient {
    /// # New
    ///
    /// new generates a new ProxmoxClient, loading `user`, `password`, `realm`, `base_url` & `port`.
    pub fn new() -> Self {
        Self{
            user: get_env_or_none(PROXMOX_USER_KEY),
            password: get_env_or_none(PROXMOX_PASS_KEY),
            realm: get_env_or_none(PROXMOX_REALM_KEY),
            base_url: get_env_or_none(PROXMOX_BASEURL_KEY),
            port: get_env_or_none(PROXMOX_PORT_KEY),
        }
    }

    /// # Http client
    ///
    /// 1. Get the PVEAuthCookie, also known as `ticket`:
    /// ```shell
    /// curl -k -d 'username=root@pam' --data-urlencode 'password=xxxxxxxxx' https://10.0.0.1:8006/api2/json/access/ticket
    /// ```
    /// 2. From the response, we also get the `csrfprevention_token`.
    ///
    /// Now we can use the ticket & csrf token as shown below:
    /// ```shell
    /// curl -k -b "PVEAuthCookie=PVE:..." https://10.0.0.1:8006/api2/json/access/ticket
    /// curl -XDELETE -H "csrfprevention_token: 4EE..."
    /// ```
    ///
    /// Please note, the Auth ticket should be set as a Cookie & the csrfprevention_token as a Header.
    async fn http_client(&self) -> reqwest::Result<Client> {
        let url = self.url();

        // client to request the authentication ticket & token
        let c = unsafe_client_builder().build().unwrap();
        let res = c.post(format_uri(
            &self.base_url.as_ref().unwrap(),
            &self.port.as_ref().unwrap(),
            PROXMOX_AUTHENTICATION_PATH
        )).send().await?;
        let res: ProxmoxAuthenticationResponse = res.json().await?;


        // Set the `CSRFPreventionToken` header.
        let mut m = HeaderMap::new();
        m.insert(
            CSRF_PREVENTION_TOKEN_KEY,
            res.csrf_prevention_token.parse().unwrap()
        );

        // Set the `PVEAuthCookie`.
        // At this point, it's safe to unwrap the `base_url` if ProxmoxClient was built using the
        // Builder.
        let mut jar = Jar::default();
        jar.add_cookie_str(&format_cookie(
            PVE_AUTH_COOKIE_KEY,
            &res.ticket
        ), &url);

        // Set the headers, cookies, & build the client
        unsafe_client_builder()
            .default_headers(m)
            .cookie_provider(Arc::new(jar))
            .build()
    }

    /// # Request
    ///
    /// `request` instantiates a new http client, creates a new request builder based on the
    /// provided method of type reqwest::Method.
    /// Then, depending on the method, we either set the payload as query argument (GET) or as form
    /// (POST, PUT, DELETE).
    ///
    /// Please note, reqwest::Method should be GET, POST, PUT or DELETE. Other methods will panic.
    pub async fn request<T: Serialize + Sized>(
        &self,
        method: Method,
        path: &str,
        payload: T
    ) -> reqwest::Result<Response> {
        let mut b: RequestBuilder = self.http_client().await?.request(
            method.clone(),
            // It's safe to unwrap the base_url if ProxmoxClient was built using the Builder.
            format_uri(&self.base_url.as_ref().unwrap(), path, path)
        );
        let _ = match method {
            Method::GET => b = b.query(&payload),
            Method::POST | Method::PUT | Method::DELETE => b = b.form(&payload),
            _ => {
                panic!("{}", HTTP_METHOD_ERROR)
            }
        };
        b.send().await
    }

    fn url(&self) -> Url {
        format!("https://{}:{}", self.base_url.as_ref().unwrap(), self.port.as_ref().unwrap())
            .parse::<Url>().unwrap()
    }
}

fn format_cookie(key: &str, value: &str) -> String {
    format!("{}={}", key, value)
}

fn format_uri(base_url: &str, port: &str, path: &str) -> String {
    format!("https://{}:{}/{}", base_url, port, path)
}

fn get_env_or_none(key: &str) -> Option<String> {
    if let Ok(value) = env::var(key) {
        if value == "" {
            return None
        }
        return Some(value)
    }
    None
}

fn unsafe_client_builder() -> ClientBuilder {
    Client::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
}

//--------------------------------------------------------------------------------------------------

pub struct Builder {
    client: ProxmoxClient
}

impl Builder {
    pub fn new() -> Self {
        Self { client: ProxmoxClient::new() }
    }

    pub fn build(&mut self) -> Result<ProxmoxClient, String> {
        if self.client.user == None { Err("user should be set".parse().unwrap()) }
        else if self.client.password == None { Err("password should be set".parse().unwrap()) }
        else if self.client.realm == None {Err("user's realm should be set".parse().unwrap())}
        else if self.client.base_url == None {Err("base_url should be set".parse().unwrap())}
        else if self.client.port == None { Err("port should be set".parse().unwrap()) }
        else {
            Ok(std::mem::replace(&mut self.client, ProxmoxClient::new()))
        }
    }
    pub fn set_user(&mut self, user: &str) -> &mut Self{
        if user == "" {
            self.client.user = None;
        } else {
            self.client.user = Some(user.to_string());
        }
        self
    }
    pub fn set_password(&mut self, password: &str) -> &mut Self{
        if password == "" {
            self.client.password = None;
        } else {
            self.client.password = Some(password.to_string());
        }
        self
    }
    pub fn set_realm(&mut self, realm: &str) -> &mut Self{
        if realm == "" {
            self.client.realm = None;
        } else {
            self.client.realm = Some(realm.to_string());
        }
        self
    }
    pub fn set_base_url(&mut self, base_url: &str) -> &mut Self{
        if base_url == "" {
            self.client.base_url = None;
        } else {
            self.client.base_url = Some(base_url.to_string());
        }
        self
    }
    pub fn set_port(&mut self, port: &str) -> &mut Self{
        if port == "" {
            self.client.port = None;
        } else {
            self.client.port = Some(port.to_string());
        }
        self
    }
}

//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
    }
}
