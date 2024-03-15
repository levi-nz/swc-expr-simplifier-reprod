# swc-expr-simplifier-reprod
swc bug re-production repo.

## Run
`cargo run`

Output should be `6;`, but is currently `3 * (2);` (unchanged from `input`) due to the bug.
