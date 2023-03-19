use rand::{Rng, thread_rng};
use std::{fs, error::Error, io::{Write}, time::{SystemTime, UNIX_EPOCH}};

enum Corpus<'a> {
    FromFile(&'a str), 
}
fn main(){
    let mut rng = thread_rng();
    let language = rng.gen_range(0..8);
    generate_text_for_language(language, false);
}

fn generate_text_for_language(language: i32, write_to_file: bool) {
    let paragraph = match language {
        0 => generate_paragraph(Corpus::FromFile("english.txt"), Some(100), Some(1000)),
        1 => generate_paragraph(Corpus::FromFile("french.txt"), None, None),
        2 => generate_paragraph(Corpus::FromFile("spanish.txt"), None, Some(500)),
        3 => generate_paragraph(Corpus::FromFile("hindi.txt"), Some(50), None),
        4 => generate_paragraph(Corpus::FromFile("russian.txt"), Some(50), None),
        5 => generate_paragraph(Corpus::FromFile("arabic.txt"), Some(50), None),
        6 => generate_paragraph(Corpus::FromFile("japanese.txt"), Some(50), None),
        7 => generate_paragraph(Corpus::FromFile("german.txt"), Some(50), None),
        _ => panic!("Invalid language index"),
    };
    println!("{}", paragraph);
    
    if write_to_file == true{
        let res = write_paragraph_to_file(paragraph, None);
        match res {
            Ok(_) => println!("File created successfully."), 
            Err(e) => panic!("{}", e.to_string())
        };
    }
}

fn read_corpus_from_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Failed to parse file.")
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

fn write_paragraph_to_file(paragraph: String, path: Option<String>) -> Result<bool, Box<dyn Error>> {
    let file_name = match path {
        Some(x) => x, 
        None => {
            let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
            let path = format!("{}.txt", time.as_secs());
            path
        }
    };

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_name)?;
    file.write_all(paragraph.as_bytes())?;

    Ok(true)
}

fn generate_paragraph(corpus: Corpus, min_sentences: Option<usize>, max_bytes: Option<usize>) -> String {
    let mut rng = thread_rng();

    let word_list: Vec<String> = match corpus {
        Corpus::FromFile(f) => {
            read_corpus_from_file(f)
        }, 
    };

    let n_sentences;
    if let Some(n) = min_sentences {
        n_sentences = rng.gen_range(n..n+4);
    } else {
        n_sentences = rng.gen_range(3..7);
    }

    let mut sentences = Vec::new();
    for _ in 0..n_sentences {
        let n_words = rng.gen_range(5..12);
        let mut words = Vec::new();
        for _ in 0..n_words {
            let word_index = rng.gen_range(0..word_list.len());
            let word = word_list[word_index].as_str();
            words.push(word);
        }

        let mut sentence = words.join(" ");
        sentence.make_ascii_lowercase();

        let first_char = sentence.chars().next().unwrap();
        //first character may not be one byte-aligned
        let first_char_len = &first_char.len_utf8();
        sentence.replace_range(..first_char_len, &first_char.to_uppercase().to_string());
        sentence.push('.');
        sentences.push(sentence);
    }
    
    let paragraph = sentences.join(" ");
    
    match max_bytes {
        None => paragraph, 
        Some(m) => {
            //only read up to limit of bytes requested by the user
            let mut truncated = String::new();
            let mut num_bytes: usize = 0;
            for char in paragraph.chars() {
                let len = char.len_utf8();
                if num_bytes + len < m {
                    truncated.push(char);
                    num_bytes += len;
                } else {
                    break;
                }
            }

            truncated
        }
    }
}