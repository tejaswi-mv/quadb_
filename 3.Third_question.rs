//Given a string of words, implement a function that returns the shortest word in the string.
fn find_shortest(s :&str)->Option<&str>{
    let mut shortest_word:Option<&str> = None;
    let mut shortest_len = usize::MAX;
    
    for words in s.split_whitespace(){
        let word_len = words.len();
        
        if word_len<shortest_len {
            shortest_len = word_len;
            shortest_word = Some(word);
        }
    }
    shortest_word
}
