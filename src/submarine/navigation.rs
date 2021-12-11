use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

pub struct NavigationSystem {
    memory: Vec<String>,
}

impl NavigationSystem {
    pub fn boot_from_file(path: &str) -> io::Result<NavigationSystem> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        NavigationSystem::boot(&buf)
    }

    pub fn boot(input: &str) -> io::Result<NavigationSystem> {
        let reader = BufReader::new(input.as_bytes());
        let memory = reader.lines().map(|line| line.unwrap()).collect();
        Ok(NavigationSystem { memory })
    }

    pub fn calculate_syntax_error_score(&self) -> usize {
        let mut score = 0;
        for line in &self.memory {
            if let Some(illegal_char) = first_syntax_error(line) {
                score += illegal_char_score(illegal_char);
            }
        }
        score
    }

    pub fn calculate_autocomplete_score(&self) -> usize {
        let mut scores = vec![];
        for line in &self.memory {
            if let Some(tail) = autocomplete_syntax(line) {
                scores.push(autocomplete_score(&tail));
            }
        }
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

fn illegal_char_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unsupported syntax {}", c),
    }
}

fn autocomplete_char_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unsupported syntax {}", c),
    }
}

fn autocomplete_score(input: &str) -> usize {
    let mut score = 0;
    for c in input.chars() {
        score *= 5;
        score += autocomplete_char_score(c);
    }
    score
}

fn chunk_pair(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("unsupported syntax {}", c),
    }
}

fn first_syntax_error(input: &str) -> Option<char> {
    let mut stack = vec![];
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let open = stack.pop().expect("very corrupted syntax");
                if c != chunk_pair(open) {
                    return Some(c);
                }
            }
            _ => panic!("very corrupted syntax {}", c),
        }
    }
    None
}

fn autocomplete_syntax(input: &str) -> Option<String> {
    let mut stack = vec![];
    let mut tail = String::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let open = stack.pop().expect("very corrupted syntax");
                if c != chunk_pair(open) {
                    return None;
                }
            }
            _ => panic!("very corrupted syntax {}", c),
        }
    }
    while !stack.is_empty() {
        tail.push(chunk_pair(stack.pop().unwrap()));
    }
    Some(tail)
}

#[cfg(test)]
mod tests {
    use crate::submarine::navigation::{autocomplete_score, autocomplete_syntax};

    use super::NavigationSystem;

    #[test]
    fn test_day10_part1() {
        let nav_system = NavigationSystem::boot(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
        )
        .expect("parse error");
        assert_eq!(26397, nav_system.calculate_syntax_error_score());
    }

    #[test]
    fn test_day10_part2() {
        let nav_system = NavigationSystem::boot(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
        )
        .expect("parse error");
        assert_eq!(288957, nav_system.calculate_autocomplete_score());
    }

    #[test]
    fn test_autocomplete_score() {
        assert_eq!(
            [288957, 5566, 1480781, 995444, 294,],
            [
                autocomplete_score("}}]])})]"),
                autocomplete_score(")}>]})"),
                autocomplete_score("}}>}>))))"),
                autocomplete_score("]]}}]}]}>"),
                autocomplete_score("])}>"),
            ]
        )
    }

    #[test]
    fn test_autocomplete() {
        assert_eq!(
            Some("}}]])})]".to_string()),
            autocomplete_syntax("[({(<(())[]>[[{[]{<()<>>")
        );
        assert_eq!(
            Some(")}>]})".to_string()),
            autocomplete_syntax("[(()[<>])]({[<{<<[]>>(")
        );
        assert_eq!(
            Some("}}>}>))))".to_string()),
            autocomplete_syntax("(((({<>}<{<{<>}{[]{[]{}")
        );
        assert_eq!(
            Some("]]}}]}]}>".to_string()),
            autocomplete_syntax("{<[[]]>}<{[{[{[]{()[[[]")
        );
        assert_eq!(
            Some("])}>".to_string()),
            autocomplete_syntax("<{([{{}}[<[[[<>{}]]]>[]]")
        );
        assert_eq!(None, autocomplete_syntax("{([(<{}[<>[]}>{[]{[(<()>"));
    }
}
