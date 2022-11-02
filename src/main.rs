const MATCH_SCORE: i32 = 5;
const MISMATCH_SCORE: i32 = -3;
const INSERTION_DELETION_SCORE: i32 = -4;

const DNA1: &[u8] = b"CGTGAA";
const DNA2: &[u8] = b"GACTTAC";

struct Node {
    highest_score: i32,
    highest_prev_nodes: Vec<(usize, usize)>,
}

fn main() {
    // Creates a grid of Option<Node> (initialised to None) indexed by (i,j) where
    // 0 <= i < DNA2.len()
    // 0 <= j < DNA1.len()
    let mut nodes: Vec<Vec<Option<Node>>> = (0..DNA2.len() + 1)
        .map(|_| (0..DNA1.len() + 1).map(|_| None).collect())
        .collect();

    // Traversing in row-major order
    //    <---j--->
    // /\
    // |
    // i
    // |
    // \/
    for i in 0..DNA2.len() + 1 {
        for j in 0..DNA1.len() + 1 {
            let mut highest_score = i32::MIN;
            let mut highest_prev_nodes = Vec::new();

            // Base case for first (top-left) node
            if i == 0 && j == 0 {
                highest_score = 0;
            }
            // Deletion case
            if i > 0 {
                let deletion_prev_node = nodes[i - 1][j].as_ref().unwrap();
                let deletion_score = deletion_prev_node.highest_score + INSERTION_DELETION_SCORE;

                if deletion_score > highest_score {
                    highest_score = deletion_score;
                    highest_prev_nodes = vec![(i - 1, j)];
                } else if deletion_score == highest_score {
                    highest_prev_nodes.push((i - 1, j));
                }
            }
            // Insertion case
            if j > 0 {
                let insertion_prev_node = nodes[i][j - 1].as_ref().unwrap();
                let insertion_score = insertion_prev_node.highest_score + INSERTION_DELETION_SCORE;

                if insertion_score > highest_score {
                    highest_score = insertion_score;
                    highest_prev_nodes = vec![(i, j - 1)];
                } else if insertion_score == highest_score {
                    highest_prev_nodes.push((i, j - 1));
                }
            }
            // Match or mismatch case
            if i > 0 && j > 0 {
                let match_mismatch_prev_node = nodes[i - 1][j - 1].as_ref().unwrap();
                let match_mismatch_score = if DNA2[i - 1] == DNA1[j - 1] {
                    MATCH_SCORE
                } else {
                    MISMATCH_SCORE
                } + match_mismatch_prev_node.highest_score;

                if match_mismatch_score > highest_score {
                    highest_score = match_mismatch_score;
                    highest_prev_nodes = vec![(i - 1, j - 1)];
                } else if match_mismatch_score == highest_score {
                    highest_prev_nodes.push((i - 1, j - 1));
                }
            }

            nodes[i][j] = Some(Node {
                highest_score,
                highest_prev_nodes,
            });
        }
    }

    // Every node should be set now, so we can unwrap all the options
    let nodes: Vec<Vec<Node>> = nodes
        .into_iter()
        .map(|row| row.into_iter().map(Option::unwrap).collect())
        .collect();

    let results = backtrace(&nodes, DNA2.len(), DNA1.len());

    println!(
        "ORIGINAL:\n DNA1:    {}\n DNA1:    {}",
        std::str::from_utf8(DNA1).unwrap(),
        std::str::from_utf8(DNA2).unwrap(),
    );

    println!("ALIGNED:");

    for (dna1, dna2, actions) in results {
        println!(
            " DNA1:    {}\n DNA2:    {}\n ACTIONS: {}\n",
            dna1, dna2, actions
        );
    }

    println!(" SCORE:   {}", nodes[DNA2.len()][DNA1.len()].highest_score);
}

fn backtrace(nodes: &Vec<Vec<Node>>, i: usize, j: usize) -> Vec<(String, String, String)> {
    if i == 0 && j == 0 {
        vec![(String::new(), String::new(), String::new())]
    } else {
        let node = &nodes[i][j];

        let mut final_results: Vec<(String, String, String)> = Vec::new();

        for (prev_i, prev_j) in node.highest_prev_nodes.clone() {
            let res = backtrace(nodes, prev_i, prev_j);

            if prev_i + 1 == i && prev_j + 1 == j {
                final_results.extend(res.clone().into_iter().map(
                    |(mut dna1, mut dna2, mut actions)| {
                        dna1.push(DNA1[prev_j] as char);
                        dna2.push(DNA2[prev_i] as char);
                        actions.push(if DNA2[prev_i] == DNA1[prev_j] {
                            'Y'
                        } else {
                            'N'
                        });
                        (dna1, dna2, actions)
                    },
                ));
            } else if prev_i + 1 == i && prev_j == j {
                final_results.extend(res.clone().into_iter().map(
                    |(mut dna1, mut dna2, mut actions)| {
                        dna1.push('-');
                        dna2.push(DNA2[prev_i] as char);
                        actions.push('D');
                        (dna1, dna2, actions)
                    },
                ));
            } else if prev_i == i && prev_j + 1 == j {
                final_results.extend(res.clone().into_iter().map(
                    |(mut dna1, mut dna2, mut actions)| {
                        dna1.push(DNA1[prev_j] as char);
                        dna2.push('-');
                        actions.push('I');
                        (dna1, dna2, actions)
                    },
                ));
            }
        }

        final_results
    }
}
