/*
 1768. Merge Strings Alternately
 https://leetcode.com/problems/merge-strings-alternately/description/?envType=study-plan-v2&envId=leetcode-75

 You are given two strings word1 and word2.
 Merge the strings by adding letters in alternating order, starting with word1.
 If a string is longer than the other, append the additional letters onto the end of the merged string.

 Return the merged string.

 Example 1:

    Input: word1 = "abc", word2 = "pqr"
    Output: "apbqcr"
    Explanation: The merged string will be merged as so:
    word1:  a   b   c
    word2:    p   q   r
    merged: a p b q c r

  Example 2:

    Input: word1 = "ab", word2 = "pqrs"
    Output: "apbqrs"
    Explanation: Notice that as word2 is longer, "rs" is appended to the end.
    word1:  a   b
    word2:    p   q   r   s
    merged: a p b q   r   s

  Example 3:

    Input: word1 = "abcd", word2 = "pq"
    Output: "apbqcd"
    Explanation: Notice that as word1 is longer, "cd" is appended to the end.
    word1:  a   b   c   d
    word2:    p   q
    merged: a p b q c   d

*/

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut answer = String::new();
    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();

    loop {
        match (chars1.next(), chars2.next()) {
            (Some(c1), Some(c2)) => {
                answer.push(c1);
                answer.push(c2);
            }
            (Some(c1), None) => {
                answer.push(c1);
                answer.extend(chars1);
                break;
            }
            (None, Some(c2)) => {
                answer.push(c2);
                answer.extend(chars2);
                break;
            }
            (None, None) => break,
        }
    }

    answer
}

#[test]
fn test_merged() {
    assert_eq!(
        merge_alternately("abc".to_string(), "pqr".to_string()),
        "apbqcr"
    );
    assert_eq!(
        merge_alternately("ab".to_string(), "pqrs".to_string()),
        "apbqrs"
    );
    assert_eq!(
        merge_alternately("abcd".to_string(), "pq".to_string()),
        "apbqcd"
    );
}
