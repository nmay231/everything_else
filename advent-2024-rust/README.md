### Solving day XX

1. Copy your cookie to a local file. When visiting the page
   https://adventofcode.com/2024/day/1, check in the network tab for the request
   for the page itself (it just has the name `1`) and find the Cookie header in
   the request headers (it should have `session=...` somewhere in it). Copy the
   value of the Cookie header into `cookie.txt` in the same directory as this
   README.

2. Build the boilerplate for that day's puzzle.

```
just init XX
```

3. Run/test the program for that day's puzzle. (though the cargo command to run it is
   really not that hard).

```
just run XX
just run XX -- ... # Pass args to binary
just test XX
just test XX -- PAT # Pass args to test binary (Here, we filter tests to match PAT)
```
