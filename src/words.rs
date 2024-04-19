use std::char;

/// Function to solve the ny times letter box daily game.
/// Constrains: Must use as many letters as possible, must not use letters that are in the same side
/// Extra: Can specify initial letter and which letters would like to include
/// 
pub fn get_words(start: Option<char>, included: Option<&[char]>) -> Vec<String> {
    let dict = dictionary2::DICTIONARY;
    let mut res: Vec<String> = dict.iter().map(|&s| s.to_owned()).collect();

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
            .filter(|w| w.contains(included.unwrap()))
            .cloned()
            .collect();
    }

    res
}