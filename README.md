__A crate for generating random placeholder text and translation in multiple languages.__

There are four modules in this crate with the following features:
1. text_generator: A module for generating random placeholder text in multiple languages.
2. deepl: A module for language translation, glossary management.......
3. my_memory: A module for translating text in different languages.
4. dictionary: A module to give the meaning of valid words in English.

## text_generator:
### Usage
1. Download the corpus (text files) from [here](https://github.com/BasantaChaulagain/text-manipulation-rs/tree/release3/corpus) and place the directory in the root folder of your rust project.
2. Include the crate name version in Cargo.toml file.
3. Use the crate and call the function in your code file. The parameters of the function `generate_text_for_language` are language: i32 which takes integers from 0 to 12 (see below for mapping) and write_to_file: bool which writes to a file if true.
```
use text_manipulation_rs::text_generator;
text_generator::generate_text_for_language(language: i32, write_to_file: bool);
```
### Languages supported
0 => english \
1 => french \
2 => spanish \
3 => hindi \
4 => russian \
5 => arabic \
6 => japanese \
7 => german \
8 => latin \
9 => czech \
10 => Irish \
11 => Swedish 

## my_memory:
### Usage
1. Include the crate name version in Cargo.toml file.
2. Use the crate and call the function in your code file. The parameters of the function 'translate_q_pair' are q: String and langpair: String. 
3. The parameter q:  the text a user wants to translate. 
4. The parameter langpair: Source and destination language separated by '|'. Keep in mind the langpair parameter follows a strict format that follows RFC 3066/2 letter ISO. Check this out for guidance - http://www.i18nguy.com/unicode/language-identifiers.html. 

```
use text_manipulation_rs::my_memory;
my_memory::translate_q_langpair(q: String, langpair: String);
```

## dictionary:
This module uses an API from [Merriam-Webster developer center](https://dictionaryapi.com/products/api-collegiate-thesaurus). User need to generate their own API key to be able to use this module.

### Usage
1. Include the crate name version in Cargo.toml file.
2. Generate a developer's API key from the site linked above, place the key in a file named 'dict_secret', and keep the file in the root directory of the project.
3. Use the crate and call the function in your code file. The parameter of a function `get_meaning` is word:&str, and it returns a vector of all the definitions of the word if valid.

```
use text_manipulation_rs::dictionary;
dictionary::translate_q_langpair(q: String, langpair: String);
```