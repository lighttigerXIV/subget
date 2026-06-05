use std::error::Error;

use dialoguer::{FuzzySelect, Input, MultiSelect, Select, console::Style, theme::ColorfulTheme};

use crate::{
    api::{Subtitle, download_subtitle, fetch_subtitles},
    config::{Config, get_config},
};

mod api;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = get_config()?;

    let mut theme = ColorfulTheme::default();
    theme.active_item_style = Style::new().yellow();
    theme.active_item_prefix = Style::new().yellow().apply_to(">".to_string());

    let mut save_config = false;

    let key = match config.api_key.clone().is_empty() {
        true => {
            let key = dialoguer::Input::with_theme(&theme)
            .with_prompt(
                "Insert your API Key. You can get one at https://www.opensubtitles.com/consumers\n",
            )
            .interact_text()?;

            save_config = true;

            key
        }
        false => config.api_key.clone(),
    };

    let langs = match config.languages.clone().is_empty() {
        true => {
            let selected_langs = MultiSelect::with_theme(&theme)
                .with_prompt("Pick an option")
                .items(&vec!["English", "Portuguese", "Spanish", "French"])
                .defaults(&[true, false, false, false])
                .interact()?;

            let use_english = selected_langs.contains(&0);
            let use_portuguese = selected_langs.contains(&1);
            let use_spanish = selected_langs.contains(&2);
            let use_french = selected_langs.contains(&3);

            let mut langs = Vec::<String>::new();

            if use_english {
                langs.push("en".to_string());
            }

            if use_portuguese {
                langs.push("pt-PT".to_string());
                langs.push("pt-BR".to_string());
            }

            if use_spanish {
                langs.push("es".to_string());
            }

            if use_french {
                langs.push("fr".to_string());
            }

            save_config = true;

            langs
        }
        false => config.languages.clone(),
    };

    if save_config {
        config = Config {
            api_key: key.clone(),
            languages: langs.clone(),
            download_dir: config.download_dir.clone(),
        };

        config::save_config(config.clone())?;
    }

    let search_type = Select::with_theme(&theme)
        .with_prompt("What type of search you would like to do?")
        .items(&["Movie", "Show"])
        .default(0)
        .interact()?;

    let search_type = if search_type == 0 {
        "movie"
    } else if search_type == 1 {
        "episode"
    } else {
        return Err("".into());
    };

    let subs: Vec<Subtitle> = if search_type == "episode" {
        let show_name: String = Input::with_theme(&theme)
            .with_prompt("Type the Show Name\n")
            .interact()?;

        let season: u8 = Input::with_theme(&theme)
            .with_prompt("Type the Season Number\n")
            .interact()?;

        let episode: u16 = Input::with_theme(&theme)
            .with_prompt("Type the Episode Number\n")
            .interact()?;

        let formatted_season = format!("{:02}", season);
        let formatted_episode = format!("{:02}", episode);

        fetch_subtitles(
            &config,
            &format!("{show_name} S{formatted_season}E{formatted_episode}"),
            &search_type,
        )
        .await?
    } else {
        let movie_name: String = Input::with_theme(&theme)
            .with_prompt("Type the Movie Name\n")
            .interact()?;

        fetch_subtitles(&config, &movie_name, &search_type).await?
    };

    if subs.is_empty() {
        println!("⚠️ Subtitles not found");
        return Ok(());
    }

    let on_windows = cfg!(target_os = "windows");

    let options: Vec<String> = subs
        .iter()
        .clone()
        .into_iter()
        .map(|subtitle| {
            format!(
                "{} {} - [{} Downloads]",
                match subtitle.language.as_str() {
                    "pt-BR" =>
                        if on_windows {
                            "[PT-BR]"
                        } else {
                            "🇧🇷"
                        },
                    "pt-PT" =>
                        if on_windows {
                            "[PT]"
                        } else {
                            "🇵🇹"
                        },
                    "en" =>
                        if on_windows {
                            "[EN]"
                        } else {
                            "🇬🇧"
                        },
                    "fr" =>
                        if on_windows {
                            "[FR]"
                        } else {
                            "🇫🇷"
                        },
                    _ => "",
                },
                subtitle.name,
                subtitle.download_count
            )
        })
        .collect();

    let sub_index = FuzzySelect::with_theme(&theme)
        .with_prompt("Select a Subtitle")
        .items(&options)
        .default(0)
        .interact()?;

    let subtitle = subs.get(sub_index).ok_or("Failed to get subtitle")?;

    let download_path = download_subtitle(&subtitle, &config).await?;

    println!("Subtitle available at: {}", &download_path.display());

    Ok(())
}
