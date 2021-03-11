//! Stores authorisation primitives for running OAuth1.

use std::collections::HashMap;
use std::str::FromStr;

use oauth::{Builder, Credentials};
use reqwest::blocking::Client;
use url::form_urlencoded;

const SCOPE: &str = "urn:websignon.warwick.ac.uk:sso:service";
const EXPIRY: &str = "forever";

#[cfg(debug_assertions)]
const OAUTH_CALLBACK: &str = "http://localhost:8000/authorised";

#[cfg(not(debug_assertions))]
const OAUTH_CALLBACK: &str = "http://blackboards.pl/authorised";

const REQUEST_TOKEN_URL: &str = "https://websignon.warwick.ac.uk/oauth/requestToken";
const AUTHORISE_TOKEN_URL: &str = "https://websignon.warwick.ac.uk/oauth/authorise";
const ACCESS_TOKEN_URL: &str = "https://websignon.warwick.ac.uk/oauth/accessToken";
const ATTRIBUTES_URL: &str = "https://websignon.warwick.ac.uk/oauth/authenticate/attributes";

#[derive(oauth::Request)]
struct Request {
    scope: &'static str,
    expiry: &'static str,
}

/// Represents a token and secret pair from the Warwick API.
#[derive(Debug)]
pub struct TokenPair {
    /// The public token itself
    pub token: String,
    /// The related secret
    pub secret: String,
}

/// Represents the information requested from the Warwick API.
#[derive(Debug)]
pub struct UserInfo {
    /// The user's Warwick ID
    pub id: u32,
    /// The user's name
    pub name: String,
}

impl From<HashMap<&str, &str>> for UserInfo {
    fn from(map: HashMap<&str, &str>) -> Self {
        Self {
            id: u32::from_str(map["id"]).unwrap(),
            name: String::from(map["name"]),
        }
    }
}

fn parse_mappings(text: &str) -> HashMap<&str, &str> {
    text.trim()
        .split('\n')
        .map(|line| {
            let index = line.find('=').unwrap();
            (&line[..index], &line[index + 1..])
        })
        .collect()
}

/// Builds the callback url for OAuth1.
pub fn build_callback(token: &str, uri: &str) -> String {
    let callback = format!("{}/{}", OAUTH_CALLBACK, uri);
    log::debug!("Callback uri: {}", callback);

    format!(
        "{}?oauth_token={}&oauth_callback={}",
        AUTHORISE_TOKEN_URL, token, callback
    )
}

/// Obtains a request token from the OAuth provider, corresponding to Stage 1.
///
/// Using the `consumer_key` and `consumer_secret`, signs a request to the SSO service and requests
/// a new request token. This can then be used later on to become an access token.
pub fn obtain_request_token(consumer_key: &str, consumer_secret: &str, uri: &str) -> TokenPair {
    let credentials = Credentials::new(consumer_key, consumer_secret);
    let request = Request {
        scope: SCOPE,
        expiry: EXPIRY,
    };

    let callback = format!("{}/{}", OAUTH_CALLBACK, uri);
    let auth = Builder::<_, _>::new(credentials, oauth::HmacSha1)
        .callback(callback.as_str())
        .post(&REQUEST_TOKEN_URL, &request);

    let client = Client::new();
    let request = client
        .post(REQUEST_TOKEN_URL)
        .header("Authorization", auth)
        .header("User-Agent", "Cinnamon")
        .query(&[("scope", SCOPE), ("expiry", EXPIRY)]);

    let response = request.send().unwrap();
    let text = response.text().unwrap();

    let query_params: HashMap<_, _> = form_urlencoded::parse(&text.as_bytes()).collect();
    let token = query_params["oauth_token"].to_string();
    let secret = query_params["oauth_token_secret"].to_string();

    TokenPair { token, secret }
}

/// Exchanges a request token for an access token, corresponding to Stage 3.
///
/// Using the `consumer_key`, `consumer_secret`, `oauth_token` and `oauth_verifier`, signs a
/// request to the SSO service and requests for the request token received previously to become an
/// access token. This requires the client secret from Stage 1 as well.
pub fn exchange_request_for_access(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_token: &str,
    oauth_secret: &str,
    oauth_verifier: &str,
) -> TokenPair {
    let token = oauth::Credentials::new(oauth_token, oauth_secret);

    let credentials = Credentials::new(consumer_key, consumer_secret);
    let auth = Builder::<_, _>::new(credentials, oauth::HmacSha1)
        .verifier(oauth_verifier)
        .token(Some(token))
        .post(&ACCESS_TOKEN_URL, &());

    let client = Client::new();
    let request = client
        .post(ACCESS_TOKEN_URL)
        .header("Authorization", auth)
        .header("User-Agent", "Cinnamon");

    let response = request.send().unwrap();
    let text = response.text().unwrap();

    let query_params: HashMap<_, _> = form_urlencoded::parse(&text.as_bytes()).collect();
    let token = query_params["oauth_token"].to_string();
    let secret = query_params["oauth_token_secret"].to_string();

    TokenPair { token, secret }
}

/// Requests the user's information from the Warwick API.
///
/// Forms a request to the Warwick API using the user's token and secret, before extracting just
/// the information needed for the website.
pub fn request_user_information(
    token: &str,
    secret: &str,
    consumer_key: &str,
    consumer_secret: &str,
) -> UserInfo {
    let token = oauth::Credentials::new(token, secret);

    let credentials = Credentials::new(consumer_key, consumer_secret);
    let auth = Builder::<_, _>::new(credentials, oauth::HmacSha1)
        .token(Some(token))
        .post(&ATTRIBUTES_URL, &());

    let client = Client::new();
    let request = client
        .post(ATTRIBUTES_URL)
        .header("Authorization", auth)
        .header("User-Agent", "Cinnamon");

    let response = request.send().unwrap();
    let text = response.text().unwrap();

    UserInfo::from(parse_mappings(&text))
}
