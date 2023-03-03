# Path finding library

This library will contain standard path finding algorithms and return the resulting path or graph object

- [How to use](#how-to-use)
    * [Create Graph](#create-graph)
    * [Minimum spanning tree](#minimum-spanning-tree)
    * [Depth-first search](#depth-first-search)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>

<b>Currently supported</b>:
- construct graphs
- create minimum spanning tree from graph
- find path with depth-first search

Download the crate: https://crates.io/search?q=path-finding-lib

## How to use

### Create Graph

- Create Edge
```rust
pub fn your_function() {
    graph::Edge::from();
}
```

- Create graph

```rust
pub fn your_function() {
    graph::Graph::from(Vec::from([edge1, edge2]));
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
    let dfs = path::find(4, 1, &graph, Box::from(DepthFirstSearch {}) as Box<dyn PathFinding>);
}
```

