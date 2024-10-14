### Kami 2 Solver

Here is my attempt to write a fully automatic solver for it. The main script
([main.py](./src/main.py)) takes the path to a screenshot of a puzzle, solves
it, and displays a copy of the screenshot with the steps needed to solve
the puzzle. While there are still aspects that I want to account for, the puzzle
itself and the colors used are automatically detected without user input.

### What is Kami 2

Kami 2 is a fun puzzle game available on Android (and IOS I guess).
https://play.google.com/store/apps/details?id=com.stateofplaygames.kami2&hl=en_US.

The premise of the game is very similar to the classic [Flood Fill](https://www.chiark.greenend.org.uk/~sgtatham/puzzles/js/flood.html)
puzzle, with a few differences: the grid is a triangular grid (which matters
very little in the end), but more importantly you choose the cell that is
flood-filled every time instead of it always being the top-left cell. This
drastically increases the difficulty of solving the puzzle. The drastically
increases the operation time of a brute force solution from `O(n_colors ^
n_moves)` to `O((n_cells * n_colors) ^ n_moves)`; And this is much worse than
just squaring the previous operation time since `n_colors` is typically no more
than 5 while `n_cells` is often more than 10 and can easily be 50+.

### What the solver does

A Kami 2 puzzle is a grid of colored cells where you selectively flood-fill (FF)
sections of cells that are the same color changing them to a new color until the
entire grid is the same color. Since it is always possible to FF a grid until
the grid is uniform, the real puzzle is to do it in the minimum number of moves.

In the app itself, you are given the grid and the minimum number of moves
required to solve the puzzle. With that information, it would be relatively easy
to brute force the solution by always pruning searches with more moves than the
given minimum and a few heuristics, e.g. prefer vertices in the center of the
grid, or ones with lots of neighbors of the same color. Therefore, my solver is
actually trying to prove that the given number of minimum moves is actually the
minimum, and provide a solution in the process.

#### How the script works (At the time of writing, at least)

##### The main script

1. First, we crop the screenshot of the puzzle to include only the grid.
2. Then we build a colored graph initially with a vertex for each cell.
3. Since Kami 2 applies a paper-like texture to the puzzle (to make it look like
   origami paper), the cells are actually made of many very similar colors. To
   account for that, we average the colors of a square sample from each cell, and
   then we apply clustering to all of the colors to find the 2-5 dominant colors.
4. We merge adjacent cells of the same color, and then pass the proper-colored
   graph to the solver.
5. We take a minimum solution given by the solver and display an annotated and
   colored screenshot with the correct steps to take.

##### The solver

While not crucial to how this solver works, it is implemented as a recursive
generator function (generators are lazy iterators in Python). The objects
returned from the iterator are `SolverStep` objects that contain information
about the current search. The reason it is written this way is because I want to
eventually write other general solvers that can be paused at any time, i.e. they
return control back to the caller in steps and generally have an upper bound on
the amount of time between each step. This would allow the solver to be written
in a single threaded application (e.g. JS in the browser) while still being
responsive to a user's input. Or even if the solver is in a separate thread, you
can still "ping" it and know it is not caught in a unforeseen infinite loop. You
can also include information in the returned steps to make the solver
more debuggable without relying on dense logs.

Anyways, I've thought about how to limit the search space for the solver, and I
cannot seem to find anything that reduces the search from being exponential. I
have made optimizations that reduce duplicate solve attempts without eating up
all my RAM. Since the search is DFS, we can set a ceiling on the minimum number
of moves required and lower this ceiling as we find better solutions, while
pruning searches that take more than that number of moves. Since there is a lot
of independent actions we can take, e.g. FFing `A` then `B` or `B` then `A`, we
reduce that duplicate work by ordering the nodes and colors and only applying
actions in the right order if they are truly independent. We can also optimize
by not FFing high-eccentricity nodes, e.g. nodes with a large maximum distance
to some other node in the graph.

### Tentative TODOs

- [ ] Account for blank cells in the grid (cells that don't need to be FF'ed)
- [ ] Account for stretching/scaling of the screenshot. Currently, the numbers
      are currently just manually set in the code which might not work for
      screenshots from devices with different aspect ratios.
- [ ] The current clustering algorithm just guesses what the number of colors is
      based on how similar the colors are. However, we should be able to know
      how many colors there are because of the controls shown on the bottom of
      the screenshot.
- [ ] I believe Kami 2 might just layer a transparent image on top of the puzzle
      grid to add interesting noise, i.e. it's predetermined for each cell. So,
      instead of averaging the color of each cell and clustering the colors by
      similarity, I might be able to apply preprocessing to get uniform colors.
      I probably still have to do some clustering since it might not be
      perfectly invertible, but it would certainly squeeze the bell curve to
      more acceptable levels.
