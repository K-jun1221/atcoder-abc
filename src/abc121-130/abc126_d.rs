use std::collections::HashMap;
use std::io::*;
use std::str::FromStr;


pub fn read<T: FromStr>() -> T {
  let stdin = stdin();
  let stdin = stdin.lock();
  let token: String = stdin
    .bytes()
    .map(|c| c.expect("failed to read char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  token.parse().ok().expect("failed to parse token")
}

struct Edge {
  to: usize,
  weight: i32,
}


fn main() {
  let n = read::<usize>();
  let mut graph: HashMap<usize, Vec<Edge>> = HashMap::new();
  let mut colors = vec![-1; n + 1];

  for _ in 0..n - 1 {
    let a = read::<usize>();
    let b = read::<usize>();
    let w = read::<i32>();

    add_edge(a, b, w, &mut graph);
    add_edge(b, a, w, &mut graph);
  }

  dfs(1000, 1, &mut colors, 0, &graph);

  colors.remove(0);
  for i in colors {
    println!("{:?}", i);
  }
}

fn add_edge(from: usize, to: usize, weight: i32, graph: &mut HashMap<usize, Vec<Edge>>) {
  if graph.contains_key(&from) {
    graph.get_mut(&from).unwrap().push(Edge {
      to: to,
      weight: weight,
    });
  } else {
    graph.insert(
      from,
      vec![Edge {
        to: to,
        weight: weight,
      }],
    );
  }
}

fn dfs(
  from: usize,
  to: usize,
  colors: &mut Vec<i32>,
  color: i32,
  graph: &HashMap<usize, Vec<Edge>>,
) {
  colors[to] = color;

  for edge in graph.get(&to).unwrap() {
    if edge.to != from {
      let new_color = (color + edge.weight) % 2;
      dfs(to, edge.to, colors, new_color, graph);
    }
  }
}
