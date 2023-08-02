use crate::cairo_type::{Type, StructType};
use crate::symbol_table::{
    FunctionDefinition, 
    SymbolTable, 
    StatementBlock,
    ConstDeclaration
};
use crate::ident_generator::IdentGenerator;
use crate::context::Context;

pub(crate) trait CairoCode {
    fn to_cairo(&mut self) -> String{
        "".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct StructDefinition {
    struct_type: StructType,
    methods: Vec<FunctionDefinition>,
}

impl CairoCode for StructDefinition{
    fn to_cairo(&mut self) -> String {
    
        format!(
            "struct {} {{\n{}\n}}\n",
            self.struct_type.struct_name(),
            self.struct_type.types()
            .iter_mut()
            .map(|(name, typ)| format!("{}: {},", name, typ.to_cairo()))
            .collect::<Vec<_>>()
            .join("\n")
        )
    }
} 

#[derive(Clone, Debug)]
pub struct ASTGenerator {
    symbol_table: SymbolTable,
    fail_fast: bool,
    ident_generator: IdentGenerator,
}

impl ASTGenerator{
    pub fn new(symbol_table: SymbolTable, fail_fast: bool, ident_generator: IdentGenerator) -> Self {
        ASTGenerator { 
            symbol_table: symbol_table, 
            fail_fast: fail_fast, 
            ident_generator: ident_generator, 
        }
    }

    pub fn generate(&mut self, context: &mut Context) -> StatementBlock{
        self.symbol_table = context.symbol_table();
        self.clone()
    }

    pub fn generateConstantDeclaration(&mut self,ctx: Context) -> ConstDeclaration {
        let const_name = self.ident_generator.generate_const();

        let mut break_condition = true;

        while (break_condition) {
            let const_type = generate_type(ctx)
            if (type is LiteralType && type !is StringType) {
                symbolTable.root()[constName] = IdentifierData(type, false, OwnershipState.VALID, 0, true)
                return ConstDeclaration(type, constName, generateLiteral(type, ctx), symbolTable)
            } else {
                continue
            }
        }
    }

    pub fn generate_type(&mut self,ctx: Context) -> Type {
        generate_specific_type(select_random_type(ctx), ctx)
    }

    pub fn generate_specific_type() {

    }

    pub fn select_random_type(ctx: Context) -> Type {
        let pickRandomByWeight = selectionManager.availableTypesWeightings(ctx).pickRandomByWeight()
        Logger.logText("Picking type: $pickRandomByWeight", ctx, Color.GREEN)
        return pickRandomByWeight
    }
}

#[macro_export]
macro_rules! to_cairo_method {
    ($type:ty) => {
        const _: () = {
            fn to_cairo<T: CairoCode>() -> String{}
        };
    };
}
