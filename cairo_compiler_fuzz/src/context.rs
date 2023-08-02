use crate::symbol_table::{SymbolTable, Statement, Variable};
use crate::cairo_type::Type;
use std::collections::LinkedList;
use std::collections::HashMap;

pub struct Context {
    node_depth_state: Vec<HashMap<String, i32>>,
    current_function_name: String,
    statements_per_scope: LinkedList<LinkedList<Statement>>,
    symbol_table: SymbolTable, 
    required_type: Option<Type>,
    return_expression_type: Option<Type>,
    return_loop_type: Option<Type>,
    previous_increment: Option<String>,
    assignment_root_node: Option<LinkedList<Variable>>,
    failed_generation_nodes: LinkedList<String>,
}

impl Context {
    pub fn new(
        node_depth_state: Vec<HashMap<String, i32>>,
        current_function_name: String,
        statements_per_scope: LinkedList<LinkedList<Statement>>,
        symbol_table: SymbolTable, 
        required_type: Option<Type>,
        return_expression_type: Option<Type>,
        return_loop_type: Option<Type>,
        previous_increment: Option<String>,
        assignment_root_node: Option<LinkedList<Variable>>,
        failed_generation_nodes: LinkedList<String>,
    ) -> Self {
        Context {
            node_depth_state,
            current_function_name,
            statements_per_scope,
            symbol_table,
            required_type,
            return_expression_type,
            return_loop_type,
            previous_increment,
            assignment_root_node,
            failed_generation_nodes,
        }
    }

    pub fn default(
        node_depth_state: Vec<HashMap<String, i32>>,
        current_function_name: String,
        statements_per_scope: LinkedList<LinkedList<Statement>>,
        symbol_table: SymbolTable, 
        
    ) -> Self {
        Context {
            node_depth_state,
            current_function_name,
            statements_per_scope,
            symbol_table,
            required_type: None,
            return_expression_type: None,
            return_loop_type: None,
            previous_increment: None,
            assignment_root_node: None,
            failed_generation_nodes: LinkedList::new(),
        }
    }

    pub fn symbol_table(&mut self) -> SymbolTable {
        self.symbol_table.clone()
    }

}







