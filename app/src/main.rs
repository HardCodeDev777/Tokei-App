#![windows_subsystem = "windows"]

slint::include_modules!();
use tokei::Config;
use slint::PlatformError;
use crate::code_counter::get_counted_code_data;

mod json_utils{
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::path::Path;
    use serde_json::to_string_pretty;
    use serde::Serialize;
    use crate::code_counter::LangStat;

    #[derive(Serialize)]
    struct AllLangsStat{
        langs_stat: Vec<LangStat>
    }

    impl AllLangsStat{
        fn new(langs_stat: Vec<LangStat>) -> Self{
            Self {langs_stat}
        }
    }

    pub fn write_to_json(path: &str, langs_stat: Vec<LangStat>) -> std::io::Result<()>{
        let all_langs_stat = AllLangsStat::new(langs_stat);
        let seriliazed_data = to_string_pretty(&all_langs_stat).unwrap();

        let mut file: File;

        if Path::new("data.json").exists(){
            file = OpenOptions::new().write(true).open(path)?;
        }
        else {
            file = File::create(path)?;
        }
        file.write_all(seriliazed_data.as_bytes())?;

        Ok(())
    }
}

mod code_counter{
    use slint::SharedString;
    use tokei::{Config, LanguageType, Languages};
    use std::path::Path;
    use serde::Serialize;
    use crate::json_utils::write_to_json;

    #[derive(Serialize)]
    pub struct LangStat{
        pub lang_type: LanguageType,
        pub lines_count: usize,
        pub code_count: usize,
        pub comments_count: usize
    }

    impl LangStat {
        pub fn new(lang_type: LanguageType, lines_count: usize, code_count: usize, comments_count: usize) -> Self{
            Self {lang_type, lines_count, code_count, comments_count}
        }
    }

    pub fn get_counted_code_data(config: &Config, string: SharedString) -> String{
        let mut lang = Languages::new();

        lang.get_statistics(&[Path::new(string.trim())], &[".git", "target"], config);

        let mut lines_text = String::new();
        let mut lang_stats_vector:Vec<LangStat> = Vec::<LangStat>::new();

        for (language_type, stats) in lang.iter(){
            if stats.lines() > 0 {   
                lines_text += &format!("Language: {}, lines: {}, code {} ,comments: {} \n", language_type, stats.lines(), 
                stats.code, stats.comments);

                let curr_lang_stat = LangStat::new(language_type.clone(), stats.lines(), 
                stats.code, stats.comments);
                lang_stats_vector.push(curr_lang_stat);
            }
        }

        match write_to_json("data.json", lang_stats_vector) {
            Ok(()) => println!("Saved to JSON successfully!"),
            Err(er) => println!("Error saving to JSON! {}", er)
        };

        lines_text
    }
}

fn main() -> Result<(), PlatformError> {
    let ui = MainWindow::new()?;
    
    let tokei_config = Config::default();

    let ui_handle = ui.as_weak();
    ui.on_count_lines(move |string| {
        let ui = ui_handle.unwrap();

        let mut result = String::new();

        ui.set_lang_text(result.clone().into());

        result = get_counted_code_data(&tokei_config, string);

        ui.set_lang_text(result.into());
    });

    ui.run()
}