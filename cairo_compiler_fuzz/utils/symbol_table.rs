use std::iter::Map;


struct FunctionDefinition {
    return_type: Type,
    function_name: String,
    arguments: Map<String, Type>,
    body: StatementBlock,
    force_no_inline: bool,
    add_self_variable: bool,
}

impl ToCairo for FunctionDefinition {
    fn to_rust() -> String{
        let inline = if self.force_no_inline (forceNoInline) {
            "#[inline(never)]"
        } else 
        { "" };

        let body = if (self.add_self_variable) {
            "&self,"
        } else 
        { "" };

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

