use rand::{Rng, thread_rng};

#[path = "lib/text_generator.rs"]
mod text_generator;
use text_generator::generate_text_for_language;

#[path = "lib/my_memory.rs"]
mod my_memory;
use my_memory::translate_q_langpair;

fn main(){
    // code to test random text generator code.
    let mut rng = thread_rng();
    let language = rng.gen_range(0..8);
    generate_text_for_language(language, false);

    // code to test sentence translation code.
    let q = String::from("My name is Aaron"); 
    let langpair = String::from("en|hi"); 
    let translated_text = translate_q_langpair(q, langpair);
    println!("{}", translated_text);

    // code to test word definition code.
    
}

