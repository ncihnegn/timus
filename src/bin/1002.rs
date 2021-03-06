use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader};
use std::io::prelude::{Read, Write};

fn transcript(s: &str) -> String {
    s.clone().chars().map(|a|
                          match a {
                              'i' => '1',
                              'j' => '1',
                              'a' => '2',
                              'b' => '2',
                              'c' => '2',
                              'd' => '3',
                              'e' => '3',
                              'f' => '3',
                              'g' => '4',
                              'h' => '4',
                              'k' => '5',
                              'l' => '5',
                              'm' => '6',
                              'n' => '6',
                              'p' => '7',
                              'r' => '7',
                              's' => '7',
                              't' => '8',
                              'u' => '8',
                              'v' => '8',
                              'w' => '9',
                              'x' => '9',
                              'y' => '9',
                              'o' => '0',
                              'q' => '0',
                              'z' => '0',
                              _ => ' '
                          }).collect::<String>()
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut line = String::new();

    while reader.read_line(&mut line).is_ok() {
        let cline = line.clone();
        let number = cline.trim();
        if number.starts_with("-1") {
            return;
        }

        line.clear();
        reader.read_line(&mut line).expect("number of words");
        let num_words = line.trim().parse::<usize>().unwrap();
        let mut words: Vec<String> = Vec::new();
        let mut sc: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut flag: Vec<bool> = vec![false; number.len()+1];
        let mut solved = false;
        for _ in 0..num_words {
            line.clear();
            reader.read_line(&mut line).expect("word");
            let word = line.trim();
            let l = word.len();
            if l <= number.len() && number.contains(&transcript(word)) {
                if number.starts_with(&transcript(word)) {
                    if l == number.len() {
                        writeln!(output, "{}", word).expect("output");
                        solved = true;
                        break;
                    }
                    sc.entry(l).or_insert(vec!(words.len()));
                    flag[l] = true;
                }
                words.push(word.to_string());
            }
        }

        let mut least_number: usize = 1;

        'outer: while !solved && !sc.is_empty() {
            let mut st: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

            for &l in sc.keys() {
                let slice = &number[l..];
                for i in 0..words.len() {
                    let ref word = words[i];
                    let next_len = l + word.len();
                    if next_len <= number.len() && !flag[next_len] && slice.starts_with(&transcript(&word)) {
                        let mut t = sc.get(&l).unwrap().clone();
                        t.push(i);
                        if next_len == number.len() {
                            writeln!(output, "{}", t.iter().fold(String::new(), |state, &j| state + " " + &words[j]).trim()).expect("output");
                            solved = true;
                            break 'outer;
                        }
                        flag[next_len] = true;
                        st.entry(next_len).or_insert(t);
                    }
                }
            }
            sc = st;
            least_number += 1;
        }

        if !solved {
            writeln!(output, "No solution.").expect("output");
        }
        line.clear();
    }
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use solve;

    #[test]
    fn basic_test() {
        let mut f = File::open("tests/1002.test").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();

        solve(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "reality our\nNo solution.\n");
    }
}
