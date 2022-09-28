use kirjat::features;

fn main() {
    #[cfg(feature = "tui")]
    features::tui::start_tui();
    #[cfg(not(feature = "tui"))]
    println!("The crate wasn't built with the \"tui\"-option, so no ui is available.");
}
