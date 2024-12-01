pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    (start.min(end)..=start.max(end)).find(|i| {
        let s = i.to_string();
        s.chars().zip(s.chars().rev()).all(|(c1, c2)| c1 == c2)
    })
}
