# Algorithm

The layout algorithm operates in four distinct phases to produce clean, readable graph layouts:

### Phase 1: Initial Placement
- **Clustering**: Nodes are grouped into clusters based on connectivity patterns
- **Position Assignment**: Clusters are positioned to minimize edge crossings
- **Simple Force Application**: Basic repulsive forces prevent node overlap

### Phase 2: Force-Directed Refinement
- **Repulsion Forces**: Nodes repel each other to maintain minimum spacing
- **Attraction Forces**: Connected nodes are pulled together along edges
- **Iterative Optimization**: Multiple iterations refine the layout until convergence
- **Configurable Parameters**: Adjustable strength values for fine-tuning results

### Phase 3: Edge Routing
- **Grid-Based Pathfinding**: Uses a discrete grid overlay for path computation
- **A\* Algorithm**: Orthogonal pathfinding with obstacle avoidance
- **Port-Aware Routing**: Routes originate from appropriate node ports based on connection direction
- **Extension Points**: Edges extend orthogonally from ports before entering the routing grid
- **Obstacle Avoidance**: Node bodies and existing edges are treated as obstacles

### Phase 4: Finalization
- **Canvas Calculation**: Determines optimal canvas dimensions
- **Centering**: Translates the entire layout to center it within the canvas
- **Path Optimization**: Ensures all edge paths are orthogonal (horizontal/vertical only)
