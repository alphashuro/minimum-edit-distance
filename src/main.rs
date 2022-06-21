use std::fmt;

use clap::Parser;

type Cost = u8;

/// Calculates the minimum edit distance from one word to another
#[derive(Parser, Debug)]
#[clap(author = "Alpha Shuro", version = "1.0", about, long_about = None)]
struct Args {
    /// Source word 
    #[clap(short, long, value_parser)]
    from: String,

    /// Target word 
    #[clap(short, long, value_parser)]
    to: String,

    /// Print the distance matrix
    #[clap(short, long, value_parser, default_value_t = false)]
    matrix: bool,

    /// Insertion cost
    #[clap(short, long, value_parser, default_value_t = 1)]
    insertion_cost: Cost,

    /// Deletion cost 
    #[clap(short, long, value_parser, default_value_t = 1)]
    deletion_cost: Cost,

    /// Substitution cost 
    #[clap(short, long, value_parser, default_value_t = 2)]
    substitution_cost: Cost,
}

struct DistanceMatrix<'a> {
    from: &'a str,
    to: &'a str,
    matrix: Vec<Vec<Cost>>,
}

struct DistanceOptions {
    insertion_cost: Option<Cost>,
    deletion_cost: Option<Cost>,
    substitution_cost: Option<Cost>,
}

impl<'a> DistanceMatrix<'a> {
    pub fn new(from: &'a str, to: &'a str) -> Self {
        let width = from.len();
        let height = to.len();

        Self {
            from,
            to,
            matrix: vec![vec![0; width + 1]; height + 1]
        }
    }

    /// Calcuates the minimum edit distance given the costs
    /// Modifies the distance matrix in place
    pub fn get_distance(&mut self, options: Option<DistanceOptions>) -> u8 {
        let options = options.unwrap_or(
            DistanceOptions{ 
                insertion_cost: Some(1), 
                deletion_cost: Some(1),
                substitution_cost: Some(2),
            },
        );
        let insertion_cost = options.insertion_cost.unwrap_or(1);
        let deletion_cost = options.deletion_cost.unwrap_or(1);
        let substitution_cost = options.substitution_cost.unwrap_or(2);

        // compute insertion costs for intial rows
        // for from:
        for i in 1..self.matrix[0].len() {
            self.matrix[0][i] = self.matrix[0][i-1] + insertion_cost;
        }

        // for to:
        for i in 1..self.matrix.len() {
            self.matrix[i][0] = self.matrix[i-1][0] + insertion_cost;
        }

        let from: Vec<char> = self.from.chars().collect();
        let to: Vec<char> = self.to.chars().collect();

        // compute the distances in the matrix
        for i in 1..self.matrix.len() {
            for j in 1..self.matrix[i].len() {
                let insertion = self.matrix[i - 1][j] + insertion_cost;
                let deletion = self.matrix[i][j - 1] + deletion_cost;
                let replacement = self.matrix[i - 1][j - 1] + substitution_cost;

                let is_same_letter = from[j - 1] == to[i - 1];

                let min_cost = if is_same_letter {
                    self.matrix[i-1][j-1]
                } else {
                    *vec![insertion, deletion, replacement].iter().min().unwrap()
                };

                self.matrix[i][j] = min_cost;
            }
        }

        // return the value in the last column of the last row
        *self.matrix.last().map(|r| r.last().unwrap()).unwrap()
    }
}

impl fmt::Display for DistanceMatrix<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  #")?;
        
        for char in self.from.chars() {
            write!(f, " {}", char)?;
        }

        write!(f, "\n")?;

        write!(f, "# ")?;
        
        for x in self.matrix[0].iter() {
            write!(f, "{} ", x)?;
        }

        write!(f, "\n")?;
    
        for (index, char) in self.to.chars().enumerate() {
            write!(f, "{} ", char)?;

            for x in self.matrix[index + 1].iter() {
                write!(f, "{} ", x)?;
            }
            
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    let mut matrix = DistanceMatrix::new(&args.from, &args.to);

    let distance = matrix.get_distance(None);

    println!("distance: {}", distance);
    
    if args.matrix {
        println!("\nmatrix: \n{}", matrix)
    }
}
