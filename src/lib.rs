use std::error::Error;

/// Result type for nodes.
type NodeResult<T> = Result<T, Box<dyn Error>>;

/// Shared context for all nodes.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NodeContext {
    pub name: String,
}

impl NodeContext {
    pub fn new(name: &str) -> Self {
        NodeContext {
            name: name.to_string(),
        }
    }
}

/// Node states.
pub mod states {
    use super::NodeState;

    pub struct New {}

    pub struct Syncing {}

    pub struct Running {}

    pub struct Leaving {}

    pub struct Failed {
        reason: String,
    }

    impl Failed {
        pub fn new(reason: String) -> Self {
            Self { reason }
        }

        pub fn reason(&self) -> &str {
            &self.reason
        }
    }

    pub struct Removed {}

    impl NodeState for New {}
    impl NodeState for Syncing {}
    impl NodeState for Running {}
    impl NodeState for Leaving {}
    impl NodeState for Failed {}
    impl NodeState for Removed {}
}

/// A node state.
///
/// Closed to extension by clients.
pub trait NodeState: private::Sealed {}

mod private {
    use super::states::*;

    pub trait Sealed {}

    impl Sealed for New {}
    impl Sealed for Syncing {}
    impl Sealed for Running {}
    impl Sealed for Leaving {}
    impl Sealed for Failed {}
    impl Sealed for Removed {}
}

pub struct Node<S: NodeState> {
    ctx: NodeContext,
    state: S,
}

impl Node<states::New> {
    /// Create a new node in a valid initial state.
    pub fn new(ctx: NodeContext) -> Self {
        Node {
            ctx,
            state: states::New {},
        }
    }

    /// Start the node.
    pub fn start(self) -> NodeResult<Node<states::Syncing>> {
        Ok(Node {
            ctx: self.ctx,
            state: states::Syncing {},
        })
    }
}

impl Node<states::Syncing> {
    /// Node is done syncing and ready to run.
    pub fn run(self) -> NodeResult<Node<states::Running>> {
        Ok(Node {
            ctx: self.ctx,
            state: states::Running {},
        })
    }
}

impl Node<states::Running> {
    /// Stop the node.
    pub fn stop(self) -> NodeResult<Node<states::Leaving>> {
        Ok(Node {
            ctx: self.ctx,
            state: states::Leaving {},
        })
    }
}

impl Node<states::Leaving> {
    /// Remove the node.
    pub fn remove(self) -> NodeResult<Node<states::Removed>> {
        Ok(Node {
            ctx: self.ctx,
            state: states::Removed {},
        })
    }
}

impl Node<states::Failed> {
    /// Remove the node.
    pub fn remove(self) -> NodeResult<Node<states::Removed>> {
        Ok(Node {
            ctx: self.ctx,
            state: states::Removed {},
        })
    }

    pub fn reason(&self) -> &str {
        &self.state.reason()
    }
}

impl Node<states::Removed> {
    // No state transitions are possible from this state.
}

/// Defined methods on all nodes.
impl<S: NodeState> Node<S> {
    /// Get the context.
    pub fn ctx(&self) -> &NodeContext {
        &self.ctx
    }
}

/// Selects the states that are active.
pub trait NodeStateActive: NodeState {}
impl NodeStateActive for states::New {}
impl NodeStateActive for states::Syncing {}
impl NodeStateActive for states::Running {}
impl NodeStateActive for states::Leaving {}

/// Define shared methods on a subset of nodes.
impl<S: NodeStateActive> Node<S> {
    pub fn fail(self, reason: &str) -> Node<states::Failed> {
        Node {
            ctx: self.ctx,
            state: states::Failed::new(reason.to_string()),
        }
    }
}
