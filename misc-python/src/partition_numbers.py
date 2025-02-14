"""
I finally know that this alternate method of calculating partition numbers has
already been discovered, and that the appropriate term for these partial
sequences are Durfee squares (https://en.wikipedia.org/wiki/Durfee_square).

This realization brings about a deep sense of closure. It might not make sense
to you, but every once in a while I went back to these formulas and kept trying
to formalize this into a closed form for calculating partition numbers. While it
might still be possible that this is the required seed to find that closed form,
I no longer feel like I *have* to find it and publish it because the base of
knowledge that I thought was my own is already public. A bit silly, I know, but
this is a topic I've thought about since I was a pre-teen a decade ago.

In any case, my approach seems to be an inverse of their definition; in fact, my
method of getting these sequences are basically the opposite. This shows that
two different aspects of partitions are equivalent which is interesting in its
own right even if we come no closer to a closed form. Therefore, let me share
this equivalence.

First off, some brief definitions. Partitions are ways of summing up positive
integers to equal `n`, without caring about the order of the summands. The
partition numbers `p(n)` describe the total number of unique partitions of `n`.
The Ferrers diagram of a partition represents it as bars of dots (or characters)
where the summands determine the length. Here's the diagram for 32=9+9+5+5+2+2.

& & & & # # # # #
& & & & # # # # #
& & & & #
& & & & #
# #
# #

Note that it is a matter of preference and convention whether the rows or
columns represent the summands, and whether they are left- or right-aligned. My
example is left-aligned with a row for each summand.

The Durfee square of a partition can be defined as the largest square that can
fit into the Ferrers diagram. The rank of a partition is defined as the
side-length of its Durfee square. My example would have a 4x4 Durfee square
(marked with ampersands `&`) making that partition rank 4.

With these definitions, we can now calculate partition numbers `p(n)` by adding
up the number of partitions of `n` with rank 1, then those of rank 2, all the
way up to rank `floor(sqrt(n))` (since that's the largest rank a partition of
`n` can have).

Skipping ahead a bit, the generating function for partitions of rank `k` is:

x^(k^2) / prod(i = 1 to k, (1 - x^i)^2)

So by summing up these values for all valid ranks `k` of `n` and finding the
coefficient of `x^n`, you can calculate partitions of `n`

How does my findings relate to this? Well I realized that to count the number of
partitions of n, you can try counting the difference between `p(n) - p(n - 1)`,
and then you get the values `p(n) - p(0)` when you take the series of that
sequence making `p(n)` trivial to calculate by adding `p(0) = 1`.

Well, how do we calculate this sequence difference? The insight is to realize
that there is a bijection between *all* partitions of `n` that end in `... + 1`
and the partitions of `n - 1`. In other words, every partition of `n - 1`, let's
call it `a_1 + ... + a_k` can be *uniquely* mapped to a partition of `n` by
adding one: `a_1 + ... + a_k + 1`.

Because this mapping is one-to-one, we can now say that the difference `p(n) -
p(n - 1)` can be thought of as counting the partitions of `n` with all terms
greater than one (aka, there are no `+ 1`s).

Let's call this difference `D'(n, 1) = p(n) - p(n - 1)`. And yes the prime `'`
and 1 are important because we are actually going to take the difference of this
sequence again. Let `D(n, 1) = D'(n, 1) - D'(n - 1, 1)`. We can call `D(n, 1)`
the second difference of the partition numbers.

But how do we interpret what this difference means? Well, we can apply a trick
to make understanding the interpretation easier. We will think about the
conjugate of a partition. The *conjugate* can be defined by transposing the
Ferrers diagram, aka switch the columns and rows, and converting back into a
sum. Using the example from above, the conjugate of `9+9+5+5+2+2` is
`6+6+4+4+4+2+2+2+2`.

To interpret the difference `D(n, 1)`, we look at the conjugate of every
partition counted by `D'(n, 1)` and `D'(n - 1, 1)` and notice that we have a
very similar situation with `p(n)` and `p(n - 1)` from earlier. Every partition
counted by `D'(n - 1, 1)` can be mapped to partitions of `D'(n, 1)` where the
latter partition is the former plus one (`... + 1`).

So, `D(n, 1)` counts the partitions fully composed by terms greater than `1` and
where the conjugate is composed of terms greater than `1`. I would like to
describe this property as "having an *elbow width* of 2 or greater". So, the
elbow width of a partition is the minimum of the terms of itself and the terms
of its conjugate. Below is a visualization of the "elbow" of 9+9+5+4+3.

& & & & & & & & &|
& & & & & & & & &|
& & # # #
& & # #
& & #
- - -

Unfortunately, taking the difference of this new sequence is useless since we
can't apply the same trick. Or can we?

We can if we take the difference between further apart values skipping a value
in the difference `D'(n, 2) = D(n, 1) - D(n - 2, 1)`. We get a similar mapping
between the partitions counted by `D(n - 2, 1)` and those counted by `D(n, 1)`
with at least one `... + 2` term. This means `D'(n, 2)` counts the number of
terms where the minimum is 3 (and the minimum of its conjugate is 2). Taking the
difference again while considering the conjugate and we get the symmetrical
definition `D(n, 2) = D'(n, 2) - D'(n - 2, 2)` which counts the number of
partitions of `n` with an elbow width of 3 or greater.

This process can be repeated indefinitely giving the recursive definition of
`D(n, i + 1) = D(n, i) - 2*D(n - i, i) + D(n - 2*i, i)` where `D(n, 0) = p(n)`
and `D(n, i)` counts the number of partitions of `n` with elbow width `i + 1` or
greater.

How does this relate to Durfee squares? Well, elbow widths have a simple
relation with square numbers. When `n = i^2` is a perfect square, there is
exactly one partition with elbow width `i` or greater; the one of a literal
square. So `D(i^2, i + 1) = 1`. And of course `D(k, i + 1) = 0` for `k < i^2`.

This allows us to build a table of the values `D(n, i)` with `i` being the row
starting at `i = 0`. We start with "seeds" of 1 at the square numbers and zeros
everywhere below and to the left.

| 1 2 3 4 5 | 6 7 8 9 10 | 11 12 13 14 15 | 16 17 18 19 20 | 21 22 23 24 25 | 26 27 28 29 30 | 31 32 33 34 35 | 36 37 38 39 40 | 41 42 43 44 45 | 46 47 48 49 50 | 51 52 53 54 55 | 56 57 58 59 60 | 61 62 63 64 65 |
| --------- | ---------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- |
|           |            |                |                |                |                |                |                |                |                |                |                |                |
| 1         |            |                |                |                |                |                |                |                |                |                |                |                |
| 0 0 0 1   |            |                |                |                |                |                |                |                |                |                |                |                |
| 0 0 0 0 0 |  0 0 0 1   |                |                |                |                |                |                |                |                |                |                |                |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  1             |                |                |                |                |                |                |                |                |                |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  1 |                |                |                |                |                |                |                |                |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  1             |                |                |                |                |                |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  1          |                |                |                |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |

Then, start at the lower rows, add to each row above at x1 to the entry directly
above itself, then x2 at the appropriate gap to the right, then x3 at that much
to the right again and so on. Pluses (+) are placed before the seeds for clarity.

| 1 2 3 4 5 | 6  7  8  9 10 |11 12  13  14  15 | 16  17  18  19  20 | 21   22   23   24   25 |  26   27   28   29   30 |  31   32    33    34    35 |   36    37    38    39    40 |   41    42    43    44    45 |    46     47     48     49     50 |    51     52     53     54     55 |    56     57     58     59     60 |     61      62      63      64      65 |     66      67      68      69      70 |     71      72      73      74      75 |     76       77       78       79       80 |      81       82       83       84       85 |      86       87       88       89       90 |      91       92       93       94        95 |       96        97        98        99       100 |
| --------- |-------------- |----------------- |------------------- |----------------------- |------------------------ |--------------------------- |----------------------------- |----------------------------- |---------------------------------- |---------------------------------- |---------------------------------- |--------------------------------------- |--------------------------------------- |--------------------------------------- |------------------------------------        |---------------------------------------------|-------------------------------------------- |--------------------------------------------- |------------------------------------------------- |
| 1 2 3 5 7 |11 15 22 30 42 |56 77 101 135 176 |231 297 385 490 627 |792 1002 1255 1575 1958 |2436 3010 3718 4565 5604 |6842 8349 10143 12310 14883 |17977 21637 26015 31185 37338 |44583 53174 63261 75175 89134 |105558 124754 147273 173525 204226 |239943 281589 329931 386155 451276 |526823 614154 715220 831820 966467 |1121505 1300156 1505499 1741630 2012558 |2323520 2679689 3087735 3554345 4087968 |4697205 5392783 6185689 7089500 8118264 |9289091 10619863 12132164 13848650 15796476 |18004327 20506255 23338469 26543660 30167357 |34262962 38887673 44108109 49995925 56634173 |64112359 72533807 82010177 92669720 104651419 |118114304 133230930 150198136 169229875 190569291 |
|+1 0 0 1 0 | 2  0  3  1  4 | 2  7   3  10   7 | 14  11  22  17  32 | 28   45   43   67   63 |  95   96  134  139  192 | 199  269   287   373   406 |  521   566   718   792   983 | 1092  1346  1496  1827  2045 |  2465   2772   3323   3733   4449 |  5016   5929   6696   7882   8897 | 10426  11784  13735  15534  18047 |  20391   23613   26692   30788   34797 |  40034   45207   51877   58564   67013 |  75614   86341   97328  110905  124953 | 142063   159945   181529   204185   231340 |  260025   294077   330286   372977   418506 |  471908   529106   595725   667380   750432 |  839938   943262  1054922  1183173   1322156 |  1481186   1653741   1850580   2064533   2307677 |
| 0 0 0+1 0 | 0  0  0  1  0 | 0  2   0   0   3 |  1   0   4   2   2 |  5    3    4    9    5 |   6   13   11   10   19 |  17   19    28    27    31 |   44    41    49    66    68 |   74    98   104   118   145 |   157    178    220    234    268 |   322    354    397    473    521 |   591    686    765    863   1003 |   1107    1254    1444    1609    1804 |   2071    2305    2597    2947    3293 |   3693    4192    4664    5236    5911 |   6594     7367     8308     9248    10345 |   11600    12926    14421    16163    17959 |   20031    22380    24886    27674    30890 |   34284    38123    42426    47081     52250 |    58102     64351     71381     79207     87703 |
| 0 0 0 0 0 | 0  0  0 +1  0 | 0  0   0   0   0 |  1   0   0   0   2 |  0    0    0    3    1 |   0    0    4    2    2 |   0    5     3     4     3 |    7     4     6     6    13 |    7    10     9    19    15 |    17     16     28     24     30 |    27     43     39     49     47 |    65     62     77     78    104 |     98     119     124     160     159 |    186     194     243     249     292 |    303     369     383     447     474 |    559      588      678      724      847 |     895     1024     1098     1271     1355 |    1534     1654     1894     2033     2293 |    2471     2806     3026     3399      3678 |     4140      4473      5004      5429      6082 |
| 0 0 0 0 0 | 0  0  0  0  0 | 0  0   0   0   0 | +1   0   0   0   0 |  0    0    0    0    1 |   0    0    0    0    2 |   0    0     0     0     3 |    1     0     0     0     4 |    2     2     0     0     5 |     3      4      3      1      6 |     4      6      6      6      9 |     7      8      9     11     17 |     13      14      15      17      25 |     25      24      27      29      39 |     39      43      44      49      62 |     64       71       76       79       98 |     102      115      124      136      153 |     164      182      199      218      249 |     261      289      315      347       390 |      422       454       499       544       608 |
| 0 0 0 0 0 | 0  0  0  0  0 | 0  0   0   0   0 |  0   0   0   0   0 |  0    0    0    0   +1 |   0    0    0    0    0 |   0    0     0     0     0 |    1     0     0     0     0 |    0     2     0     0     0 |     0      0      3      1      0 |     0      0      0      4      2 |     2      0      0      0      5 |      3       4       3       1       0 |      6       4       6       6       6 |      2       9       5       8       9 |     11        9       15       10       13 |      13       16       16       27       19 |      24       23       27       25       41 |      35       40       41       47        45 |       64        58        68        68        77 |
| 0 0 0 0 0 | 0  0  0  0  0 | 0  0   0   0   0 |  0   0   0   0   0 |  0    0    0    0    0 |   0    0    0    0    0 |   0    0     0     0     0 |   +1     0     0     0     0 |    0     0     0     0     0 |     0      0      0      1      0 |     0      0      0      0      0 |     2      0      0      0      0 |      0       0       3       1       0 |      0       0       0       0       4 |      2       2       0       0       0 |      0        5        3        4        3 |       1        0        0        6        4 |       6        6        6        2        2 |       7        5        8        9        11 |        9         7        12         9        10 |
| 0 0 0 0 0 | 0  0  0  0  0 | 0  0   0   0   0 |  0   0   0   0   0 |  0    0    0    0    0 |   0    0    0    0    0 |   0    0     0     0     0 |    0     0     0     0     0 |    0     0     0     0     0 |     0      0      0     +1      0 |     0      0      0      0      0 |     0      0      0      0      0 |      0       0       0       1       0 |      0       0       0       0       0 |      0       2       0       0       0 |      0        0        0        0        3 |       1        0        0        0        0 |       0        0        4        2        2 |       0        0        0        0         0 |        5         3         4         3         0 |
| 0 0 0 0 0 | 0  0  0  0  0 | 0  0   0   0   0 |  0   0   0   0   0 |  0    0    0    0    0 |   0    0    0    0    0 |   0    0     0     0     0 |    0     0     0     0     0 |    0     0     0     0     0 |     0      0      0      0      0 |     0      0      0      0      0 |     0      0      0      0      0 |      0       0       0      +1       0 |      0       0       0       0       0 |      0       0       0       0       0 |      0        0        0        0        0 |       1        0        0        0        0 |       0        0        0        0        2 |       0        0        0        0         0 |        0         0         0         3         0 |
| 0 0 0 0 0 | 0  0  0  0  0 | 0  0   0   0   0 |  0   0   0   0   0 |  0    0    0    0    0 |   0    0    0    0    0 |   0    0     0     0     0 |    0     0     0     0     0 |    0     0     0     0     0 |     0      0      0      0      0 |     0      0      0      0      0 |     0      0      0      0      0 |      0       0       0       0       0 |      0       0       0       0       0 |      0       0       0       0       0 |      0        0        0        0        0 |      +1        0        0        0        0 |       0        0        0        0        0 |       0        0        0        0         0 |        0         0         0         0         0 |

Looking at the rows above each seed, you can observe the sequence 1,2,3,4,...
separated by zeros of constant length per row where the gap increases by one the
lower you go. Of course, the pattern is there until the row below the seed
starts causing interference.

| 1 2 3 4 5 | 6 7 8 9 10 | 11 12 13 14 15 | 16 17 18 19 20 | 21 22 23 24 25 | 26 27 28 29 30 | 31 32 33 34 35 | 36 37 38 39 40 | 41 42 43 44 45 | 46 47 48 49 50 | 51 52 53 54 55 | 56 57 58 59 60 | 61 62 63 64 65 |
| --------- | ---------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |
| 0 0 0 0 0 |  0 0 0 0 0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |  0  0  0  0  0 |

Full disclosure, this property that 

Funny note, I just started a google search
"""

from functools import lru_cache

import numpy as np


def fill_diagonal(arr: np.ndarray, diag_index: int, fill: int):
    """Given the `diag_index` (positive points down-left) fill the arr with `fill`"""
    assert diag_index >= 0  # TODO
    assert arr.shape[0] == arr.shape[1]  # Must be square
    len_ = len(arr)
    arr[np.arange(diag_index, len_), np.arange(len_ - diag_index)] = fill


def slope(n: int, spacing: int):
    """Return a n by n matrix with each lower-half diagonal filled by incrementing
    numbers with `spacing` gaps between each diagonal, e.g.

    >>> slope(6, 2)
    [[1. 0. 0. 0. 0. 0.]
     [0. 1. 0. 0. 0. 0.]
     [2. 0. 1. 0. 0. 0.]
     [0. 2. 0. 1. 0. 0.]
     [3. 0. 2. 0. 1. 0.]
     [0. 3. 0. 2. 0. 1.]]
    """
    arr = np.zeros((n, n))
    for fill, diag_index in zip(range(1, n + 1), range(0, n, spacing)):
        fill_diagonal(arr, diag_index, fill)
    return arr


@lru_cache
def partitions_of_rank(rank: int, up_to_n: int = 100) -> np.ndarray:
    result = np.zeros(up_to_n)
    result[0] = 1
    for i in range(1, rank + 1):
        result = slope(len(result), i) @ result
        # result = result[: -(2 * i + 1)]
    result = result.astype(int)
    return np.pad(result, [(rank**2 - 1, 0)], mode="constant", constant_values=0)


def partitions_up_to(n: int) -> np.ndarray:
    """Returns the partition numbers `[p(1), ... p(n)]`"""
    sequence_of_differences = np.zeros(n)
    sequence_of_differences[0] = 1
    result = np.zeros(n)
    for i in range(1, int(n**0.5) + 1):
        sequence_of_differences = (
            slope(len(sequence_of_differences), i) @ sequence_of_differences
        )
        result[i**2 - 1 :] += sequence_of_differences
        sequence_of_differences = sequence_of_differences[: -(2 * i + 1)]
    return result


def euler_partitions(n: int) -> np.ndarray:
    """Returns the partition numbers `[p(0), ... p(n)]`"""
    indexes = np.zeros(n)
    for i, element in enumerate(pentagonal(n)):
        # +q_1 +q_2 -q_3 -q_4 +a_5 +a_6 -a_7 ...
        indexes[n - element] = 1 if not i & 2 else -1

    result = np.zeros(n + 1)
    result[0] = 1

    for i in range(1, n + 1):
        result[i] = result[:i].dot(indexes[-i:])

    return result


def pentagonal(n: int):
    """Generate *generalized* pentagonal numbers up to `pentagonal(i) <= n`"""
    pent = 0
    for i in range(n):
        for large in [True, False]:
            pent += 2 * i + 1 if large else i + 1
            if pent > n:
                return
            yield pent


@lru_cache(1000)
def brute_force_partitions(n: int, max_: int | None = None) -> list[tuple[int, ...]]:
    if n == 0:
        return [(0,)]
    assert n > 0 and int(n) == n, "partitions only defined for natural numbers"

    results: list[tuple[int, ...]] = []
    if max_ is None:
        for i in range(1, n + 1):
            results.extend(brute_force_partitions(n, i))
        return results
    elif n == max_:
        return [(n,)]

    start = (max_,)
    n -= max_
    for i in range(1, min(n + 1, max_ + 1)):
        for end in brute_force_partitions(n, i):
            results.append(start + end)
    return results


if __name__ == "__main__":

    N = 100
    stack = np.zeros(N)
    row = np.zeros(N)
    for i in range(9, 0, -1):
        row[i**2 - 1] = 1
        row = slope(N, i) @ row
        stack = np.vstack([row, stack])
    stack = stack.astype(int)

    for i in range(1, len(stack)):
        # These are the seeds at the square numbers
        stack[i, i**2 - 1] = 1

    widths = [max(len(string) for string in col) for col in stack.astype(str).T]

    def print_row(row: list[str], group_size=5) -> None:
        print("| ", end="")
        for i in range(0, len(row), group_size):
            group = row[i : i + group_size]
            group_widths = widths[i : i + group_size]
            print(
                " ".join(
                    " " * (width - len(x)) + x for x, width in zip(group, group_widths)
                ),
                "|",
                end="",
            )
        print()

    print_row(list(map(str, range(1, N + 1))))
    for row in stack.astype(str):
        print_row(row)


# TODO: Calculate the difference in computation cost between both methods
def flops_euler_method(n: int):
    """How many flops to calculate partitions up to n using Euler's pentagonal method"""
    assert n > 1
    total_flops = 0
    flops_in_current_block = 1
    for index in range(1, n):
        for is_large_step in [True, False]:
            block_size = 2 * index - 1 if is_large_step else index

            if block_size >= n:
                return total_flops + flops_in_current_block * n
            n -= block_size

            total_flops += flops_in_current_block * block_size
            flops_in_current_block += 1


if __name__ == "__main__":
    # Then I could work on calculating the flops of each, probably by using rust and simply incrementing a number instead of trying to be smart about it, but then that means I actually have to look at the compiled output if I want to be completely accurate.
    assert [*pentagonal(40)] == [1, 2, 5, 7, 12, 15, 22, 26, 35, 40]
