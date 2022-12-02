### Part 1:
```
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```
->

elf 1:
```
1000
2000
3000
```

elf 2:
```
4000
```

elf 3:
```
5000
6000
```

elf 4:
```
7000
8000
9000
```

elf 5:
```
10000
```

- discriminate elves by two consecutive `\n` (new line char) i.e. .skip('\n\n')

- discriminate food items by `\n` (new line char)
- collect sum of numbers (how big? default to an unsized and see if you can go smaller. rust will panic if fails to parse)

- compare elf totals to find the elf carrying the most calories
  - do we have to do that retrovactively? i guess we can sort after we collect but why don't we approach is by creating a vec and pushing the sums in order as we read the input?
  - wait, no. too much work. just do the built in max() function for now lol

### Part 2:

- obviously, we sort -> find the top three -> collect the sum
  - how do we sort though. why not traverse through the list -> keep an array of 3 max length and push/pop accordingly
  - or, use built in rust functions like max(), reverse(), collect(), and sum()?