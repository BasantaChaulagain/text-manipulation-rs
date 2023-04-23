__A crate for generating random placeholder text and translation in multiple languages.__

__Random Placeholder Text:__
## Usage
1. Download the corpus (text files) from [here](https://github.com/BasantaChaulagain/text-manipulation-rs/tree/release/src) and place them in the root folder of your rust project.
2. Include the crate name version in Cargo.toml file.
3. Use the crate and call the function in your code file. The parameters of the function `generate_text_for_language` are language: i32 which takes integers from 0 to 12 (see below for mapping) and write_to_file: bool which writes to a file if true.
```
use text_manipulation_rs::text_manipulation;
...
text_manipulation::generate_text_for_language(language: i32, write_to_file: bool);
```

## Languages supported
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

__Word/Sentence Translation:__ 

## Usage
1. Include the crate name version in Cargo.toml file.
2. Use the crate and call the function in your code file. The parameters of the function 'translate_q_pair' are q: String and langpair: String. 
3. The parameter q:  the text a user wants to translate. 
4. The parameter langpair: Source and destination language separated by '|'. Keep in mind the langpair parameter follows a strict format that follows RFC 3066/2 letter ISO. Check this out for guidance - http://www.i18nguy.com/unicode/language-identifiers.html. 

```
use text_manipulation_rs::text_manipulation;
...
text_manipulation::translate_q_langpair(q: String, langpair: String);
```
