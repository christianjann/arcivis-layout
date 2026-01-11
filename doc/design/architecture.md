# Design

## Core Components

- **Node**: Represents a graph vertex with position, size, and optional ports
- **Edge**: Represents a graph connection with source/target indices and routed waypoints
- **Port**: Optional connection points on nodes (up to 8 per node) with input/output types
- **Grid**: Discrete spatial partitioning system for pathfinding and collision detection

## Key Features

- **Orthogonal Edge Routing**: All edges follow horizontal and vertical paths only
- **Port-Aware Connections**: Intelligent port selection based on connection direction
- **Obstacle Avoidance**: Automatic routing around nodes and other edges
- **Force-Directed Layout**: Physics-based node positioning for natural-looking layouts
- **Configurable Parameters**: Adjustable forces, spacing, and iteration counts
- **Grid-Snapping**: Positions are snapped to a 5-unit grid for consistency

## Technical Details

- **Grid Cell Size**: 5.0 units (configurable)
- **Extension Distance**: Dynamic distance from ports to nearest non-obstacle cell (was fixed 25.0 units)
- **Pathfinding**: Breadth-First Search (BFS) on orthogonal grid
- **Force Iterations**: Default 100 iterations for convergence
- **Minimum Spacing**: 20.0 units between nodes

### Port Selection Logic

When nodes have defined ports:
1. Calculate direction vector from source node center to target node center
2. Evaluate each port's position relative to node center
3. Select the port whose directional vector best matches the connection direction
4. Fall back to node center (grid-snapped) if no ports are defined

## Grid System

- **Origin Alignment**: Grid origin is aligned to multiples of cell size for precision
- **Obstacle Mapping**: Node bodies and port areas are marked as obstacles
- **Path Reconstruction**: A* paths are converted back to continuous coordinates
- **Boundary Handling**: Grid automatically expands to cover all nodes with padding

## API Options

The crate provides two API approaches for different use cases:

### Ownership-Based API
```rust
let layout = ArciVisLayout::default();
let result = layout.layout(nodes, edges);
// Returns new LayoutResult with positioned nodes and routed edges
```

### In-Place API
```rust
let layout = ArciVisLayout::default();
layout.layout_in_place(&mut nodes, &mut edges)?;
// Modifies existing data structures directly
```

**When to use each:**
- **Ownership API**: When you need a complete snapshot or are working with immutable data
- **In-Place API**: When performance matters and you want to avoid copying large datasets
