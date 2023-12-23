use std::collections::HashMap;

enum Next {
    Left,
    Right
}

impl From<char> for Next {
    fn from(c: char) -> Self {
        match c {
            'L' => Next::Left,
            _ => Next::Right,
        }
    }
}

struct Node {
    left: String,
    right: String,
}

pub fn solve_a(input: &Vec<&str>) -> usize {
    let seq: Vec<Next> = input[0].chars().map(|c| Next::from(c)).collect();
    let mut nodes = HashMap::<String, Node>::with_capacity(input.len() - 2);
    for node_desc in input[2..].iter() {
        let s: Vec<&str> = node_desc.split(" = ").collect(); 
        let id = s[0].to_string();
        let left = (&s[1][1..4]).to_string();
        let right = (&s[1][6..9]).to_string();
    
        nodes.insert(id, Node {
            left,
            right
        });
    }

    let mut steps = 0;

    let mut node_id = "AAA".to_string();
    let mut seq_idx = 0;
    while node_id != "ZZZ" {
        let next = &seq[seq_idx];
        let node = &nodes[&node_id];
        node_id = match next {
            Next::Left => node.left.clone(),
            Next::Right => node.right.clone(),
        };

        steps += 1;
        seq_idx += 1;
        if seq_idx >= seq.len() {
            seq_idx = 0;
        }
    }

    steps
}

pub fn solve_b(input: &Vec<&str>) -> usize {
    let seq: Vec<Next> = input[0].chars().map(|c| Next::from(c)).collect();
    let mut nodes = HashMap::<String, Node>::with_capacity(input.len() - 2);
    let mut visitors = Vec::<String>::new();
    for node_desc in input[2..].iter() {
        let s: Vec<&str> = node_desc.split(" = ").collect(); 
        let id = s[0].to_string();

        if id.ends_with("A") {
            visitors.push(id.clone());
        }

        let left = (&s[1][1..4]).to_string();
        let right = (&s[1][6..9]).to_string();
    
        nodes.insert(id, Node {
            left,
            right
        });
    }


    let mut visitor_steps = vec![0; visitors.len()];
    for (i, v) in visitors.iter().enumerate() {
        let mut steps = 0;
        let mut seq_idx = 0;
        let mut node_id = v.to_string();
        while !node_id.ends_with("Z") {
            let next = &seq[seq_idx];
            let node = &nodes[&node_id];
            node_id = match next {
                Next::Left => node.left.clone(),
                Next::Right => node.right.clone(),
            };

            steps += 1;
            seq_idx += 1;
            if seq_idx >= seq.len() {
                seq_idx = 0;
            }
        }

        visitor_steps[i] = steps;
    }

    fn gcd(mut a:usize, mut b:usize) -> usize{
        if a==b { return a; }
        if b > a {
            let temp = a;
            a = b;
            b = temp;
        }
        while b>0 {
            let temp = a;
            a = b;
            b = temp%b;
        }
        return a;
    }

    fn lcm(a:usize, b:usize) -> usize{
        // LCM = a*b / gcd
        return a*(b/gcd(a,b));
    }

    let mut x = lcm(visitor_steps[0], 1);
    for s in visitor_steps[1..].iter() {
        x = lcm(x, *s);
    }
    
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        let input = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        assert_eq!(solve_a(&input), 2);

        
        let input = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        assert_eq!(solve_a(&input), 6);
    }

    #[test]
    fn test_solve_b() {
        let input = vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];
        assert_eq!(solve_b(&input), 6);
    }
}
