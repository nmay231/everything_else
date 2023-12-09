### Solving day XX

When visiting the page https://adventofcode.com/2023/day/1, check in the network tab for the request for the page itself (it just has the name `1`) and find the Cookie header in the request headers (it'll probably have `session=...` somewhere in it). Copy the value of the Cookie header into `cookie.txt`, then run:

```
COOKIE=$(cat cookie.txt) make init-dayXX
```

To run:

```
make run-dayXX # Or just use the cargo command directly, it's not that bad
```
