use std::{iter::Map, collections::LinkedList};
use std::collections::{HashMap, HashSet};

use crate::astnode::{CairoCode, StructDefinition};
use crate::cairo_type::{Type, TupleType, ArrayType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    ExpressionStatement,
    Declaration,
    ConstDeclaration,
}

impl CairoCode for Statement{
    fn to_cairo(&mut self) -> String{
        self.to_cairo()
    }
}

struct ExpressionStatement {
    symbol_table: SymbolTable,
    expression: Expression,
    add_semicolon: bool,
}

impl CairoCode for ExpressionStatement{
    fn to_cairo(&mut self) -> String{
        if self.add_semicolon {
            let cairo_expression = self.expression.to_cairo();
            format!("{cairo_expression} ; ").to_string()
        } else {
            self.expression.to_cairo()
        }
    }
}

struct Declaration {
    mutable: bool,
    variable_name: String,
    variable_type: Type,
    value: Expression,
    symbol_table: SymbolTable,
}

impl CairoCode for Declaration{
    fn to_cairo(&mut self) -> String{
        let mutability = if self.mutable {
            "mut"
        } else {
            ""
        }; 
        
        format!("let {} {}: {} = {} ; ", mutability, self.variable_name, self.variable_type.to_cairo(), self.value.to_cairo()).to_string()
    }
}

pub struct ConstDeclaration {
    symbol_table: SymbolTable,
    variable_name: String,
    variable_type: Type,
    value: Expression,
}

impl CairoCode for ConstDeclaration{
    fn to_cairo(&mut self) -> String{
        format!("const {}: {} = {};", self.variable_name, self.variable_type.to_cairo(), self.value.to_cairo())
    }
}

enum Expression {
    LiteralExpression,
    LHSAssignmentNode,
    VoidLiteral,

}

impl CairoCode for Expression {
    fn to_cairo(&mut self) -> String{
        self.to_cairo()
    }
}

struct VoidLiteral {
    simbol_table: SymbolTable,
}

impl CairoCode for VoidLiteral {
    fn to_cairo(&mut self) -> String{
        format!("()")
    }
}

struct LHSAssignmentNode {
    variable: Option<Variable>,
}

impl CairoCode for LHSAssignmentNode {
    fn to_cairo(&mut self) -> String{
        match &mut self.variable {
            Some(variable) => variable.to_cairo(),
            None => "".to_string(),
        }
    }
}

pub struct Variable {
    value: String,
    simbol_table: SymbolTable,
}

impl CairoCode for Variable {
    fn to_cairo(&mut self) -> String{
       self.value.clone()
    }
}

enum LiteralExpression {
    // CLIArgumentAccessExpression,
    UInt8Literal,
    UInt16Literal,
    UInt32Literal,
    UInt64Literal,
    UInt128Literal,
    UInt256Literal,
    Felt252Literal,
    USizeLiteral,
    StringLiteral,
    BooleanLiteral,
    TupleLiteral,
}

// struct CLIArgumentAccessExpression {
//     index: u64,
//     exp_type: Type,
//     simbol_table: SymbolTable,
// }

// impl CairoCode for CLIArgumentAccessExpression{
//     fn to_cairo(&mut self) -> String{
//         format!("cli_args[{self.index}].clone().parse::<{self.type.to_cairo(&mut self)}>")
//     }
// }

struct UInt8Literal{
    value: u8,
    simbol_table: SymbolTable,
}

impl CairoCode for UInt8Literal {
    fn to_cairo(&mut self) -> String{
        format!("{}u8", self.value)
    }
}

struct UInt16Literal{
    value: u16,
    simbol_table: SymbolTable,
}

impl CairoCode for UInt16Literal {
    fn to_cairo(&mut self) -> String{
        format!("{}u16", self.value)
    }
}

struct UInt32Literal{
    value: u32,
    simbol_table: SymbolTable,
}

impl CairoCode for UInt32Literal {
    fn to_cairo(&mut self) -> String{
        format!("{}u32", self.value)
    }
}

struct UInt64Literal{
    value: u64,
    simbol_table: SymbolTable,
}

impl CairoCode for UInt64Literal {
    fn to_cairo(&mut self) -> String{
        format!("{}u64", self.value)
    }
}

struct UInt128Literal{
    value: u128,
    simbol_table: SymbolTable,
}

impl CairoCode for UInt128Literal {
    fn to_cairo(&mut self) -> String{
        format!("{}u128", self.value)
    }
}

struct UInt256Literal{
    first_value: u128,
    second_value: u128,
    simbol_table: SymbolTable,
}

impl CairoCode for UInt256Literal {
    fn to_cairo(&mut self) -> String{
        format!("{}{}u256", self.first_value, self.second_value)
    }
}

struct USizeLiteral{
    value: u32,
    simbol_table: SymbolTable,
}

impl CairoCode for USizeLiteral {
    fn to_cairo(&mut self) -> String{
        format!("{}usize", self.value)
    }
}

struct Felt252Literal{
    first_value: u128,
    second_value: u128,
    simbol_table: SymbolTable,
}

impl CairoCode for Felt252Literal {
    fn to_cairo(&mut self) -> String{
        format!("{}{}Felt252", self.first_value, self.second_value)
    }
}

struct StringLiteral{
    value: String,
    simbol_table: SymbolTable,
}

impl CairoCode for StringLiteral {
    fn to_cairo(&mut self) -> String{
        format!("'{}'", self.value)
    }
}

struct BooleanLiteral{
    value: bool,
    simbol_table: SymbolTable,
}

impl CairoCode for BooleanLiteral {
    fn to_cairo(&mut self) -> String{
        format!("{}", self.value.to_string())
    }
}

struct TupleLiteral{
    value: LinkedList<Expression>,
    simbol_table: SymbolTable,
}

impl CairoCode for TupleLiteral {
    fn to_cairo(&mut self) -> String{
        let mut tuple_string = self.value
        .iter_mut()
        .map(|e| e.to_cairo())
        .collect::<Vec<_>>()
        .join(" , ");

        format!("'{tuple_string}'")
    }
}

#[derive(Clone, Debug)]
pub struct StatementBlock {
    pub statements: LinkedList<Statement>,
    symbol_table: SymbolTable,
}

impl CairoCode for StatementBlock {
    fn to_cairo(&mut self) -> String{
        
        let mut final_statement = self.statements
            .iter_mut()
            .map(|stmt| {stmt.to_cairo()} )
            .collect::<Vec<_>>()
            .join("\n");
        final_statement
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    return_type: Type,
    function_name: String,
    arguments: Vec<(String, Type)>,
    body: StatementBlock,
}

impl CairoCode for FunctionDefinition {
    fn to_cairo(&mut self) -> String{

        // this has to convert the list of arguments to cairo arguments for the header of a function
        let cairo_arguments = self.arguments
            .iter_mut()
            .map(|(s, t)| format!("{}: {}",s, t.to_cairo()))
            .collect::<Vec<_>>()
            .join(" ,");

        let cairo_return = self.return_type.to_cairo();
        let cairo_body = self.body.to_cairo();
        let cairo_content = 
            format!("func {}({}) -> {} {{\n{}\n}}\n", 
            self.function_name, 
            cairo_arguments, 
            cairo_return,
            cairo_body);

        cairo_content
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentifierData {
    identifier_type: Type,
    mutable: bool,
    depth: usize,
    constant: bool,
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
    parent: Option<Box<SymbolTable>>,
    function_symbol_table: FunctionSymbolTable,
    global_symbol_table: GlobalSymbolTable,
    symbol_map: HashMap<String, IdentifierData>,
}

impl SymbolTable{
    pub fn new(function_symbol_table: FunctionSymbolTable,  global_symbol_table: GlobalSymbolTable) -> Self {
        SymbolTable {
            parent: None,
            function_symbol_table: function_symbol_table,
            global_symbol_table: global_symbol_table,
            symbol_map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionSymbolTable {
    symbol_map: HashMap<String, IdentifierData>,
    functions: Vec<FunctionDefinition>,
}

impl FunctionSymbolTable{
    pub fn new() -> Self {
        FunctionSymbolTable {
            symbol_map: HashMap::new(),
            functions: Vec::new(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct GlobalSymbolTable {
    symbol_map: HashMap<String, IdentifierData>,
    structs: HashSet<StructDefinition>,
    tuple_types: HashSet<TupleType>,
    array_types: HashSet<ArrayType>,
}

impl GlobalSymbolTable{
    pub fn new() -> Self {
        GlobalSymbolTable {
            symbol_map: HashMap::new(),
            structs: HashSet::new(),
            tuple_types: HashSet::new(),
            array_types: HashSet::new(),
        }
    }
}

