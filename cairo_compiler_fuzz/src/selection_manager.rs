pub struct SelectionManager {
    config: HashMap<Type, i32>
}

impl SelectionManager {
    pub fn available_types_weightings(&mut self, ctx: Context){
        NodeSelectionWeighting::new()
    }
}

pub struct NodeSelectionWeighting<T> {
    pub weightings: HashMap<T, u64>,
}

impl NodeSelectionWeighting {
    pub fn new() -> Self {
        NodeSelectionWeighting {
            weightings = HashMap::new()
        }
    }

    pub fn add_weighting(&mut self, type_id: Type, weighting: u64) {
        self.weightings.insert(type_id, weighting);
    }

    pub fn available_types_weightings(){
        let mut type_weightings = NodeSelectionWeighting::new();

    }

    pub fn pick_random_by_weight(&mut self) {
        let sum = self.weighting
        .iter()
        .map(|(t, n)| n )
        .sum();
    }
}
