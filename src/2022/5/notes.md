### Part 1:
- Crates:
  - each crate looks like `[V]` -> which is a total width of three chars. each create is separated by a ` ` (space) char. we can assume that parsing should be done by chunking by 4.
  - if we chunk by 4, we must add by one as well (essentially we get the same output as dividing by three, except edge cases where 0 / 3 is 0 and we what we really want is 1).

- Instructions:
  - use a regex for the instruction sets, parsing for "move {value} from {origin} to {destination}

### Part 2:
-