use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

use chrono::Utc;
use google_drive3::api::Scope as GoogleScope;
use google_drive3::client::GetToken;
use google_drive3::hyper::client::HttpConnector;
use google_drive3::hyper::{Body, Client as HyperClient};
use google_drive3::hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use google_drive3::DriveHub;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, StandardRevocableToken, TokenResponse,
    TokenUrl,
};
use tiny_http::Server;
use url::Url;

pub struct GoogleClient {
    pub hub: DriveHub<HttpsConnector<HttpConnector>>,
}

pub struct AuthFlow {
    pub client: BasicClient,
    pub token_response: Mutex<Option<BasicTokenResponse>>,
}

impl GoogleClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let auth_flow = AuthFlow::new()?;

        let (authorize_url, csrf_state, code_verifier) = auth_flow.get_authorization_url()?;
        let (code, state) = auth_flow.get_authorization_code(&authorize_url)?;
        auth_flow.exchange_code(code, code_verifier, state, csrf_state)?;

        Ok(Self::from_auth_flow(auth_flow))
    }
    pub fn from_auth_flow(auth_flow: AuthFlow) -> Self {
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        let client = HyperClient::builder().build::<_, Body>(connector);
        let hub = DriveHub::new(client, auth_flow);

        Self { hub }
    }
}

impl AuthFlow {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Credentials 'develop' are being used, avoid using it in production");

        let client_id = ClientId::new(
            "1009503188998-n33ka24o0i0bciu0jbhlp50g9pbkku6r.apps.googleusercontent.com".to_string(),
        );
        let client_secret = Some(ClientSecret::new(
            "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".to_string(),
        ));

        let redirect_url = RedirectUrl::new("http://localhost:8000".to_string())?;
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;
        let revocation_url =
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())?;

        let client = BasicClient::new(client_id, client_secret, auth_url, Some(token_url))
            .set_revocation_uri(revocation_url)
            .set_redirect_uri(redirect_url);

        Ok(Self {
            client,
            token_response: Mutex::new(None),
        })
    }

    pub fn get_authorization_url(
        &self,
    ) -> Result<(Url, CsrfToken, PkceCodeVerifier), Box<dyn std::error::Error>> {
        let (code_challenge, code_verifier) = PkceCodeChallenge::new_random_sha256();
        let metadata_scope = Scope::new(GoogleScope::MetadataReadonly.as_ref().to_string());

        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(code_challenge)
            .add_scope(metadata_scope)
            .url();

        Ok((authorize_url, csrf_state, code_verifier))
    }

    pub fn get_authorization_code(
        &self,
        authorize_url: &Url,
    ) -> Result<(AuthorizationCode, CsrfToken), Box<dyn std::error::Error>> {
        if webbrowser::open(authorize_url.as_str()).is_ok() {
            let server = Server::http("localhost:8000")
                .map_err(|e| format!("Failed to start server: {}", e))?;

            for request in server.incoming_requests() {
                let url = Url::parse(&("http://localhost".to_string() + request.url())).unwrap();

                let mut query_pairs = url.query_pairs();
                if query_pairs.clone().count() <= 1 {
                    return Err("Not enough query parameters".into());
                }

                let (_, state) = query_pairs
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();
                let (_, code) = query_pairs
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let response = tiny_http::Response::from_string("Success");
                request.respond(response)?;

                let code = AuthorizationCode::new(code.to_string());
                let state = CsrfToken::new(state.to_string());

                drop(server);
                return Ok((code, state));
            }
        }

        Err("Failed to open browser".into())
    }

    pub fn exchange_code(
        &self,
        code: AuthorizationCode,
        code_verifier: PkceCodeVerifier,
        state: CsrfToken,
        csrf_state: CsrfToken,
    ) -> Result<BasicTokenResponse, Box<dyn std::error::Error>> {
        if state.secret() != csrf_state.secret() {
            return Err(
                "CSRF tokens do not match, an attacker may be attempting a CSRF attack".into(),
            );
        }

        let token_response = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(code_verifier)
            .request(http_client)?;

        *self.token_response.lock().unwrap() = Some(token_response.clone());
        Ok(token_response)
    }

    pub fn refresh_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token_response = match self.token_response.lock().unwrap().clone() {
            Some(token) => token,
            None => return Err("No token response".into()),
        };

        let refresh_token = token_response
            .refresh_token()
            .clone()
            .ok_or("No refresh token")?;

        let response = self
            .client
            .exchange_refresh_token(refresh_token)
            .request(http_client)?;
        *self.token_response.lock().unwrap() = Some(response);

        Ok(())
    }

    pub fn revoke_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token_response = match self.token_response.lock().unwrap().clone() {
            Some(token) => token,
            None => return Err("No token response".into()),
        };

        let token_to_revoke: StandardRevocableToken = match token_response.refresh_token() {
            Some(token) => token.into(),
            None => token_response.access_token().into(),
        };

        self.client
            .revoke_token(token_to_revoke)?
            .request(http_client)?;
        *self.token_response.lock().unwrap() = None;

        Ok(())
    }

    pub fn is_token_expired(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let token_response = match self.token_response.lock().unwrap().clone() {
            Some(token) => token,
            None => return Err("No token response".into()),
        };

        // Expiration date is in seconds
        let expires_at = token_response.expires_in().ok_or("No expiration date")?;
        let expires_at = expires_at.as_secs() as i64;
        let now = Utc::now().timestamp();

        println!("Token expires at: {}, now: {}", expires_at, now);

        Ok(expires_at < now)
    }
}

impl Clone for AuthFlow {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            token_response: Mutex::new(self.token_response.lock().unwrap().clone()),
        }
    }
}

pub type GetTokenOutput<'a> = Pin<
    Box<
        dyn Future<Output = Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>>
            + Send
            + 'a,
    >,
>;

impl GetToken for AuthFlow {
    fn get_token<'a>(&'a self, _scopes: &'a [&str]) -> GetTokenOutput<'a> {
        let future = async move {
            let token_response = match self.token_response.lock().unwrap().clone() {
                Some(token) => token,
                None => return Err("No token response".into()),
            };

            if self
                .is_token_expired()
                .map_err(|e| format!("Failed to check if token is expired: {}", e))?
            {
                self.refresh_token()
                    .map_err(|e| format!("Failed to refresh token: {}", e))?;
            }

            let access_token = token_response.access_token().secret().clone();
            Ok(Some(access_token))
        };

        Box::pin(future)
    }
}
