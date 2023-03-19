** A crate for generating random placeholder text in different languages. **

## Usage
Include the crate name version in Cargo.toml file, and use the crate in the source file. The parameters of the function `generate_text_for_language` are language: i32 which takes integers from 0 to 7 (see below for mapping) and write_to_file: bool which writes to a file if true.
```
use text_manipulation::text_manipulation;
...
text_manipulation::generate_text_for_language(language: i32, write_to_file: bool);
```

## Languages supported
0 => english
1 => french
2 => spanish
3 => hindi
4 => russian
5 => arabic
6 => japanese
7 => german