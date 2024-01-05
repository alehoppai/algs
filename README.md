# Algs

This year i promised myslef to learn new language. Meanwhile I was looking for a new job, and I had to remember basic algorithms for my tech interviews. So I decided to combine those two things using Rust.

## Cheat sheet for algs presented in this repo

### Binary search `./src/bin_search.rs` O(log n)

Search alg on a sorted list.
Given sorted list of integers, and item to lookup for.
Should return index of an element or nothing;

1. Define `low` and `high`. With zero and last elm indexes.
2. Loop while `low <= high`.
3. Each iteration:
    1. Calculate `mid` index by adding `low` to `high`;
    2. Getting value from `list` by `mid` index;
    3. Comparing this `mid value` to input.
        1. If we get a match => return `mid` index;
        2. If `mid value` lesser than input => set `low` value to increased `mid` index.
        3. If `mid value` greater than input => set `high` value to decreased `mid` index.
4. Not found -> return nothing;

### Selection sort `./src/selection_search.rs` O(n**2)

Sorting list by picking smallest value on each sub iteration.
Given list.
Should return sorted list.

This algorithm consits of 2 tasks.

#### 1. Iterate over list to fullfil sorted array which should be return

1. Declaring new array with capacity of orignial.
2. Iterate over original array.
3. On each iteration call second task to find smallest value.
4. Push smallest found value to new array.
5. Remove smallest value from original array.

#### 2. On each iteration iterate over list again to pick smallest value

1. Declare two vars, first should be same as value of first element of input array, second track by you index value defined to zero.
2. Loop through input array starting 1st element, not 0.
3. On each iteration compare check if `list[i]` lesser than value.
    1. If True -> reassign value to new `list[i]` and update index controlled by you to index from current iteration;
    2. If False -> do nothing;
4. return controlled index.
