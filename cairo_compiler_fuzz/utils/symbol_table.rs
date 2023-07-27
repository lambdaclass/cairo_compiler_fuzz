use std::iter::Map;
use crate::ASTnode::CairoCode;

enum Statement {
    ExpressionStatement,
    Declaration,
    ConstDeclaration,
}

struct ExpressionStatement {
    symbol_table: SymbolTable,
    expression: Expression,
    add_semicolon: bool,
}

impl CairoCode for ExpressionStatement{
    fn to_cairo() -> String{
        if self.add_semicolon {
            format!("{self.expression.to_cairo()} ; ").to_string()
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
    fn to_cairo() -> String{
        let mutability = if self.mutable {
            "mut"
        } else {
            ""
        }; 

        format!("let {mutability} {self.variable_name}: 
            {self.variable_type.to_cairo()} = 
            {self.value.to_cairo()} ; ")
            .to_string()
    }
}

struct ConstDeclaration {
    symbol_table: SymbolTable,
    variable_name: String,
    variable_type: Type,
    value: Expression,
}

impl CairoCode for ConstDeclaration{}

enum Expression {
    LiteralExpression,
    LHSAssignmentNode,
    VoidLiteral,

}

enum LiteralExpression {
    CLIArgumentAccessExpression,
    Int8Literal,
    Int16Literal,
    Int32Literal,
    Int64Literal,
    Int128Literal,
    UInt8Literal,
    UInt16Literal,
    UInt32Literal,
    UInt64Literal,
    UInt128Literal,
    USizeLiteral,
    Float32Literal,
    Float64Literal,
    StringLiteral,
    BooleanLiteral,
    TupleLiteral,

}

struct StatementBlock {
    statements: List<Statement>,
    symbol_table: SymbolTable,
}

impl CairoCode for StatementBlock {
    fn to_cairo() -> String{
        
        let mut final_statement = self.statements
            .iter()
            .map(|stmt| {stmt.to_cairo()} )
            .map(|stmt| {stmt.join("\n")})
            .collect();
        final_statement
    }
}

struct FunctionDefinition {
    return_type: Type,
    function_name: String,
    arguments: Map<String, Type>,
    body: StatementBlock,
}

impl CairoCode for FunctionDefinition {
    fn to_cairo() -> String{

        // this has to convert the list of arguments to cairo arguments for the header of a function
        let cairo_arguments = self.arguments.map();
        let cairo_return = self.return_type.to_cairo();
        let cairo_body = self.body.to_cairo();
        let cairo_content = format!("{inline}\nfunc {self.function_name}({
        cairo_arguments}) -> {cairo_return} {\n{cairo_body}\n}\n");
        cairo_content
    }
}

struct FunctionSymbolTable {
    symbol_map: Map<FunctionDefinition>,
}

