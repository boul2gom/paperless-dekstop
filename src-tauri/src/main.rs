// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use paperless_rs::authorization::AuthorizationType;
use paperless_rs::authorization::CertificateType;
use paperless_rs::authorization::Credentials;
use paperless_rs::ternary;
use paperless_rs::PaperlessClient;
use tauri::{async_runtime, Manager};
use tauri_plugin_store::StoreBuilder;

use paperless_desktop::__cmd__documents_query;
use paperless_desktop::__cmd__fetch_latest_paperless_release;
use paperless_desktop::__cmd__fetch_latest_release;

use paperless_desktop::data::documents::documents_query;
use paperless_desktop::data::system::fetch_latest_paperless_release;
use paperless_desktop::data::system::fetch_latest_release;
use paperless_desktop::store::Storage;

fn main() {
    // Template variables
    std::env::set_var("PAPERLESS_USERNAME", "admin");
    std::env::set_var("PAPERLESS_PASSWORD", "admin");
    std::env::set_var("PAPERLESS_URL", "https://paperless.example.com");

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            documents_query,
            fetch_latest_release,
            fetch_latest_paperless_release
        ])
        .manage(setup_http())
        .manage(setup_paperless())
        .setup(|app| {
            let store = StoreBuilder::new(app.handle(), "preferences.bin".parse()?).build();
            let storage = Storage::new(store);
            app.manage(storage);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn get_env(name: &str) -> Option<String> {
    match std::env::var(name) {
        Ok(val) => ternary!(val.is_empty(), None, Some(val)),
        Err(_) => None,
    }
}

pub fn setup_http() -> reqwest::Client {
    let builder = reqwest::Client::builder();
    let builder = builder.user_agent("paperless-rs");
    let client = builder.build();

    if client.is_err() {
        panic!("Could not create HTTP client! Please check running environment.");
    }

    client.unwrap()
}

pub fn setup_paperless() -> PaperlessClient {
    async_runtime::block_on(async {
        let cred_username = self::get_env("PAPERLESS_USERNAME");
        let cred_password = self::get_env("PAPERLESS_PASSWORD");
        let auth_token = self::get_env("PAPERLESS_TOKEN");

        let auth_type = if auth_token.is_some() {
            AuthorizationType::Token(auth_token.unwrap())
        } else if cred_username.is_some() && cred_password.is_some() {
            AuthorizationType::Basic(Credentials::new(
                cred_username.unwrap(),
                cred_password.unwrap(),
                None,
            ))
        } else {
            panic!("No credentials or token provided! Please check running environment.");
        };

        let pem_path = self::get_env("PAPERLESS_PEM_PATH");
        let der_path = self::get_env("PAPERLESS_DER_PATH");
        let pem_cert = ternary!(
            pem_path.is_some(),
            Some(CertificateType::Pem(pem_path.unwrap())),
            None
        );
        let der_cert = ternary!(
            der_path.is_some(),
            Some(CertificateType::Der(der_path.unwrap())),
            None
        );

        let cert_type = ternary!(
            pem_cert.is_some(),
            pem_cert,
            ternary!(der_cert.is_some(), der_cert, None)
        );

        let paperless_url = self::get_env("PAPERLESS_URL")
            .expect("No Paperless URL provided! Please check running environment.");
        let paperless_client =
            PaperlessClient::new(paperless_url.as_str(), auth_type, cert_type).await;

        if paperless_client.is_err() {
            panic!("Could not connect to Paperless! Please check running environment.");
        }

        paperless_client.unwrap()
    })
}
