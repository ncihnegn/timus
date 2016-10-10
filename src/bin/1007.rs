use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader};
use std::io::prelude::{Read, Write};

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut line = String::new();

    reader.read_line(&mut line).expect("usize");
    let code_length = line.trim().parse::<usize>().expect("usize");
    for line in reader.lines() {
        let cline = line.unwrap().clone();
        let word = cline.trim();
        if word.is_empty() {
            continue;
        }
        let len = word.len();
        let mut removal_offsets: BTreeMap<usize, usize> = BTreeMap::new();
        let mut insertion_offsets: BTreeMap<usize, (usize, bool)> = BTreeMap::new();
        let (sum, count) = word.char_indices().rev().collect::<Vec<_>>().into_iter().fold((0, 0), |(sum, count), (i, c)| {
            if c == '1' {
                if len == code_length+1 {
                    let offset = (i+1 + count) % (code_length+1);
                    removal_offsets.entry(offset).or_insert(i);
                }
                if len == code_length-1 {
                    insertion_offsets.entry(count+1).or_insert((i, false));
                    let offset = (i+1 + count+1) % (code_length+1);
                    insertion_offsets.entry(offset).or_insert((i, true));
                }
                (sum + i, count+1)
            } else {
                if len == code_length+1 {
                    removal_offsets.entry(count).or_insert(i);
                }
                if len == code_length-1 {
                    insertion_offsets.entry(count).or_insert((i, false));
                    let offset = (i+1 + count) % (code_length+1);
                    insertion_offsets.entry(offset).or_insert((i, true));
                }
                (sum, count)
            }
        });
        let offset = (sum + count) % (code_length+1);
        let mut chars: Vec<u8> = word.as_bytes().to_vec();
        if len == code_length {
            if offset == 0 {
                writeln!(output, "{}", word).expect("output");
                continue;
            }
            else if chars[offset-1] == '1' as u8 {
                chars[offset-1] = '0' as u8;
            } else {
                panic!("Uncorrectable flipping");
            }
        } else if len == code_length+1 {
            if let Some(&i) = removal_offsets.get(&offset) {
                chars.remove(i);
            } else {
                panic!("Uncorrectable insertion");
            }
        } else if len == code_length-1 {
            if offset == 0 {
                chars.push('0' as u8);
            } else if offset == 1 {
                chars.push('1' as u8);
            } else if let Some(&(i, b)) = insertion_offsets.get(&(code_length+1 - offset)) {
                chars.insert(i, if b { '1' as u8 } else { '0' as u8 });
            } else {
                panic!("Uncorrectable removal");
            }
            //assert_eq!(chars.len(), code_length);
            //let mut sum = 0;
            //for i in 0..code_length {
            //    if chars[i] == '1' {
            //        sum += i+1;
            //    }
            //}
            //assert_eq!(sum % (code_length+1), 0);
        } else {
            panic!("Uncorrectable length");
        }
        writeln!(output, "{}", String::from_utf8(chars).unwrap()).expect("output");
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
        let mut f = File::open("tests/1007.test").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();

        solve(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "0000\n0110\n1001\n1111\n0000\n1001\n0000\n1111\n0000\n0000\n1001\n1111\n1001\n1111\n1001\n1001\n");
    }
}
