This was developed during this [Live Stream](https://www.youtube.com/live/Y8859XCokBs).

# How to use SIMD in Rust

This is meant to be starting tutorial, not really
advanced but enough to get most people started.

This is not just a sum exaple, we will try to
do something a little more involved.

# The problem

Let's say we have a dataframe with N columns
that we want to filter by a single value.

Col1: [1, 2, 3, 1, 1, 2, 3, 3, 3, 4]
Col2: [1, 1, 1, 1, 1, 1, 1, 1, 1, 3]
Col3: [2, 2, 2, 2, 2, 1, 2, 2, 2, 2]
Col4: [4, 4, 4, 4, 4, 4, 4, 4, 4, 4]
 
We want to get the row numbers where
the value is equal to 1 in any of the
columns.

In R it would look something like this:

```R
df$col1 == 1 | df$col2 == 1 | df$col3 ...
```

# How to run

```sh
RUSTFLAGS='-C target-feature=+avx2' cargo run --release
