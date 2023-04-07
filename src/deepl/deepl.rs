use serde_json::{Value};
use std::{fmt::Display, str::FromStr};

pub enum SourceLang {
    Bg, 
    Cs, 
    Da, 
    De, 
    El, 
    En, 
    Es, 
    Et, 
    Fi, 
    Fr, 
    Hu, 
    Id, 
    It, 
    Ja, 
    Ko, 
    Lt, 
    Lv, 
    Nb, 
    Nl, 
    Pl, 
    Pt, 
    Ro, 
    Ru, 
    Sk, 
    Sl, 
    Sv, 
    Tr, 
    Uk, 
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

pub enum TargetLang {
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnGb,
    EnUs,
    Es,
    Et,
    Fi,
    Fr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Nb,
    Nl,
    Pl,
    Pt,
    PtBr,
    PtPt,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Tr,
    Uk,
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

pub enum SplitSentences {
    None, 
    PunctuationAndNewline, 
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

pub enum Formality {
    Default, 
    More, 
    Less, 
    PreferMore, 
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

pub enum TagHandling {
    Xml, 
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

struct Glossary {
    glossary_id: String, 
    name: String, 
    ready: bool, 
    source_lang: SourceLang, 
    target_lang: TargetLang, 
    creation_time: String, 
    entry_count: u64
}

fn capitalize(s: &str) -> String {
    let mut vec : Vec<String> = Vec::new();

    //capitalize each character in given string
    for c in s.chars() {
        let upper = c.to_uppercase().collect::<String>();
        vec.push(upper);
    }

    vec.join("")
}

impl Glossary {
    fn new(v: Value) -> Glossary {
        let upper_source = capitalize(v["source_lang"].as_str().unwrap());
        let upper_target = capitalize(v["target_lang"].as_str().unwrap());

        Glossary {
            glossary_id: v["glossary_id"].to_string(),
            name: v["name"].to_string(),
            ready: v["ready"].as_bool().unwrap(),
            source_lang: SourceLang::from_str(&upper_source.as_str()).unwrap(),
            target_lang: TargetLang::from_str(&upper_target.as_str()).unwrap(),
            creation_time: v["creation_time"].to_string(),
            entry_count: v["entry_count"].as_u64().unwrap() 
        }
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