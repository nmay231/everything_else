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

// TODO: Version that seems to work but is too slow and times-out
// I mean, it's not like repeatedly searching the tree breadth first has anything to do with it...

type Tree = Record<number, number[]>;
type NodePair = Array<[number, number]>;
function minimumTotalPrice(n: number, edges: NodePair, prices: number[], trips: NodePair): number {
    const tree: Tree = {}
    for (const [a, b] of edges) {
        tree[a] = tree[a] || []
        tree[b] = tree[b] || []
        tree[a].push(b)
        tree[b].push(a)
    }

    const weights = Array.from<number>({ length: n }).fill(0);
    for (const trip of trips) {
        // weights[node] = occurances * prices[node]
        const path = breadthFirst(tree, trip)
        for (const node of path) {
            weights[node] += prices[node];
        }
    }
    const maxNonAdjacent = maxSumNonAdjacentNodes(tree, weights);
    return sum(weights) - maxNonAdjacent / 2;
};

function maxSumNonAdjacentNodes(tree: Tree, weights: number[]): number {
    return _maxSumNonAdjacentNodes(-1, 0)

    // Function/var hoisting in it's natural habitat *evil grin*
    function _maxSumNonAdjacentNodes(ignore: number, root: number): number {
        const children = tree[root].filter(node => node !== ignore)
        const grandchildren = children.map(child => tree[child].filter(node => node !== root))
        return Math.max(
            sum(children.map(child => _maxSumNonAdjacentNodes(root, child))),
            weights[root] + sum(grandchildren.map((gcs, i) => sum(gcs.map(gc => _maxSumNonAdjacentNodes(children[i], gc)))))
        )
    }
}

function sum(arg: number[]): number {
    return arg.reduce((sum, x) => sum + x, 0);
}

function breadthFirst(tree: Tree, trip: [number, number]): number[] {
    const [start, end] = trip;
    if (start === end) return [start]
    let neigh = tree[start]
    const dequeue = [{root: start, ignore: undefined as undefined | number, path: [start]}]

    while (dequeue.length) {
        const {root, ignore, path} = dequeue.shift()!
        for (const neigh of tree[root]) {
            if (neigh === end) {
                return [...path, neigh]
            }
            if (neigh !== ignore) {
                dequeue.push({root: neigh, ignore: root, path: [...path, neigh]})
            }
        }
    }
    throw Error("Shoulda gotten it by now...")
}
