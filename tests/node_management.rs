use typestate_pattern::{states, Node, NodeContext};

#[test]
#[allow(unused_variables)]
fn new_node_in_valid_state() {
    let node = Node::new(NodeContext::default());
    let node = Node::<states::New>::new(NodeContext::default());

    // The following will NOT compile:
    // let node = Node::<states::Running>::new(Context::default());
    // let node = Node::<states::Leaving>::new(Context::default());
    // ...
}

#[test]
#[allow(unused_variables)]
fn fail_node() {
    let node = Node::new(NodeContext::default()).fail("some reason");
    let node = Node::<states::New>::new(NodeContext::default()).fail("some reason");

    // You can call `fail` on any node that is in an active state.
    let failed_from_syncing_node = Node::new(NodeContext::default())
        .start()
        .unwrap()
        .fail("some reason");

    // The following will NOT compile, as `Node<Failed>` doesn't satisfy
    // `NodeActiveState`:
    //
    // let node = Node::new(Context::default()).fail("reason").fail("reason");
}

#[test]
fn shared_context() {
    let ctx1 = NodeContext::new("foobar");
    let node = Node::new(ctx1.clone());

    // To `Syncing` state
    let node = node.start().unwrap();
    assert_eq!(node.ctx(), &ctx1);

    // To `Running` state
    let node = node.run().unwrap();
    assert_eq!(node.ctx(), &ctx1);

    // To `Leaving` state
    let node = node.stop().unwrap();
    assert_eq!(node.ctx(), &ctx1);

    // To `Removed` state
    let node = node.remove().unwrap();
    assert_eq!(node.ctx(), &ctx1);
}

#[test]
fn failures() {
    let node = Node::new(NodeContext::default());

    // Fail the node
    let node = node.fail("some reason");

    // The following will compile for `Node<Failed>` only (other node types do not
    // have this method/state data):
    assert_eq!(node.reason(), "some reason");
}
