pub mod bellman_ford {
    pub fn shortest_path(
        graph: &Vec<Vec<(usize, i64)>>,
        start: usize,
        inf: i64,
    ) -> (Vec<i64>, Vec<bool>) {
        let n = graph.len();
        let mut dist = vec![inf; n];
        dist[start] = 0;
        for _ in 0..n {
            for v in 0..n {
                for &(to, cost) in &graph[v] {
                    if dist[v] == inf || dist[to] <= dist[v] + cost {
                        continue;
                    }
                    dist[to] = dist[v] + cost;
                }
            }
        }

        let mut negative = vec![false; n];
        for _ in 0..n {
            for v in 0..n {
                for &(to, cost) in &graph[v] {
                    if dist[v] == inf {
                        continue;
                    }
                    if dist[to] > dist[v] + cost {
                        dist[to] = dist[v] + cost;
                        negative[to] = true;
                    }
                    if negative[v] {
                        negative[to] = true;
                    }
                }
            }
        }

        return (dist, negative);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::Tester;
    use std;

    #[test]
    fn solve_grl_1_b() {
        let tester = Tester::new("./assets/GRL_1_B/in/", "./assets/GRL_1_B/out/");
        tester.test_solution(|sc| {
            let v: usize = sc.read();
            let e: usize = sc.read();
            let r: usize = sc.read();

            let mut graph = vec![vec![]; v];

            for _ in 0..e {
                let s: usize = sc.read();
                let t: usize = sc.read();
                let d: i64 = sc.read();
                graph[s].push((t, d));
            }

            let inf = std::i64::MAX;

            let (dist, negative) = bellman_ford::shortest_path(&graph, r, inf);
            let mut neg = false;
            for &b in &negative {
                neg = neg || b;
            }

            if neg {
                sc.write("NEGATIVE CYCLE\n");
            } else {
                for i in 0..v {
                    if dist[i] == inf {
                        sc.write("INF\n");
                    } else {
                        sc.write(format!("{}\n", dist[i]));
                    }
                }
            }
        });
    }
}
