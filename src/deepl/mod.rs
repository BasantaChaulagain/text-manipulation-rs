//! This module gives general DeepL API access and translation information needed for translation requests.

use serde_json::{Value};
use std::{fmt::Display, str::FromStr};
use std::fs;

fn read_secret(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// All DeepL requests need a valid API key to function.
pub struct DeepLKey {
    /// The user's DeepL API key
    pub key: String, 
}

impl<'a> DeepLKey {
    /// This function reads the user's API key from a specified text file.  
    /// If it isn't found, it will return an error.
    pub fn new(path: &'a str) -> Result<DeepLKey, std::io::Error> {
        let res = read_secret(path);

        match res {
            Ok(key) => Ok(DeepLKey { key }), 
            Err(e) => Err(e)
        }
    }
}

/// For any translation request, these are the possible source languages the user can opt to specify.  
/// When using a glossary in a translation request, SourceLang is required.
pub enum SourceLang {
    ///Bulgarian
    Bg, 

    ///Czech
    Cs, 

    ///Danish
    Da, 

    ///German
    De, 

    ///Greek
    El, 

    ///English
    En, 
    
    ///Spanish 
    Es,
    
    /// Estonian
    Et,
    
    /// Finnish
    Fi,
    
    /// French
    Fr,
    
    /// Hungarian
    Hu,
    
    /// Indonesian
    Id,
    
    /// Italian
    It,
    
    /// Japanese
    Ja,
    
    /// Korean
    Ko,
    
    /// Lithuanian
    Lt,
    
    /// Latvian
    Lv,

    /// Norwegian (Bokmål)
    Nb,
    
    /// Dutch
    Nl,
    
    /// Polish
    Pl,
    
    /// Portuguese
    Pt,
    
    /// Romanian
    Ro,
    
    /// Russian
    Ru,
    
    /// Slovak
    Sk,
    
    /// Slovenian
    Sl,
    
    /// Swedish
    Sv,
    
    /// Turkish
    Tr,
    
    /// Ukrainian
    Uk,
    
    /// Chinese
    Zh,
}

impl Display for SourceLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceLang::Bg => f.write_str("BG"),  
            SourceLang::Cs => f.write_str("CS"),  
            SourceLang::Da => f.write_str("DA"),  
            SourceLang::De => f.write_str("DE"),  
            SourceLang::El => f.write_str("EL"),  
            SourceLang::En => f.write_str("EN"),  
            SourceLang::Es => f.write_str("ES"),  
            SourceLang::Et => f.write_str("ET"),  
            SourceLang::Fi => f.write_str("FI"),  
            SourceLang::Fr => f.write_str("FR"),  
            SourceLang::Hu => f.write_str("HU"),  
            SourceLang::Id => f.write_str("ID"),  
            SourceLang::It => f.write_str("IT"),  
            SourceLang::Ja => f.write_str("JA"),  
            SourceLang::Ko => f.write_str("KO"),  
            SourceLang::Lt => f.write_str("LT"),  
            SourceLang::Lv => f.write_str("LV"),  
            SourceLang::Nb => f.write_str("NB"),  
            SourceLang::Nl => f.write_str("NL"),  
            SourceLang::Pl => f.write_str("PL"),  
            SourceLang::Pt => f.write_str("PT"),  
            SourceLang::Ro => f.write_str("RO"),  
            SourceLang::Ru => f.write_str("RU"),  
            SourceLang::Sk => f.write_str("SK"),  
            SourceLang::Sl => f.write_str("SL"),  
            SourceLang::Sv => f.write_str("SV"),  
            SourceLang::Tr => f.write_str("TR"),  
            SourceLang::Uk => f.write_str("UK"),  
            SourceLang::Zh => f.write_str("ZH"), 
        }
    }
}

impl FromStr for SourceLang {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BG" => Ok(SourceLang::Bg),
            "CS" => Ok(SourceLang::Cs),
            "DA" => Ok(SourceLang::Da),
            "DE" => Ok(SourceLang::De),
            "EL" => Ok(SourceLang::El),
            "EN" => Ok(SourceLang::En),
            "ES" => Ok(SourceLang::Es),
            "ET" => Ok(SourceLang::Et),
            "FI" => Ok(SourceLang::Fi),
            "FR" => Ok(SourceLang::Fr),
            "HU" => Ok(SourceLang::Hu),
            "ID" => Ok(SourceLang::Id),
            "IT" => Ok(SourceLang::It),
            "JA" => Ok(SourceLang::Ja),
            "KO" => Ok(SourceLang::Ko),
            "LT" => Ok(SourceLang::Lt),
            "LV" => Ok(SourceLang::Lv),
            "NB" => Ok(SourceLang::Nb),
            "NL" => Ok(SourceLang::Nl),
            "PL" => Ok(SourceLang::Pl),
            "PT" => Ok(SourceLang::Pt),
            "RO" => Ok(SourceLang::Ro),
            "RU" => Ok(SourceLang::Ru),
            "SK" => Ok(SourceLang::Sk),
            "SL" => Ok(SourceLang::Sl),
            "SV" => Ok(SourceLang::Sv),
            "TR" => Ok(SourceLang::Tr),
            "UK" => Ok(SourceLang::Uk),
            "ZH" => Ok(SourceLang::Zh), 
            _ => Err(())
        }
    }
}

/// For each translation/glossary request, the target language is always required.  
/// This is the language that the desired text should be translated to/the target language for the glossary definitions.
pub enum TargetLang {
    ///Bulgarian
    Bg, 

    ///Czech
    Cs, 

    ///Danish
    Da, 

    ///German
    De, 

    ///Greek
    El, 

    ///English (please use EN-GB or EN-US instead)
    En, 

    ///British English
    EnGb,

    ///American English
    EnUs,

    ///Spanish 
    Es,
    
    /// Estonian
    Et,
    
    /// Finnish
    Fi,
    
    /// French
    Fr,
    
    /// Hungarian
    Hu,
    
    /// Indonesian
    Id,
    
    /// Italian
    It,
    
    /// Japanese
    Ja,
    
    /// Korean
    Ko,
    
    /// Lithuanian
    Lt,
    
    /// Latvian
    Lv,

    /// Norwegian (Bokmål)
    Nb,
    
    /// Dutch
    Nl,
    
    /// Polish
    Pl,
    
    /// Portuguese (please use PT-BR or PT-PT instead)
    Pt,

    ///Brazilian Portuguese
    PtBr,

    ///Portuguese (all Portuguese varieties excluding Brazilian Portuguese)
    PtPt,
    
    /// Romanian
    Ro,
    
    /// Russian
    Ru,
    
    /// Slovak
    Sk,
    
    /// Slovenian
    Sl,
    
    /// Swedish
    Sv,
    
    /// Turkish
    Tr,
    
    /// Ukrainian
    Uk,
    
    /// Chinese
    Zh,
}

impl Display for TargetLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetLang::Bg => f.write_str("BG"), 
            TargetLang::Cs => f.write_str("CS"), 
            TargetLang::Da => f.write_str("DA"), 
            TargetLang::De => f.write_str("DE"), 
            TargetLang::El => f.write_str("EL"), 
            TargetLang::En => f.write_str("EN"), 
            TargetLang::EnGb => f.write_str("EN-GB"), 
            TargetLang::EnUs => f.write_str("EN-US"), 
            TargetLang::Es => f.write_str("ES"), 
            TargetLang::Et => f.write_str("ET"), 
            TargetLang::Fi => f.write_str("FI"), 
            TargetLang::Fr => f.write_str("FR"), 
            TargetLang::Hu => f.write_str("HU"), 
            TargetLang::Id => f.write_str("ID"), 
            TargetLang::It => f.write_str("IT"), 
            TargetLang::Ja => f.write_str("JA"), 
            TargetLang::Ko => f.write_str("KO"), 
            TargetLang::Lt => f.write_str("LT"), 
            TargetLang::Lv => f.write_str("LV"), 
            TargetLang::Nb => f.write_str("NB"), 
            TargetLang::Nl => f.write_str("NL"), 
            TargetLang::Pl => f.write_str("PL"), 
            TargetLang::Pt => f.write_str("PT"), 
            TargetLang::PtBr => f.write_str("PT-BR"), 
            TargetLang::PtPt => f.write_str("PT-PT"), 
            TargetLang::Ro => f.write_str("RO"), 
            TargetLang::Ru => f.write_str("RU"), 
            TargetLang::Sk => f.write_str("SK"), 
            TargetLang::Sl => f.write_str("SL"), 
            TargetLang::Sv => f.write_str("SV"), 
            TargetLang::Tr => f.write_str("TR"), 
            TargetLang::Uk => f.write_str("UK"), 
            TargetLang::Zh => f.write_str("ZH"),  
        }
    }
}

impl FromStr for TargetLang {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BG" => Ok(TargetLang::Bg),
            "CS" => Ok(TargetLang::Cs),
            "DA" => Ok(TargetLang::Da),
            "DE" => Ok(TargetLang::De),
            "EL" => Ok(TargetLang::El),
            "EN" => Ok(TargetLang::En),
            "EN-GB" => Ok(TargetLang::EnGb), 
            "EN-US" => Ok(TargetLang::EnUs), 
            "ES" => Ok(TargetLang::Es),
            "ET" => Ok(TargetLang::Et),
            "FI" => Ok(TargetLang::Fi),
            "FR" => Ok(TargetLang::Fr),
            "HU" => Ok(TargetLang::Hu),
            "ID" => Ok(TargetLang::Id),
            "IT" => Ok(TargetLang::It),
            "JA" => Ok(TargetLang::Ja),
            "KO" => Ok(TargetLang::Ko),
            "LT" => Ok(TargetLang::Lt),
            "LV" => Ok(TargetLang::Lv),
            "NB" => Ok(TargetLang::Nb),
            "NL" => Ok(TargetLang::Nl),
            "PL" => Ok(TargetLang::Pl),
            "PT" => Ok(TargetLang::Pt),
            "PT-BR" => Ok(TargetLang::PtBr),
            "PT-PT" => Ok(TargetLang::PtPt),
            "RO" => Ok(TargetLang::Ro),
            "RU" => Ok(TargetLang::Ru),
            "SK" => Ok(TargetLang::Sk),
            "SL" => Ok(TargetLang::Sl),
            "SV" => Ok(TargetLang::Sv),
            "TR" => Ok(TargetLang::Tr),
            "UK" => Ok(TargetLang::Uk),
            "ZH" => Ok(TargetLang::Zh), 
            _ => Err(())
        }
    }
}

/// Dictates whether/where sentences in a translation request should be split.
/// 
/// For text translations where tag_handling is not set to html, the default value is 1, meaning the engine splits on punctuation and on newlines.
///
/// For text translations where tag_handling=html, the default value is nonewlines, meaning the engine splits on punctuation only, ignoring newlines.
pub enum SplitSentences {
    /// Do not split sentences
    None, 
    ///Split on all punctuation and newlines
    PunctuationAndNewline, 
    ///Ignores newline
    Punctuation, 
}

impl Display for SplitSentences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SplitSentences::None => f.write_str("0"), 
            SplitSentences::PunctuationAndNewline => f.write_str("1"), 
            SplitSentences::Punctuation => f.write_str("nonewlines"), 
        }
    }
}

/// Dictates the formality of the output translated text.
pub enum Formality {
    /// Default formality
    Default, 

    /// Use more formal language
    More, 

    /// Use less formal language
    Less, 

    /// Use more formal language if possible and use default formality otherwise
    PreferMore, 

    /// Use less formal language if possible and use default formality otherwise
    PreferLess
}

impl Display for Formality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formality::Default => f.write_str("default"), 
            Formality::More => f.write_str("more"), 
            Formality::Less => f.write_str("less"), 
            Formality::PreferMore => f.write_str("prefer_more"), 
            Formality::PreferLess => f.write_str("prefer_less"), 
        }
    }
}

/// Determines which types of tags should be handled in the request.
pub enum TagHandling {
    ///Handle XML tags
    Xml, 

    ///Handle HTML tags
    Html
}

impl Display for TagHandling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagHandling::Html => f.write_str("html"), 
            TagHandling::Xml => f.write_str("xml"), 
        }
    }
}

/// A Glossary maps words from a source language to desired translations in a target language.
/// 
/// For example, an English to German glossary "Hello\tHallo" would translate 
/// "Hello" to "Hallo" over other possible greetings.
/// 
/// This information is returned when creating/querying a glossary.  You can request 
/// glossary entries and other actions through the [glossary_request](super::request::glossary_request) module.
pub struct Glossary {
    /// Generated ID for this glossary
    pub glossary_id: String, 

    /// Name of the Glossary
    pub name: String, 

    ///Whether or not the Glossary can be used
    pub ready: bool, 

    ///Source Language of Glossary definitions
    pub source_lang: SourceLang, 

    ///Target Language of Glossary definitions
    pub target_lang: TargetLang, 

    ///When the Glossary was created
    pub creation_time: String, 

    ///Number of entries defined in the Glossary
    pub entry_count: u64
}

fn capitalize(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut vec : Vec<String> = Vec::new();

    //capitalize each character in given string
    for c in s.chars() {
        let upper = c.to_uppercase().collect::<String>();
        vec.push(upper);
    }

    Ok(vec.join(""))
}

#[derive(Debug)]
struct GlossaryError;

impl Display for GlossaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid Glossary")
    }
}

impl std::error::Error for GlossaryError {}

impl Glossary {
    /// Given a Json object, this method constructs a Glossary object or a dummy Glossary otherwise.
    pub fn new(v: Value) -> Result<Glossary, Box<dyn std::error::Error>> {
        let us = v["source_lang"].as_str().ok_or(GlossaryError)?;
        let upper_source = capitalize(us)?;
        let ut = v["target_lang"].as_str().ok_or(GlossaryError)?;
        let upper_target = capitalize(ut)?;

        let gid = v["glossary_id"].to_string();

        Ok(Glossary {
            glossary_id: gid[1..gid.len()-1].to_string(),
            name: v["name"].to_string(),
            ready: v["ready"].as_bool().unwrap(),
            source_lang: SourceLang::from_str(&upper_source.as_str()).unwrap(),
            target_lang: TargetLang::from_str(&upper_target.as_str()).unwrap(),
            creation_time: v["creation_time"].to_string(),
            entry_count: v["entry_count"].as_u64().unwrap() 
        })
    }
}

impl Display for Glossary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("ID: {}\nName: {}\nReady: {}\nSourceLang: {}\nTargetLang: {}\n Creation time: {}\nEntry count: {}", &self.glossary_id, &self.name, &self.ready, &self.source_lang, &self.target_lang, &self.creation_time, &self.entry_count);
        let res = f.write_str(str.as_str());

        if let Ok(_) = res {
            Ok(())
        } else {
            Err(std::fmt::Error)
        }
    }
}