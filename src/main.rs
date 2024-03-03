use structopt::StructOpt;
use regex::Regex;
use std::{fs::File, io::{BufRead, BufReader, Write}, sync::{Arc, Mutex}, thread};

mod logger;

#[derive(Debug, StructOpt)]
#[structopt(name = "Email extractor", about = "Извлекает email из списка ссылок")]
struct OptArgs {
    #[structopt(short, long, help = "Извлекает из почту из каждого сайта в списке")]
    list: Option<String>,

    #[structopt(short, long, help = "Записывает отчет о работе в выбранный файл")]
    output: Option<String>,

    #[structopt(short, long, help = "Показывает справку по командам")]
    help: bool
}

fn main() {
    let opt = OptArgs::from_args();

    if opt.help {
        OptArgs::clap().print_help().expect("Не удалось вывести справку по командам.");
    } else if let Some(filename) = opt.list {
        extract_email(&filename);
    } else {
        OptArgs::clap().print_help().expect("Не удалось вывести справку по командам.");
    }
}

fn extract_email(file: &str) {
    let reader = BufReader::new(File::open(file).expect("Ошибка открытие файла"));
    let email_regex = Regex::new(r#"\b[a-zA-Z0-9.-]+@[a-zA-Z0-9.-]+\.[a-zA-Z0-9.-]+\b"#).expect("Не удалось создать RegEx");
    let result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for line in reader.lines() {
        let result_clone = Arc::clone(&result);
        let email_regex_clone = email_regex.clone();
        let url = line.unwrap();

        let handle = thread::spawn(move || {
            let mut curl = curl::easy::Easy::new();
            curl.url(&url).unwrap();

            let data: Vec<u8> = Vec::new();
            let data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(data));
            let data_clone = Arc::clone(&data);

            curl.write_function(move |new_data| {
                let mut data_clone = data_clone.lock().unwrap();
                data_clone.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            curl.perform().unwrap();

            let data = data.lock().unwrap();
            let page = String::from_utf8_lossy(&data);

            let mut result_clone = result_clone.lock().unwrap();

            for email in email_regex_clone.find_iter(&page) {
                if result_clone.contains(&email.as_str().to_string()) {
                    continue;
                }

                let email_str = email.as_str().to_string();
                logger::info(&format!("Найдена почта: {}", email_str));
                if !result_clone.contains(&email_str) {
                    result_clone.push(email_str);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_result = result.lock().unwrap();

    if let Some(output_file) = OptArgs::from_args().output {
        let mut result_file = File::create(&output_file).unwrap();

        for email in  final_result.iter() {
            let _ = result_file.write(format!("{}\n", email).as_bytes());
        }
        logger::success(&format!("Работа закончена, результат успешно записан в файл {}.", output_file));
    } else {
        logger::success("Работа закончена, запись в файл не требуется.");
    }
}
