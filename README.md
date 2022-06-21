# minimum-edit-distance
A CLI implementing the minimum edit distance algorithm in rust.

## building and running

1. set up rust [the raw way](https://doc.rust-lang.org/book/ch01-01-installation.html) or [install nix](https://nixos.org/manual/nix/stable/installation/installing-binary.html) and start a `nix-shell`
2. `cargo run -- --help`

for example:
`cargo run -- --from common --to coming --matrix`

You can of course just run the executable directly as well.

## explanation

The _minimum edit distance_ for two strings is the minimum number of _insertion, deletion, or replacement_ operations 
required to convert the first string to the second. It is generally calculated using a grid, 
where each value in the cells is the minimum edit distance for the substring that ends at that column and row.
For example:

<img width="673" alt="image" src="https://user-images.githubusercontent.com/8580352/174901058-08643009-5d4e-4b53-9eed-7002db161a5a.png">

In this case, 4 is the minimum edit distance.

## steps to build the grid

1. first, the initial cells for header column and row (`grid[header_row][1]` and `grid[first_row][0]`) represent an empty string, `""`, or `#` in NLP lingo. 
  the cost for these is 0.
1. from the initial 0 cell, for each word, add the accumulating insertion cost for each character (i.e. _1, 2, 3, 4...length of word_).
1. for each of the empty cells, the cost of each operation is found in adjascent cells:
  <img width="677" alt="image" src="https://user-images.githubusercontent.com/8580352/174904420-0764af7a-12c8-4cf4-81a8-981638a107f6.png">
   if the letters in the columns are the same, just copy the cell labelled "cost of replacement", 
   otherwise pick the cell with the lowest value, add it's associated cost (cost of insert, cost of delete, or or cost of replacement) and use that
