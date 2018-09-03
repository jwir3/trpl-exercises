use std::io;

fn main() {
    println!("Enter a string:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut result: String = String::new();

    // Break the string into words
    let words = break_into_words(input);

    // Translate the words into pig latin
    for word in words {
        result.push_str(&translate_to_pig_latin(word));
        result.push_str(" ");
    }

    // Output the result
    println!("Your translated phrase is: {}", result);
}

fn break_into_words(input: String) -> Vec<String> {
    let mut words = Vec::new();

    let mut next_word = String::new();
    for next_char in input.chars() {
        if next_char == ' ' || next_char == ',' || next_char == '.' || next_char == '\n' {
            words.push(next_word);
            next_word = String::new();
        } else {
            next_word.push(next_char);
        }
    }
    words
}

fn translate_to_pig_latin(word: String) -> String {
    let mut result = String::new();

    // Note that this won't actually work for strings containing non-single-byte characters. That
    // said, it's unclear how pig latin would be defined for a string like Здравствуйте, so we can
    // probably safely assume we're working in English here.
    if is_vowel(&word[0..1]) {
        // Append '-hay' to the end of the string and return it.
        result.push_str(&word);
        result.push_str("-hay");
    } else {
        // Append everything except for the first character, add the first character to the end
        // and then add 'ay'.
        result.push_str(&word[1..]);
        result.push_str(&word[0..1]);
        result.push_str("ay");
    }

    result
}

fn is_vowel(character: &str) -> bool {
    return character == "a" || character == "e" || character == "i" || character == "o"
        || character == "u";
}
