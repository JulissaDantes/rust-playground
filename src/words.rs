use std::char;



/// Function to solve the ny times letter box daily game.
/// Constrains: Must use as many letters as possible, must not use letters that are in the same side
/// Extra: Can specify initial letter and which letters would like to include
pub fn get_words(start: Option<char>, included: Option<&[char]>, square:[[char;3]; 4]) -> Vec<String> {
    let dict = dictionary2::DICTIONARY;
    let mut res: Vec<String> = dict.iter().map(|&s| s.to_owned()).collect();
    let invalid_words = vec!["lowp".to_string(),"srinivas".to_string(),"jayant".to_string(),"enlinkment".to_string(),"suji".to_string(),"saj".to_string(), "receiptment".to_string(), "enrockment".to_string(), "kemptken".to_string(), "recompilement".to_string(), "temporopontine".to_string(), "kleinite".to_string(), "klicket".to_string(), "knopite".to_string(),"inqilab".to_string(), "ketipic".to_string(), "knickpoint".to_string(), "unantiquity".to_string(), "yariyari".to_string(), "solvabling".to_string(),"yanquis".to_string()];
    //remove words with less than 3 letters
    res = res
    .iter()
    .filter(|w| w.len() > 2)
    .cloned()
    .collect();

    if let Some(start_char) = start {
        // Filter words that start with the specified character
        res = res
            .iter()
            .filter(|w| w.starts_with(start_char))
            .cloned()
            .collect();
    }

    if included.is_some() {
        // Filter words that contain the specified character
        res = res
            .iter()
            .filter(|w| has_valid_letters(w, included.unwrap()))
            .cloned()
            .collect();
    }

    // Make sure letters on the same side are not adjacent
    for side in square {
        res = res
            .iter()
            .filter(|w| !has_adjacent_letters(w, &side))
            .cloned()
            .collect();
    }

    // filter invalid words
    res.retain(|word| !invalid_words.contains(word));

    res
}

fn has_adjacent_letters(word: &str, letters: &[char; 3]) -> bool {
    // Iterate through the characters of the word
    for (i, c) in word.chars().enumerate().take(word.len() - 1) {
        // Check if the pair of adjacent characters exists in the letters array
        if letters.contains(&c) && letters.contains(&word.chars().nth(i + 1).unwrap()) {
            return true; // Found a pair of adjacent letters from the array
        }
    }
    false // No pair of adjacent letters found
}

fn has_valid_letters(word: &str, letters: &[char]) -> bool {
    // Iterate through the characters of the word
    for (_i, c) in word.chars().enumerate().take(word.len()) {
        // check if a char is not in the array
        if !letters.contains(&c) {
            return false; // Found letter not included in the square
        }
    }
    true // All letters are in the square
}