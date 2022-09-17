use dialoguer::Input;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use crate::search_book_from_all_sources;
use console::Term;

pub fn start_tui() {
    let term = Term::stdout();
    term.set_title("Kirjat-rs");
    term.clear_screen().unwrap();
    println!("Kirjoita \"q\" poistuaksesi.");
    let input: String = Input::new()
        .with_prompt("Kirjan nimi")
        .interact_text().unwrap();
    if input == "q" {
        term.clear_screen().unwrap();
        term.write_line("Käyttäjä poistui.").unwrap();
        return;
    }
    term.clear_screen().unwrap();
    term.write_line("Haetaan...").unwrap();

    let results = search_book_from_all_sources(&input, &None).unwrap();

    let selectable = results.iter().map(|x| x.name.clone()).collect::<Vec<String>>();
    term.clear_screen().unwrap();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&selectable)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap();
    match selection {
        Some(index) => {
            let selected_item = &results[index];
            println!("Kirja {}", selected_item.id);
            println!("- nimi: {}", selected_item.name);
            println!("- linkki: {}", selected_item.links.buy);
            println!("- Vaihtoehdot:");
            for condition in &selected_item.conditions {
                println!("\t{}: {}", condition.name, condition.price);
            }
        },
        None => {
            term.write_line("Yhtäkään kirjaa ei valittu.").unwrap();
        },
    }
    
}