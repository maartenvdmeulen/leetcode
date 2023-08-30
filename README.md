# leetcode

My leetcode solutions.

## solutions

| nr.  | title                                                                                               | difficulty | code                                                                                               | algo / datastructure   |
| :--- | :-------------------------------------------------------------------------------------------------- | :--------- | :------------------------------------------------------------------------------------------------- | :--------------------- |
| 1    | [two sum](https://leetcode.com/problems/two-sum/)                                                   | Easy       | [rust](/rust/src/two_sum_1/mod.rs)                                                                 | `hashmap`              |
| 9    | [palindrome number](https://leetcode.com/problems/palindrome-number/)                               | Easy       | [rust](/rust/src/palindrome_number_9/mod.rs)                                                       | `pointers`             |
| 20   | [valid parentheses](https://leetcode.com/problems/valid-parentheses/)                               | Easy       | [rust](/rust/src/valid_parentheses_20/mod.rs), [ts](/typescript/valid_parentheses_20/main.test.ts) | `stack`                |
| 49   | [group anagrams](https://leetcode.com/problems/group-anagrams/)                                     | Medium     | [rust](/rust/src/group_anagrams_49/mod.rs)                                                         | `hashmap` `flaky-test` |
| 125  | [valid palindrome](https://leetcode.com/problems/valid-palindrome/)                                 | Easy       | [rust](/rust/src/valid_palindrome_125/mod.rs)                                                      | `pointer`              |
| 150  | [evaluate reverse polish notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/) | Medium     | [rust](/rust/src/evaluate_reverse_polish_notation_150/mod.rs)                                      | `stack`                |
| 155  | [min stack](https://leetcode.com/problems/min-stack/)                                               | Medium     | [rust](/rust/src/min_stack_155/mod.rs)                                                             | `stack`                |
| 217  | [contains duplicate](https://leetcode.com/problems/contains-duplicate/)                             | Easy       | [rust](/rust/src/contains_duplicate_217/mod.rs)                                                    | `hashset`              |
| 238  | [product of array except self](https://leetcode.com/problems/product-of-array-except-self/)         | Medium     | [rust](/rust/src/product_of_array_except_self_238/mod.rs)                                          | `array`                |
| 242  | [valid anagram](https://leetcode.com/problems/valid-anagram/)                                       | Easy       | [rust](/rust/src/valid_anagram_242/mod.rs)                                                         | `hashmap`              |
| 347  | [top k frequent elements](https://leetcode.com/problems/top-k-frequent-elements/)                   | Medium     | [rust](/rust/src/top_k_frequent_elements_347/mod.rs)                                               | `hashmap`              |

## run tests

### typescript

```bash
deno test
```

### rust

```bash
cd rust
cargo test
```
