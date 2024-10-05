fn most_frequent_word(text: &str) -> (String, usize) {
    // output
    let mut word_counts: Vec<(String, usize)> = Vec::new();

    // text parsed into words
    let words:Vec<&str> = text.split_whitespace().collect();

    for word in words {
        let mut found = false;
        // loop through word_counts
        for (existing_word, count) in &mut word_counts {
            // Increment count if the word is found
            if existing_word == word {
                *count += 1; 
                found = true;
                break;
            }
        }

        // Increment count if the word is found
        if !found {
            word_counts.push((word.to_string(), 1)); 
        }
    }

    // Find the word with the highest frequency
    let mut max_word = String::new();
    let mut max_count = 0;

    for (word, count) in word_counts {
        if count > max_count {
            max_word = word;
            max_count = count;
        }
    }
    
    (max_word, max_count) // return tuple
}

pub fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}