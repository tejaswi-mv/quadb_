//Implement a function that checks whether a given string is a palindrome or not.
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
       let str = s.chars()
                  .filter(|c| c.is_alphanumeric())
                  .collect::<String>()
                  .to_lowercase();
            str.chars()
               .eq(str.chars().rev())
    }
}
