use kirjat::features;

#[cfg(feature = "tui")]
#[tokio::main]
async fn main() {
    #[cfg(feature = "tui")]
    features::tui::start_tui().await;
}

#[cfg(not(feature = "tui"))]
fn main() {
    panic!("The crate wasn't built with the \"tui\"-feature, so no ui is available.");
}
