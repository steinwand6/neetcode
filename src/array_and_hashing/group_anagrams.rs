use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut answer: Vec<Vec<String>> = vec![];
        for s in strs.into_iter() {
            let mut key: Vec<char> = s.chars().collect();
            let mut found_group = false;
            key.sort();
            for ans in answer.iter_mut() {
                if s.len() != ans[0].len() {
                    continue;
                }
                let mut another_key: Vec<char> = ans[0].chars().collect();
                another_key.sort();
                if key == another_key {
                    ans.push(s.clone());
                    found_group = true;
                    break;
                }
            }
            if !found_group {
                answer.push(vec![s]);
            }
        }
        answer
    }
}

#[cfg(test)]
mod group_anagrams_test {
    use crate::Solution;

    #[test]
    fn test1() {
        let strs = str_to_string(vec!["eat", "tea", "tan", "ate", "nat", "bat"]);

        let expect: Vec<Vec<String>> =
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
                .into_iter()
                .map(|s| str_to_string(s.into_iter().collect()))
                .collect();
        let result = Solution::group_anagrams(strs);
        assert_eq!(expect.len(), result.len());
        for s in result.iter() {
            let mut tmp = s.clone();
            tmp.sort();
            assert!(expect.contains(&tmp));
        }
    }

    #[test]
    fn test2() {
        let strs = str_to_string(vec!["a"]);

        let expect: Vec<Vec<String>> = vec![vec!["a"]]
            .into_iter()
            .map(|s| str_to_string(s.into_iter().collect()))
            .collect();
        assert_eq!(Solution::group_anagrams(strs), expect);
    }

    fn str_to_string(strs: Vec<&str>) -> Vec<String> {
        strs.into_iter().map(|s| s.to_string()).collect()
    }
}
