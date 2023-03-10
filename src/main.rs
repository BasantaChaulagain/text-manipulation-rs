use rand::{Rng, thread_rng};
use std::fs;

enum Corpus<'a> {
    FromFile(&'a str), 
    FromLanguage(&'a str)
}

fn main() {
    let mut rng = thread_rng();
    let language = rng.gen_range(0..4);
    let paragraph = match language {
        0 => generate_paragraph(Corpus::FromLanguage("english"), Some(100), Some(1000)),
        1 => generate_paragraph(Corpus::FromLanguage("french"), None, None),
        2 => generate_paragraph(Corpus::FromLanguage("spanish"), None, Some(500)),
        3 => generate_paragraph(Corpus::FromFile("sample.txt"), Some(50), None), 
        _ => panic!("Invalid language index"),
    };
    println!("{}", paragraph);
}

fn read_corpus_from_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Failed to parse file.")
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

fn get_corpus(language: &str) -> Vec<&str> {
    match language {
        "english" => vec![
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
            "a", "man", "walks", "into", "a", "bar", "the", "bartender", "says", "what'll", "it", "be",
            "to", "be", "or", "not", "to", "be", "that", "is", "the", "question",
            "there", "are", "many", "variations", "of", "passages", "of", "Lorem", "Ipsum", "available",
            "but", "the", "majority", "have", "suffered", "alteration", "in", "some", "form", "or", "another",
        ],
        "french" => vec![
            "le", "chat", "est", "sur", "le", "toit",
            "je", "suis", "fatigué", "aujourd'hui",
            "une", "fois", "n'est", "pas", "coutume",
            "la", "vie", "est", "belle", "et", "pleine", "de", "surprises",
            "il", "y", "a", "toujours", "quelque", "chose", "à", "apprendre",
        ],
        "spanish" => vec![
            "el", "gato", "está", "en", "el", "tejado",
            "hoy", "es", "un", "día", "soleado",
            "a", "veces", "la", "vida", "es", "difícil",
            "no", "hay", "mal", "que", "por", "bien", "no", "venga",
            "lo", "importante", "es", "mantenerse", "siempre", "positivo",
        ],
        _ => panic!("Invalid language"),
    }
}

fn generate_paragraph(corpus: Corpus, min_sentences: Option<usize>, max_bytes: Option<usize>) -> String {
    let mut rng = thread_rng();

    let word_list: Vec<String> = match corpus {
        Corpus::FromFile(f) => {
            read_corpus_from_file(f)
        }, 
        Corpus::FromLanguage(l) => {
            get_corpus(l)
                .iter()
                .map(|s| s.to_string())
                .collect()
        }
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
    // sentences.push("\n".to_string());
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
