
pub fn translate(word: &str) -> String {
    let vowels = ["a", "e", "i", "o", "u"];
    let word_str = word.to_string();
    let (head, tail) = word_str.split_at(1);
    if vowels.contains(&head) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", tail, head)
    }
}
