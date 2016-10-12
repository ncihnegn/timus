use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::io::prelude::{Read, Write};
use std::usize;

fn dijkstra(neighbors: &Vec<HashMap<usize, usize>>, source: usize) -> (usize, Vec<usize>) {
    let max_vertex = neighbors.len();
    let mut vertex_set: HashSet<usize> = (1usize..max_vertex).collect();

    let mut dist: Vec<usize> = vec![usize::MAX; max_vertex];
    let mut prev: Vec<usize> = vec![source; max_vertex];
    dist[source] = 0;

    while !vertex_set.is_empty() {
        let u = *vertex_set.iter().min_by_key(|&&x| dist[x]).unwrap();
        let d = dist[u];
        //println!("choose {} {} {}", source, u, d);
        if u == source {
            dist[source] = usize::MAX;
        }
        if d == usize::MAX {
            break;
        }
        vertex_set.remove(&u);
        for (&v, &l) in neighbors[u].iter() {
            let alt = d + l;
            if alt < dist[v] {
                dist[v] = alt;
                prev[v] = u;
            }
        }
    }
    let mut route: Vec<usize> = vec!(source);
    let mut next: usize = prev[source];
    while next != source {
        route.push(next);
        next = prev[next];
    }
    //println!("{:?}", route);

    return (dist[source], route);
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut line = String::new();

    while reader.read_line(&mut line).is_ok() {
        if line.starts_with("-1") {
            return;
        }
        let mut numbers: Vec<_> = line.trim().split(' ').map(|a| a.parse::<usize>().unwrap()).collect();
        line.clear();

        let num_vertices = numbers[0];
        let num_edges = numbers[1];

        let mut neighbors: Vec<HashMap<usize, usize>> = vec![HashMap::new(); num_vertices+1];
        for _ in 0..num_edges {
            reader.read_line(&mut line).expect("road");
            numbers = line.trim().split(' ').map(|a| a.parse::<usize>().unwrap()).collect();
            line.clear();
            let l = neighbors[numbers[0]].entry(numbers[1]).or_insert(numbers[2]);
            if *l > numbers[2] {
                *l = numbers[2];
            }
        }
        let mut min_dist: usize = usize::MAX;
        let mut shortest_path: Vec<usize> = Vec::new();
        for i in 1usize..num_vertices+1 {
            let (d, r) = dijkstra(&neighbors, i);
            if d < min_dist {
                min_dist = d;
                shortest_path = r;
            }
        }
        if shortest_path.is_empty() {
            writeln!(output, "No solution.").expect("output");
        } else {
            writeln!(output, "{}", shortest_path.iter().fold("".to_string(), |s, i| s + " " + &i.to_string()).trim()).expect("output");
        }
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
        let mut f = File::open("tests/1004.test").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();

        solve(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "1 3 5 2\nNo solution.\n");
    }
}
