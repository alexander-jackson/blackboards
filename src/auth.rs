//! Stores authorisation primitives for running OAuth1.

use oauth::{Builder, Credentials};
use reqwest::blocking::Client;

#[derive(oauth::Request)]
struct Request {
    scope: &'static str,
    expiry: &'static str,
}

const SCOPE: &str = "urn:websignon.warwick.ac.uk:sso:service";
const EXPIRY: &str = "forever";
const OAUTH_CALLBACK: &str = "http://localhost:8000/authorised";

const REQUEST_TOKEN_URL: &str = "https://websignon.warwick.ac.uk/oauth/requestToken";
const AUTHORISE_TOKEN_URL: &str = "https://websignon.warwick.ac.uk/oauth/authorise";
const ACCESS_TOKEN_URL: &str = "https://websignon.warwick.ac.uk/oauth/accessToken";

/// Obtains a request token from the OAuth provider, corresponding to Stage 1.
///
/// Using the `consumer_key` and `consumer_secret`, signs a request to the SSO service and requests
/// a new request token. This can then be used later on to become an access token.
pub fn obtain_request_token(consumer_key: &str, consumer_secret: &str) -> String {
    let credentials = Credentials::new(consumer_key, consumer_secret);
    let request = Request {
        scope: SCOPE,
        expiry: EXPIRY,
    };
    let auth = Builder::<_, _>::new(credentials, oauth::HmacSha1)
        .callback(OAUTH_CALLBACK)
        .post(&REQUEST_TOKEN_URL, &request);

    let client = Client::new();
    let request = client
        .post(REQUEST_TOKEN_URL)
        .header("Authorization", auth)
        .header("User-Agent", "Cinnamon")
        .query(&[("scope", SCOPE), ("expiry", EXPIRY)]);

    let response = request.send().unwrap();
    response.text().unwrap()
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
) -> String {
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

    response.text().unwrap()
}

/// Builds the callback url for OAuth1.
pub fn build_callback(token: &str) -> String {
    format!(
        "{}?oauth_token={}&oauth_callback={}",
        AUTHORISE_TOKEN_URL, token, OAUTH_CALLBACK
    )
}
