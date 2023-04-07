use rand::{Rng, thread_rng};

#[path = "lib/text_generator.rs"]
mod text_generator;
use text_generator::generate_text_for_language;

#[path = "lib/my_memory.rs"]
mod my_memory;
use my_memory::translate_q_langpair;

#[path = "lib/dictionary.rs"]
mod dictionary;
use dictionary::get_meaning;


fn main(){
    // let mut rng = thread_rng();
    // let language = rng.gen_range(0..8);
    // generate_text_for_language(language, false);


    // let q = String::from("My name is Aaron"); 
    // let langpair = String::from("en|hi"); 
    // let translated_text = translate_q_langpair(q, langpair);
    // println!("{}", translated_text);

    deepl();
}


// commenting for now, is not a running code..
/*

fn deepl() {
    let secret = read_secret("secret.txt");
    let auth = DeepLKey{key: secret};

    //translation

    let text = ["Philosophy is based on a false pretense, so philosophy itself is nonsense."];

    let tr = TranslationRequest::new(Box::new(text), TargetLang::De);
    let request = TranslationRequest::create_request(&tr, &auth);

    let res = request.execute();
    match res {
        Ok(v) => println!("OK: {}", v), 
        Err(_) => println!("Flop")
    };

    //glossary fun

    // let glossary_request = get_glossaries(&auth);

    // if let Ok(glossaries) = glossary_request {
    //     for glossary in glossaries {
    //         println!("NEW GLOSSARY: \n{}\n", glossary);
    //     }
    // } else {
    //     println!("UH OH");
    // }

    // // get specific glossary
    // let gid = "36ee07b0-ddc9-426e-a273-57b1dba75291".to_string();
    // let glossary_request = get_glossary(&auth, gid);

    // if let Ok(gloss) = glossary_request {
    //     println!("\n\n\nGLOSS: {}", gloss);
    // } else {
    //     println!("Error getting glossary");
    // }

    // //delete specific glossary
    // let gid = "2d55f35e-2b81-47d9-a12b-89498e540874".to_string();
    // let delete_request = delete_glossary(&auth, gid);
    
    // if let Ok(()) = delete_request {
    //     println!("YIPEEEE!");
    // } else {
    //     println!("Failed to delete glossary");
    // }

    //create a glossary
    // let glossary = String::from("Hello\tGabagool\nBye\tVa Fangul");
    // let create_request = create_glossary_from_string(&auth, "Nonsense".to_string(), SourceLang::En, TargetLang::It, glossary);

    // if let Ok(g) = create_request {
    //     println!("glossary_id: {}", g["glossary_id"]);
    //     println!("ready: {}", g["ready"]);
    //     println!("name: {}", g["name"]);
    //     println!("source_lang: {}", g["source_lang"]);
    //     println!("target_lang: {}", g["target_lang"]);
    //     println!("creation_time: {}", g["creation_time"]);
    //     println!("entry_count: {}", g["entry_count"]);
    // } else {
    //     println!("Failed to create glossary");
    // }
}

 */