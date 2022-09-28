use crate::{search_book, search_book_from_all_sources};
use anyhow::Context;
use console::Term;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn start_tui() {
    let term = Term::stdout();
    term.set_title("Kirjat-rs");
    term.clear_screen().unwrap();
    term.write_line("Valitse lähde").unwrap();
    let mut available_sources = vec!["Hae kaikista lähteistä samanaikaisesti".to_string()];
    available_sources.append(
        &mut crate::sources::AVAILABLE_SOURCES
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>(),
    );
    let source_index = Select::with_theme(&ColorfulTheme::default())
        .items(&available_sources)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();
    term.clear_screen().unwrap();
    let input: String = Input::new()
        .with_prompt("Kirjan nimi")
        .interact_text()
        .unwrap();
    term.clear_screen().unwrap();
    term.write_line("Haetaan...").unwrap();

    let results: Vec<crate::structs::kirja::Kirja>;
    if source_index == 0 {
        results = search_book_from_all_sources(&input, &None)
            .context("Kirjojen haku epäonnistui")
            .unwrap();
    } else {
        let actual_index = source_index - 1; // Substract one as we added an option
        results = search_book(
            &input,
            crate::sources::AVAILABLE_SOURCES[actual_index],
            &None,
        )
        .context("Kirjojen haku epäonnistui")
        .unwrap();
    }

    if results.len() == 0 {
        term.write_line("Hakusanalla ei löytynyt kirjoja").unwrap();
        return;
    }

    let selectable = results
        .iter()
        .map(|x| format!("{}\t{}\t\t{}", x.get_min_price().unwrap(), x.source, x.name))
        .collect::<Vec<String>>();
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
        }
        None => {
            term.write_line("Yhtäkään kirjaa ei valittu.").unwrap();
        }
    }
}
