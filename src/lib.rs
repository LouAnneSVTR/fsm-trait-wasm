mod utils;
pub type NodeIndex = usize;
pub type TransitionIndex = usize;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
// src/lib.rs
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct FSMProxy {
    simple_fsm: Box<dyn FSM>,
}

#[wasm_bindgen]
impl FSMProxy {
    #[wasm_bindgen(constructor)]
    pub fn new(& mut self) -> FSMProxy {
        return FSMProxy {
            simple_fsm: Box::new(SimpleFiniteStateMachine {
                name_fsm: "".to_string(),
                transitions: vec![],
                nodes: vec![]
            }),
        }
    }
}

pub trait FSM {
    fn new(&mut self,nameFSM: &str) -> SimpleFiniteStateMachine;
    fn add_node(&mut self, name: i32) -> NodeIndex;
    fn remove_node(&mut self, name: i32) -> NodeIndex;
    fn add_transition(&mut self, letter: char, input_nodes: NodeIndex, output_nodes: NodeIndex) -> TransitionIndex;
    fn remove_transition(&mut self, letter: char, input_nodes: NodeIndex, output_nodes: NodeIndex) -> TransitionIndex;
    fn exist_transition(&mut self, transition_test: TransitionIndex, c_char: char) -> bool;
    fn display_transition(&self);
    fn display_fsm(&self);
    //fn process_fsm(&mut self, nodeIndex: NodeIndex, word: &str) -> Vec<(usize, usize)>;
    fn print_vec( &mut self, vec : Vec<Vec<&str>>);
}

pub struct SimpleFiniteStateMachine {
    name_fsm: String,
    transitions:  Vec<TransitionImpl>,
    nodes: Vec<NodeImpl>
}

impl FSM for SimpleFiniteStateMachine {
    fn new(&mut self,nameFSM: &str) -> SimpleFiniteStateMachine {
        todo!()
    }

    fn add_node(&mut self, name: i32) -> NodeIndex {
        todo!()
    }

    fn remove_node(&mut self, name: i32) -> NodeIndex {
        todo!()
    }

    fn add_transition(&mut self, letter: char, inputNodes: NodeIndex, outputNodes: NodeIndex) -> TransitionIndex {
        todo!()
    }

    fn remove_transition(&mut self, letter: char, input_nodes: NodeIndex, output_nodes: NodeIndex) -> TransitionIndex {
        todo!()
    }

    fn exist_transition(&mut self, transitionTest: TransitionIndex, c_char: char) -> bool {
        todo!()
    }

    fn display_transition(&self) {
        todo!()
    }

    fn display_fsm(&self) {
        todo!()
    }

    fn print_vec(&mut self, vec: Vec<Vec<&str>>) {
        todo!()
    }
}

pub trait NamedElement{
    fn get_name(self) -> String;
}

pub trait Node : NamedElement {

}
struct NodeImpl {
    name_node: String,
    pub name: i32,
    output_transition: Vec<TransitionIndex>,
    input_transition: Vec<TransitionIndex>
}

impl NamedElement for NodeImpl {
    fn get_name(self) -> String {
        self.name_node
    }


}

impl Node for NodeImpl {

}

pub trait Transition : NamedElement {

}
struct TransitionImpl {
    name_transition: String,
    pub letter: char,
    output_nodes: NodeIndex,
    input_nodes: NodeIndex,
}

impl NamedElement for TransitionImpl {
    fn get_name(self) -> String {
        self.name_transition
    }
}

impl Transition for TransitionImpl {

}



