# Path finding library

<b>Beginner in Rust - Feedback highly appreciated!</b>

This library will contain standard path finding algorithms and return the resulting path or graph object

- [How to use](#how-to-use)
    * [Create Graph](#create-graph)
    * [Graph operations](#graph-operations)
        + [sorted_by_weight_asc](#sorted-by-weight-asc)
        + [offer_positions](#offer-positions)
    * [Minimum spanning tree](#minimum-spanning-tree)
    * [Depth-first search](#depth-first-search)
    * [Breadth-first search](#breadth-first-search)
    * [Bidirectional breadth-first search](#bidirectional-breadth-first-search)
    * [Dijkstra path search](#dijkstra-path-search)
    * [A* path search](#a--path-search)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with
markdown-toc</a></i></small>

<b>Currently supported</b>:

- Construct graphs
- Graph operations
- Create grids
- Grid operations
- Create Minimum Spanning Tree (MST) from a graph
- Find path with Depth-First Search (DFS)
- With Breadth-First Search (BFS)
- With Bidirectional Breadth-First Search (BBFS)
- With the Dijkstra algorithm
- With the A* algorithm, with heuristic:
    - Euclidean distance
    - Manhattan distance
- TBC: with a Hierarchical Path-Finding A* (HPA*), with heuristic:
    - Euclidean distance
    - Manhattan distance

Download the crate: https://crates.io/crates/path-finding

## How to use

At the moment, we have following major concepts:

- Edge
- Node
- Graph
- Position
- Grid

You only need to pass edges to the graph. The nodes are generated automatically. Each pathfinding method will accept a
graph,
and return a graph that only contains the edges and nodes of the result.

Alternatively, you can also create a graph if you provide an adjacency matrix. Edges and nodes will be generated
automatically.

If you want to use the A* path-finding algorithm, please make sure to provide positional information for each node.

Additionally, we work on a support for each of those algorithms based on a 2D grid based structure.

### Create Graph

- Create Edge

```rust
pub fn your_function() {
    graph::Edge::from(
        0 /* edge index */,
        0 /* source node */,
        1 /* destination node */,
        0.1, /* weight */
    );
}
```

- Create Graph from edges

```rust
pub fn your_function() {
    graph::Graph::from(Vec::from([edge1, edge2]));
}
```

- Create Graph from adjacency matrix

```rust
pub fn your_function() {
    let mut matrix: &[&[f32]] = &[
        &[0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0],
        &[4.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 11.0, 0.0],
        &[0.0, 8.0, 0.0, 7.0, 0.0, 4.0, 0.0, 0.0, 2.0],
        &[0.0, 0.0, 7.0, 0.0, 9.0, 14.0, 0.0, 0.0, 0.0],
        &[0.0, 0.0, 0.0, 9.0, 0.0, 10.0, 0.0, 0.0, 0.0],
        &[0.0, 0.0, 4.0, 14.0, 10.0, 0.0, 2.0, 0.0, 0.0],
        &[0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 6.0],
        &[8.0, 11.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 7.0],
        &[0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 6.0, 7.0, 0.0]
    ];

    graph::Graph::from_adjacency_matrix(matrix);
}
```

### Graph operations

You may want to get some information or mutate the graph in some way.
Therefore, the graph currently supports three functions for convenience operations or to provide data for a heuristic
function.

#### sorted_by_weight_asc

```rust
pub fn your_function() {
    let edges: Vec<Edge> = graph.sorted_by_weight_asc(); // will return a vector with edges ascending by weight 
}
```

#### offer_positions

```rust
pub fn your_function() {
    // provide a hashmap, mapping the node id to a position - used for a* pathfinding heuristics
    graph.offer_positions(HashMap::from([(1, Position::from(0.1, 0.2, 0.3))]));
}
```

### Create Grid

- Create Grid from cost matrix

In some cases, such as 2D games, a grid based structure is advantageous. The grid can be created by passing a cost
matrix to the construction function `::from`. At each entry of the matrix you provide an unsigned integer, indicating
the effort it takes
to move to this position from any neighbouring position. In the example below moving from position `(0, 0)`to position
`(0, 1)` will require an effort, or require a cost, of 2. Equivalently, moving from `(2, 2)` to `(1, 1)` will incur a
cost of 1. Based on this information, we want to provide a support for grid based structures in each path finding
algorithm.

```rust
pub fn your_function() {
    let grid = grid::Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);
}
```

### Graph operations

#### outside
Check if a position is outside the grid. Pass a coordinate to the function below.

```rust
pub fn your_function() {
    grid.outside((0, 1));
}
```

#### within
Check if a position is within the grid. Pass a coordinate to the function below.

```rust
pub fn your_function() {
    grid.within((0, 1));
}
```

#### node_id
Convert your coordinate to a node_id

```rust
pub fn your_function() {
    grid.node_id((0, 1));
}
```

#### node_id
Retrieve the cost required to move to a provided node id

```rust
pub fn your_function() {
    grid.cost(3);
}
```


### Minimum spanning tree

```rust
pub fn your_function() {
    let mst_graph = graph::minimum_spanning(graph);
}
```

### Depth-first search

```rust
pub fn your_function() {
    let dfs = path::in_graph(
        4 /* source */,
        1 /* target */,
        &graph,
        Box::from(DepthFirstSearch {}) /* used algorithm */
    );
}
```

### Breadth-first search

```rust
pub fn your_function() {
    let bfs = path::in_graph(
        4 /* source */,
        1 /* target */,
        &graph,
        Box::from(BreadthFirstSearch {}) /* used algorithm */
    );
}
```

### Bidirectional breadth-first search

```rust
pub fn your_function() {
    let bi_bfs = path::in_graph(
        4 /* source */,
        1 /* target */,
        &graph,
        Box::from(BiBreadthFirstSearch {}) /* used algorithm */
    );
}
```

### Dijkstra path search

```rust
pub fn your_function() {
    let dijkstra = path::in_graph(
        4 /* source */,
        1 /* target */,
        &graph,
        Box::from(Dijkstra {}) /* used algorithm */
    );
}
```

### A* path search

You can use the A* path-finding algorithm by providing either an existing heuristic function as shown below. Or you
provide your own heuristic function. In case you use an existing heuristic function, make sure to provide the positional
information for the nodes.

```rust
pub fn your_function_with_euclidean_distance() {
    let a_star = path::in_graph(
        4 /* source */,
        1 /* target */,
        &graph,
        Box::from(AStar { heuristic: Box::from(euclidean_distance) }), /* used algorithm + euclidean distance heuristic function */
    );
}
```

```rust
pub fn your_function_with_manhattan_distance() {
    let a_star = path::in_graph(
        4 /* source */,
        1 /* target */,
        &graph,
        Box::from(AStar { heuristic: Box::from(manhattan_distance) }), /* used algorithm + manhattan distance heuristic function */
    );
}
```

### TBC: Hierarchical A* path search

Similar to the A* path-finding algorithm, you can provide either an existing heuristic function as shown in the previous
section. Or you
provide your own heuristic function. In case you use an existing heuristic function, make sure to provide the positional
information for the nodes.

In addition to the functionality of A*, this algorithm will require you to pass the graph to the Hierarchical A*
instance.
The reason is simple, the algorithm will divide the graph into segments on creation and cache information required in
the subsequent path-finding process.

```rust
pub fn your_function_with_euclidean_distance() {
    let a_star = path::in_graph(
        4 /* source */,
        1 /* target */,
        &graph,
        Box::from(HierarchicalAStar { heuristic, graph }), /* used algorithm and graph */
    );
}
```
