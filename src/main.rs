use rand::{Rng, thread_rng};

fn main() {
    let mut rng = thread_rng();
    let language = rng.gen_range(0..3);
    let paragraph = match language {
        0 => generate_paragraph("english"),
        1 => generate_paragraph("french"),
        2 => generate_paragraph("spanish"),
        _ => panic!("Invalid language index"),
    };
    println!("{}", paragraph);
}

fn generate_paragraph(language: &str) -> String {
    let mut rng = thread_rng();
    let word_list = match language {
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
    };
    let n_sentences = rng.gen_range(3..7);
    let mut sentences = Vec::new();
    let mut sentence_start = true;
    for _ in 0..n_sentences {
        let n_words = rng.gen_range(5..12);
        let mut words = Vec::new();
        let mut use_conjunction = !sentence_start;
        for _ in 0..n_words {
            let word_index = rng.gen_range(0..word_list.len());
            let word = word_list[word_index];
            if word == "," || word == ":" || word == ";" || word == "?" || word == "!" {
                use_conjunction = false;
            } else if use_conjunction {
                words.push(",");
                words.push(word);
                use_conjunction = false;
            } else {
                words.push(word);
                use_conjunction = true;
            }
        }
        sentence_start = false;
        let mut sentence = words.join(" ");
        sentence.make_ascii_lowercase();
        let first_char = sentence.chars().next().unwrap();
        sentence.replace_range(..1, &first_char.to_uppercase().to_string());
        if let Some(last_char) = sentence.chars().rev().next() {
            if last_char != '?' && last_char != '!' {
                sentence.push('.');
            }
        }
        sentences.push(sentence);
    }
    sentences.join(" ")
}
