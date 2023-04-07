//use crate::deepl::deepl::
use crate::deepl::{Formality, SplitSentences, SourceLang, TargetLang, TagHandling};
use crate::DeepLKey;
use crate::request::http_request::{HttpRequest, RequestType};

pub struct TranslationRequest<'a> {
    text: Box<[&'a str]>, 
    source_lang: Option<SourceLang>, 
    target_lang: TargetLang, 
    split_sentences: Option<SplitSentences>, 
    preserve_formatting: Option<bool>, 
    formality: Option<Formality>, 
    glossary_id: Option<&'a str>, 
    tag_handling: Option<TagHandling>, 
    non_splitting_tags: Option<&'a str>, 
    outline_detection: Option<bool>, 
    splitting_tags: Option<&'a str>, 
    ignore_tags: Option<&'a str>, 
}

impl<'a> TranslationRequest<'a> {
    pub fn new(text: Box<[&'a str]>, target_lang: TargetLang) -> TranslationRequest {
        TranslationRequest {
            text: text, 
            source_lang: None, 
            target_lang: target_lang, 
            split_sentences: None, 
            preserve_formatting: Some(false), 
            formality: Some(Formality::Default), 
            glossary_id: None, 
            tag_handling: None, 
            non_splitting_tags: None, 
            outline_detection: None, 
            splitting_tags: None, 
            ignore_tags: None 
        }
    }

    pub fn create_request(&self, auth : &'a DeepLKey) -> HttpRequest {
        let mut par : Vec<String> = Vec::new();

        let text_box = &*self.text;
        let text_format = format!("text={}", text_box[0]);
        par.push(text_format);

        if let Some(sl) = &self.source_lang {
            let source_lang_format = format!("source_lang={}", sl);
            par.push(source_lang_format);
        }

        let target_lang_format = format!("target_lang={}", &self.target_lang);
        par.push(target_lang_format);

        if let Some(split_sen) = &self.split_sentences {
            let split_sen_format = format!("split_sentences={}", split_sen);
            par.push(split_sen_format);
        }

        if let Some(pf) = &self.preserve_formatting {
            let pf_str = match pf {
                false => "0", 
                true => "1"
            };
            let pf_format = format!("preserve_formatting={}", pf_str);
            par.push(pf_format);
        }

        if let Some(form) = &self.formality {
            let form_format = format!("formality={}", form);
            par.push(form_format);
        }

        if let Some(gid) = &self.glossary_id {
            let gid_format = format!("glossary_id={}", gid);
            par.push(gid_format);
        }

        if let Some(th) = &self.tag_handling {
            let th_format = format!("tag_handling={}", th);
            par.push(th_format);
        }

        if let Some(nst) = &self.non_splitting_tags {
            let nst_format = format!("non_splitting_tags={}", nst);
            par.push(nst_format);
        }

        if let Some(od) = &self.outline_detection {
            let od_format = format!("outline_detection={}", od);
            par.push(od_format);
        }

        if let Some(st) = &self.splitting_tags {
            let st_format = format!("splitting_tags={}", st);
            par.push(st_format);
        }

        if let Some(it) = &self.ignore_tags {
            let it_format = format!("ignore_tags={}", it);
            par.push(it_format);
        }

        HttpRequest {
            endpoint: "https://api-free.deepl.com/v2/translate", 
            auth: &auth.key, 
            headers: None, 
            body: Some(par), 
            request_type: RequestType::Post
        }
    }

    pub fn set_source_lang(mut self, sl: SourceLang) -> TranslationRequest<'a> {
        self.source_lang = Some(sl);
        self
    }

    pub fn set_split_sentences(mut self, split: SplitSentences) -> TranslationRequest<'a> {
        self.split_sentences = Some(split);
        self
    }

    pub fn set_preserve_formatting(mut self, pf: bool) -> TranslationRequest<'a> {
        self.preserve_formatting = Some(pf);
        self
    }

    pub fn set_formality(mut self, formality: Formality) -> TranslationRequest<'a> {
        self.formality = Some(formality);
        self
    }

    pub fn set_glossary_id(mut self, id: &'a str) -> TranslationRequest<'a> {
        self.glossary_id = Some(id);
        self
    }

    pub fn set_tag_handling(mut self, th: TagHandling) -> TranslationRequest<'a> {
        self.tag_handling = Some(th);
        self
    }

    pub fn set_non_splitting_tags(mut self, tags: &'a str) -> TranslationRequest<'a> {
        self.non_splitting_tags = Some(tags);
        self
    }

    pub fn set_outline_detection(mut self, od: bool) -> TranslationRequest<'a> {
        self.outline_detection = Some(od);
        self
    }

    pub fn set_splitting_tags(mut self, tags: &'a str) -> TranslationRequest<'a> {
        self.splitting_tags = Some(tags);
        self
    }

    pub fn set_ignore_tags(mut self, tags: &'a str) -> TranslationRequest<'a> {
        self.ignore_tags = Some(tags);
        self
    }
}