/**
 * Hard Puzzle
 * There exists an undirected and unrooted tree with n nodes indexed from 0 to n - 1. You are given the integer n and a 2D integer array edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
 * 
 * Each node has an associated price. You are given an integer array price, where price[i] is the price of the ith node.
 * 
 * The price sum of a given path is the sum of the prices of all nodes lying on that path.
 * 
 * Additionally, you are given a 2D integer array trips, where trips[i] = [starti, endi] indicates that you start the ith trip from the node starti and travel to the node endi by any path you like.
 * 
 * Before performing your first trip, you can choose some non-adjacent nodes and halve the prices.
 * 
 * Return the minimum total price sum to perform all the given trips.
 * 
 *  
 * 
 * Example 1:
 * [image](https://assets.leetcode.com/uploads/2023/03/16/diagram2.png)
 * 
 * Input: n = 4, edges = [[0,1],[1,2],[1,3]], price = [2,2,10,6], trips = [[0,3],[2,1],[2,3]]
 * Output: 23
 * Explanation: The diagram above denotes the tree after rooting it at node 2. The first part shows the initial tree and the second part shows the tree after choosing nodes 0, 2, and 3, and making their price half.
 * For the 1st trip, we choose path [0,1,3]. The price sum of that path is 1 + 2 + 3 = 6.
 * For the 2nd trip, we choose path [2,1]. The price sum of that path is 2 + 5 = 7.
 * For the 3rd trip, we choose path [2,1,3]. The price sum of that path is 5 + 2 + 3 = 10.
 * The total price sum of all trips is 6 + 7 + 10 = 23.
 * It can be proven, that 23 is the minimum answer that we can achieve.
 * 
 * Example 2:
 * [image](https://assets.leetcode.com/uploads/2023/03/16/diagram3.png)
 * 
 * Input: n = 2, edges = [[0,1]], price = [2,2], trips = [[0,0]]
 * Output: 1
 * Explanation: The diagram above denotes the tree after rooting it at node 0. The first part shows the initial tree and the second part shows the tree after choosing node 0, and making its price half.
 * For the 1st trip, we choose path [0]. The price sum of that path is 1.
 * The total price sum of all trips is 1. It can be proven, that 1 is the minimum answer that we can achieve.
 *  
 * 
 * Constraints:
 * 
 * 1 <= n <= 50
 * edges.length == n - 1
 * 0 <= ai, bi <= n - 1
 * edges represents a valid tree.
 * price.length == n
 * price[i] is an even integer.
 * 1 <= price[i] <= 1000
 * 1 <= trips.length <= 100
 * 0 <= starti, endi <= n - 1
 */
type Tree = Record<number, number[]>;
type NodePair = Array<[number, number]>;
function minimumTotalPrice(n: number, edges: NodePair, prices: number[], trips: NodePair): number {
    if (!edges.length) {
        return prices[0] / 2;
    }
    const tree: Tree = {}
    for (const [a, b] of edges) {
        tree[a] = tree[a] || []
        tree[b] = tree[b] || []
        tree[a].push(b)
        tree[b].push(a)
    }
    const tripPaths = findTripPaths(n, tree, trips)

    const weights = Array.from<number>({ length: n }).fill(0);
    for (const trip of tripPaths) {
        for (const node of trip) {
            // weights[node] = occurances * prices[node]
            weights[node] += prices[node];
        }
    }

    const maxNonAdjacent = maxSumNonAdjacentNodes(tree, weights);
    return sum(weights) - maxNonAdjacent / 2;
};

function findTripPaths(n: number, tree: Tree, trips: Array<[number, number]>): number[][] {
    // Technically not trees, just using the same type
    const unvisitedTrips: Tree = {}
    const currentTrips: Tree = {}
    const visitedTripPaths = [] as number[][]

    for (const [start, end] of trips) {
        if (start === end) {
            // Kinda a stupid condition, but whatever leetcode...
            visitedTripPaths.push([start])
        } else {
            unvisitedTrips[start] = unvisitedTrips[start] || [];
            unvisitedTrips[start].push(end)
            unvisitedTrips[end] = unvisitedTrips[end] || [];
            unvisitedTrips[end].push(start)
        }
    }

    const path = [] as number[];

    walkDepthFirst(tree, (node) => {
        path.push(node)

        if (node in currentTrips) {
            for (const start of currentTrips[node]) {


                visitedTripPaths.push(
                    path.slice(path.lastIndexOf(start))
                )
            }
            delete currentTrips[node];
        }
        if (node in unvisitedTrips) {
            for (const end of unvisitedTrips[node]) {
                currentTrips[end] = currentTrips[end] || [];
                currentTrips[end].push(node)

                unvisitedTrips[end].splice(unvisitedTrips[end].indexOf(node), 1)
            }
            delete unvisitedTrips[node];
        }
    })

    // Remove any backtracking
    for (const path of visitedTripPaths) {
        let i = 0;
        while (i < path.length) {
            while (path[i] === path[i + 2]) {
                path.splice(i, 2)
                i -= 1
            }
            i += 1;
        }
    }

    return visitedTripPaths;
}

function walkDepthFirst(tree: Tree, visit: (node: number) => void) {
    let top = { index: 0, node: 0 }
    const depthStack = [] as Array<typeof top>
    const seen = [0] as number[];
    let safeGuard = 1000;
    while (safeGuard-- > 0) {
        visit(top.node)
        let next = tree[top.node][top.index]
        depthStack.push(top)
        top = { node: next, index: 0 }

        while (seen.includes(tree[top.node][top.index])) {
            while (++top.index >= tree[top.node].length) {
                visit(top.node) // Visit the node while backtracking
                const parent = depthStack.pop();
                if (!parent) {
                    return
                }
                top = parent;
            }
        }
        seen.push(top.node)
    }
}

function maxSumNonAdjacentNodes(tree: Tree, weights: number[]): number {
    return _maxSumNonAdjacentNodes(-1, 0)

    // Function/var hoisting in it's natural habitat *evil grin*
    function _maxSumNonAdjacentNodes(ignore: number, root: number): number {
        const children = tree[root].filter(node => node !== ignore)
        const bestExcludingRoot = sum(children.map(child => _maxSumNonAdjacentNodes(root, child)))
        if (!weights[root]) {
            return bestExcludingRoot;
        }
        const grandchildren = children.map(child => tree[child].filter(node => node !== root))
        const bestWithRoot = sum(grandchildren.map((gcs, i) => sum(gcs.map(gc => _maxSumNonAdjacentNodes(children[i], gc)))));
        return Math.max(
            bestExcludingRoot,
            weights[root] + bestWithRoot,
        )
    }
}

function sum(arg: number[]): number {
    return arg.reduce((sum, x) => sum + x, 0);
}

console.log("expected 29", minimumTotalPrice(5, [[2,0],[3,1],[1,0],[0,4]], [2,16,4,16,6], [[4,3]]))
console.log("expected 314", minimumTotalPrice(50, [[0,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,11],[11,12],[12,13],[13,14],[14,15],[15,16],[16,17],[17,18],[18,19],[19,20],[20,21],[21,22],[22,23],[23,24],[24,25],[25,26],[26,27],[27,28],[28,29],[29,30],[30,31],[31,32],[32,33],[33,34],[34,35],[35,36],[36,37],[37,38],[38,39],[39,40],[40,41],[41,42],[42,43],[43,44],[44,45],[45,46],[46,47],[47,48],[48,49]], [2,820,460,262,598,192,758,922,266,628,74,720,614,304,716,764,110,328,344,160,884,80,154,424,858,466,602,114,432,140,726,438,774,346,944,596,974,552,536,564,938,888,376,980,502,196,80,870,1000,998], [[9,9]]))
