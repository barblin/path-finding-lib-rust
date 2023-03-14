# Path finding library
<b>Beginner in Rust - Feedback highly appreciated!</b>

This library will contain standard path finding algorithms and return the resulting path or graph object

- [How to use](#how-to-use)
  * [Create Graph](#create-graph)
  * [Minimum spanning tree](#minimum-spanning-tree)
  * [Depth-first search](#depth-first-search)
  * [Breadth-first search](#breadth-first-search)
  * [Bidirectional breadth-first search](#bidirectional-breadth-first-search)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>


<b>Currently supported</b>:
- construct graphs
- create minimum spanning tree from graph
- find path with depth-first search
- find path with breadth-first search
- find path with bidirectional breadth-first search

Download the crate: https://crates.io/search?q=path-finding-lib

## How to use
At the moment, we have three major concepts:
- Edge
- Node
- Graph

You only need to pass edges to the graph. The nodes are generated automatically. Each pathfinding method will accept a graph,
and return a graph that only contains the edges and nodes of the result.

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

### Minimum spanning tree
```rust
pub fn your_function() {
    let mst_graph = graph::minimum_spanning(graph);
}
```

### Depth-first search
```rust
pub fn your_function() {
    let dfs = path::find(
        4 /* source */, 
        1 /* target */, 
        &graph, 
        Box::from(DepthFirstSearch {}) as Box<dyn PathFinding> /* used algorithm */
    );
}
```

### Breadth-first search
```rust
pub fn your_function() {
    let dfs = path::find(
        4 /* source */, 
        1 /* target */, 
        &graph, 
        Box::from(BreadthFirstSearch {}) as Box<dyn PathFinding> /* used algorithm */
    );
}
```

### Bidirectional breadth-first search
```rust
pub fn your_function() {
    let dfs = path::find(
        4 /* source */, 
        1 /* target */, 
        &graph, 
        Box::from(BiBreadthFirstSearch {}) as Box<dyn PathFinding> /* used algorithm */
    );
}
```