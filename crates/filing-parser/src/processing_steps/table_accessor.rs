use crate::helpers::get_parent_and_index;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use std::rc::Rc;

pub trait TableAccessor {
    fn dom(&self) -> &RcDom;

    fn income_statement_table_nodes(&self) -> &Vec<Handle>;

    fn push_to_income_statement_table_nodes(&mut self, handle: Handle);

    fn filing_contents(&self) -> &String;

    fn filing_key(&self) -> &String;

    /// Gets the Node containing the entire parsed document
    fn get_doc(&self) -> Rc<Node> {
        Rc::clone(&self.dom().document)
    }

    fn delete_node(&mut self, handle: &Handle) {
        let (parent, index) = get_parent_and_index(&Rc::clone(handle))
            .expect("Couldn't find parent and index for dropping node");
        let removed = parent.children.borrow_mut().remove(index as usize);
    }
}
