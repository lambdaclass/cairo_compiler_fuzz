use clap::{Parser, ValueHint};
use std::io::{self, Write};
use clap::error::Error;
use std::thread;
use rand::Rng;
use std::collections::HashMap;

const TYPE_PREFIX: &str = "Type";
const CONST_PREFIX: &str = "const";
const VARIABLE_PREFIX: &str = "tempvar";
const FUNCTION_PREFIX: &str = "func";
const STRUCT_PREFIX: &str = "struct";

struct IdentGenerator {
    prefix_map: HashMap<String, i32>,
}

impl IdentGenerator {
    fn new() -> IdentGenerator {
        IdentGenerator {
            prefix_map: HashMap::new(),
        }
    }

    fn generate_variable(&mut self) -> String {
        self.generate(VARIABLE_PREFIX)
    }

    fn generate_const(&mut self) -> String {
        self.generate(CONST_PREFIX)
    }

    fn generate_type_alias(&mut self) -> String {
        self.generate(TYPE_PREFIX)
    }

    fn generate_function_name(&mut self) -> String {
        self.generate(FUNCTION_PREFIX)
    }

    fn generate_struct_name(&mut self) -> String {
        self.generate(STRUCT_PREFIX)
    }

    fn generate(&mut self, prefix: &str) -> String {
        let count = self.prefix_map.entry(prefix.to_string()).or_insert(0);
        *count += 1;
        format!("{}{}", prefix, count)
    }
}


#[derive(Parser, Debug)]
struct Args {
    #[clap()]
    count: usize,
    print: Option<bool>,
    output_stats: Option<bool>,
    threads: Option<u32>,
    chosen_selection_managers: Option<SelectionManagerOptions>,
    fail_fast: Option<bool>,
    seed: Option<u64>,
    directory: Option<String>,
}

#[derive(Debug, Clone)]
enum SelectionManagerOptions {
    BaseSelection,
    SwarmSelection,
    OptimalSelection,
    AggressiveSelection,
}

impl From<&str> for SelectionManagerOptions {
    fn from(value: &str) -> Self {
       match value {
        "base_selection" => Self::BaseSelection,
        "swarm_selection" => Self::SwarmSelection,
        "optimal_selection" => Self::OptimalSelection,
        "aggressive_selection" => Self::AggressiveSelection,
        _ => Self::BaseSelection,
       }
    }
}

struct ProgressBarBuilder {
    task: String,
    initial_max: i64,
    update_interval_millis: u64,
    continous_update: bool,
    unit_name: String,
    unit_size: i64,
    show_speed: bool,
    processed: i64,
    max_rendered_lenght: i64,
}

impl ProgressBarBuilder {
    fn set_task_name(&mut self, value: &str) -> &mut Self{
        self.task = value.to_string();
        self
    }

    fn set_initial_max(&mut self, initial_max: i64) -> &mut Self{
        self.initial_max = initial_max;
        self
    }

    fn new() -> Self {
        Self { 
            task: "".to_string(),
            initial_max: -1,
            update_interval_millis: 1000,
            continous_update: false,
            unit_name: "".to_string(),
            unit_size: 1,
            show_speed: false,
            processed: 0,
            max_rendered_lenght: -1,
        }
    }

}

// struct File {
//     file_system: FileSystem,
//     path: String,
//     path_status: PathStatus,
//     status: bool,
// }

fn generate_program(seed: u64, ident_generator: IdentGenerator, fail_fast: bool) -> (Program, CliArguments) {
    let funtion_symbol_table = FunctionSymbolTable::new();
    let global_symbol_table = GlobalSymbolTable::new();
    let symbol_table = SymbolTable::new(function_symbol_table,  global_symbol_table);
    let ast_generator = ASTGenerator::new(symbol_table, fail_fast, ident_generator);
    let main_function_context = Context("main", symbol_table);
    let number_of_constants = CustomRandom.nextInt(10);
    let mut constant_declarations = Vec::new();
    for x in 0..number_of_constants {
        constant_declarations.push(ast_generator.generate_constant_declaration(main_function_context))
    }
    let body = ast_generator(main_function_context);
    let body_with_output =
        StatementBlock::new(statements, symbol_table);
    let main_function = FunctionDefinition::new(
        "main",
        body_with_output ,
        false,
        false
    );
}


fn run(args: impl Iterator<Item = String>) -> Result<(), Error> {
    let args = Args::try_parse_from(args);
    let args = match args {
        Ok(args) => args,
        Err(error) => {
            return Err(error);
        }
    };
    let count = args.count;
    let print = args.print.unwrap_or(false);
    let output_stats = args.output_stats.unwrap_or(false);
    let threads = args.threads.unwrap_or(8);
    let chosen_selection_managers = args.chosen_selection_managers.unwrap_or(SelectionManagerOptions::BaseSelection);
    let fail_fast = args.fail_fast.unwrap_or(false);
    let mut seed = args.seed.unwrap_or(0);
    let directory = args.directory.unwrap_or("out_cairo".to_string());
    
    // if (!print) {
    //     File(directory).deleteRecursively();
    //     File(directory).mkdirs();
    // }

    let progress_bar = if !print {
        
        ProgressBarBuilder::new().set_task_name("Generating").set_initial_max(count as i64);
    };

    // let executor = Executors.newFixedThreadPool(count.coerceAtMost(threads));
    
    for x in 1..threads{
        thread::spawn(|| {
            let mut rng = rand::thread_rng();
            if seed == 0 { seed = rng.gen()}

            let mut ident_generator = IdentGenerator::new();
            let (generated_program, cli_arguments) = generate_program(seed, ident_generator, fail_fast);


        });
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    match run(std::env::args()) {
        Ok(()) => Ok(()),
        Err(error) => Err(error),
    }
}
