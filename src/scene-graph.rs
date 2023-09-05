//
//  Not sure how to do this quite. There are several different types of
//  nodes, some having significantly different formats. Should each node
//  type have its own array? Or just one array with a node containing
//  information for everyone?
//

// This can't be Vec's but must be arrays for transferring to GPU.
// Should this be in one vec and split up for transfer to GPU?

// Start with one vec and only one node
// You can have a node with nothing pointing to it, then it is a root.
// There can be more than one root.

struct Node {
    op: Op,
    data: i32,  // index into an array or single integer of data
                // use dependent on Op
    left: NodeIdx,
    right: NodeIdx,
}

impl Node {
    fn new(op: Op, data: i32, left: NodeIdx, right: NodeIdx) -> Self {
        Self {
            op,
            data,
            left,
            right,
        }
    }
}

struct NodeIdx ( u32 );

const NullNode: NodeIdx = NodeIdx ( 0 );
const DefaultData: i32 = 0;

struct SceneGraph {
    root: NodeIdx,
    nodes: Vec<Node>,
    transforms: Vec<cgmath::Matrix4>,
}

// Needs a root node
pub fn a_scene1() -> SceneGraph {
    let mut a = SceneGraph::new();
    let sphere = a.sphere(500);
    a.root = a.union(
        a.transform(translate(-0.25, 0.0, 0.0), sphere),
        a.transform(translate( 0.25, 0.0, 0.0), sphere),
    );
    a
}


impl SceneGraph {
    fn new() -> Self {
        Self {
            nodes: vec![Node{NoOp, DefaultData, NullNode, NullNode}],
            trans: vec![{cgmath::Matrix4::One}],
        }
    }
    fn new_node(
        &self, op: Op, data: i32, left: NodeIdx, right: NodeIdx
    ) -> NodeIdx {
        self.nodes.push(Node::new(op, data, left, right));
        NodeIdx::new(nodes.len() - 1)
    }
    fn unary(&self, op: Op, data: i32, left: Node) -> NodeIdx {
        self.new_node(op, data, left, NullNode)
    }
    fn leaf(&self, op: Op, data: i32) -> NodeIdx {
        self.new_node(op, data, NullNode, NullNode)
    }
    fn new_trans(&self, transform: cgmath::Matrix4) -> i32 {
        self.trans.push(transform);
        trans.len() - 1
    }

    // Combinators
    pub fn union(&self, left: Node, right: Node) -> NodeIdx {
        self.new_node(Op::Union, DefaultData, left, right)
    }
    pub fn intersect(&self, left: Node, right: Node) -> NodeIdx {
        self.new_node(Op::Intersect, DefaultData, left, right)
    }
    pub fn subtract(&self, left: Node, right: Node) -> NodeIdx {
        self.new_node(Op::Subtract, DefaultData, left, right)
    }

    // Unary operators with data
    pub fn transform(&self, trans: cgmath::Matrix4, left: Node) -> NodeIdx { 
        self.unary(Op::Transform, self.new_trans(trans), left, NullNode)
    }
    pub fn texture(&self, tex: i32, left: Node) -> NodeIdx { 
        self.unary(Op::Texture, tex, left, NullNode)
    }

    // Primitives
    // Sphere radius in thousandths
    pub fn sphere(&self, radius: i32) -> NodeIdx {
        self.leaf(Op::Sphere, radius)
    }
}

enum Op {
    // Combinators
    Union,
    Intersect,
    Subtract,

    // Unary operators
    Transform,
    Texture,

    // Primitives
    Sphere,
}

// struct ShapeTree {
//     roots: Vec<NodeIdx>,
//     bin: Vec<BinaryNode>,
//     unary: Vec<UnaryNode>,
//     trans: Vec<Transform>,
//     leaf: Vec<Leaf>,
// }

// struct NodeIdx {
//     atype: NodeType,    // Binary node, Unary node, transform, leaf
//     aIdx: u32,          // Node index
// }

// struct BinaryNode {
//     op: Op,
//     left: NodeIdx,
//     right: NodeIdx,
// }

// struct UnaryNode {
//     op: UnaryOp,
//     child: NodeIdx,
// }

// struct Transform {
//     trans: cgmath::Matrix4<f32>,
//     child: NodeIdx,
// }

// struct Leaf {
//     op: BasicOp,
// }

