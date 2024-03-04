// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Arc;
use std::time::Duration;

use clokwerk::Interval;
use clokwerk::Scheduler;
use moka::sync::CacheBuilder;
use paperless_desktop::__cmd__documents_query;
use paperless_desktop::__cmd__latest_paperless_release;
use paperless_desktop::__cmd__latest_release;
use paperless_desktop::__cmd__get_favourites;
use paperless_desktop::__cmd__add_to_favourites;
use paperless_desktop::__cmd__remove_from_favourites;
use paperless_desktop::__cmd__document_thumbnail;
use paperless_desktop::documents::favourites::Favourites;
use paperless_desktop::documents::search::documents_query;
use paperless_desktop::data::system::latest_paperless_release;
use paperless_desktop::data::system::latest_release;
use paperless_desktop::documents::favourites::get_favourites;
use paperless_desktop::documents::favourites::add_to_favourites;
use paperless_desktop::documents::favourites::remove_from_favourites;
use paperless_desktop::documents::document_thumbnail;
use paperless_desktop::store::local::Storage;
use paperless_desktop::store::MemoryCache;
use paperless_rs::authorization::AuthorizationType;
use paperless_rs::authorization::CertificateType;
use paperless_rs::authorization::Credentials;
use paperless_rs::ternary;
use paperless_rs::PaperlessClient;
use std::path::PathBuf;
use tauri::http::ResponseBuilder;
use tauri::{async_runtime, Manager};
use tauri_plugin_store::StoreBuilder;

fn main() {
    // Template variables
    std::env::set_var("PAPERLESS_USERNAME", "boul2gom");
    std::env::set_var("PAPERLESS_PASSWORD", "GvcDacuTC@eor5#M");
    std::env::set_var("PAPERLESS_URL", "http://127.0.0.1:8000");

    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = devtools::init();

    let builder = tauri::Builder::default();

    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let builder = builder.plugin(devtools);
    
    builder.plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            documents_query,
            latest_release,
            latest_paperless_release,

            get_favourites,
            add_to_favourites,
            remove_from_favourites,

            document_thumbnail
        ])
        .manage(setup_paperless().expect("Could not setup Paperless"))
        .register_uri_scheme_protocol("paperless-desktop", |_, _| {
            let response = ResponseBuilder::new().status(200);

            let body = "You can now close this screen and go back to the app.".as_bytes();
            let response = response.body(Vec::from(body)).unwrap();
            Ok(response)
        })
        .setup(|app| {
            let mut scheduler = Scheduler::with_tz(chrono::Utc);
            let cache: MemoryCache = CacheBuilder::new(100_000).build();
            let cache = Arc::new(cache);

            let builder = reqwest::Client::builder();
            let builder = builder.user_agent("paperless-rs");
            let client = builder.build().expect("Could not create HTTP client");
            app.manage(client);

            let path = PathBuf::from("preferences.bin");
            let store = StoreBuilder::new(app.handle(), path.clone()).build();
            let storage = Arc::new(Storage::new(app.handle(), path, store).expect("Could not create storage"));

            Favourites::load(&cache, &storage);

            app.manage(cache.clone());
            app.manage(storage.clone());

            scheduler.every(Interval::Minutes(5)).run(move || {
                println!("Saving favourites and store on disk...");
                Favourites::save(&cache, &storage).expect("Could not save favourites");
                storage.save_store().expect("Could not save store");
                println!("Stores has been saved!");
            });

            let scheduler_handle = scheduler.watch_thread(Duration::from_secs(30));
            app.manage(scheduler_handle);

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

pub fn setup_paperless() -> Result<PaperlessClient, Box<dyn std::error::Error>> {
    async_runtime::block_on(async {
        let cred_username = self::get_env("PAPERLESS_USERNAME");
        let cred_password = self::get_env("PAPERLESS_PASSWORD");
        let auth_token = self::get_env("PAPERLESS_TOKEN");

        let auth_type = if auth_token.is_some() {
            AuthorizationType::Token(auth_token.unwrap())
        } else if cred_username.is_some() && cred_password.is_some() {
            AuthorizationType::Basic(Credentials::new(
                cred_username.unwrap(),
                cred_password.unwrap()
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

        // Ping http://127.0.0.1:8000/api/remote_version/ to check if Paperless is running
        paperless_client
    })
}