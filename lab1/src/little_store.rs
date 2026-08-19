pub struct Item {
    pub code: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub available: i64,
}

pub struct InventoryStore {
    pub items: Vec<Item>,
}

impl InventoryStore {
    pub fn new() -> Self {
        InventoryStore {
            items: vec![
                Item {
                    code: 12,
                    name: String::from("Refresco"),
                    description: String::from("Bebida gaseosa con sabor a fresa"),
                    price: 23.50,
                    available: 65,
                },
                Item {
                    code: 124,
                    name: String::from("Escoba"),
                    description: String::from("Escoba con palo de madera de 4 hilos"),
                    price: 64.95,
                    available: 30,
                },
                Item {
                    code: 56,
                    name: String::from("Papel sanitario"),
                    description: String::from("Paquete con 24 rollos y 500 hojas"),
                    price: 12.0,
                    available: 24,
                },
                Item {
                    code: 145,
                    name: String::from("Sal"),
                    description: String::from("Frasco de sal de cristales del Himalaya"),
                    price: 125.45,
                    available: 12,
                },
                Item {
                    code: 24,
                    name: String::from("Trapeador"),
                    description: String::from("Trapeador de algodón"),
                    price: 75.0,
                    available: 30,
                },
                Item {
                    code: 75,
                    name: String::from("Suero"),
                    description: String::from("Electrolit suero rehidratante sabor mora azul"),
                    price: 25.0,
                    available: 100,
                },
            ],
        }
    }

    pub fn get_all(&self) -> Result<&Vec<Item>, String> {
        (!self.items.is_empty())
            .then_some(&self.items)
            .ok_or("No Items :V".to_string())
    }

    pub fn register(&mut self, item: Item) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if item.description.is_empty() {
            errors.push("description".to_string());
        }
        if item.name.is_empty() {
            errors.push("name".to_string());
        }
        if item.price < 0.0 {
            errors.push("negative_price".to_string());
        }
        if item.available < 0 {
            errors.push("negative_available".to_string());
        }
        if item.code < 0 {
            errors.push("code".to_string());
        }

        if self.items.iter().any(|i| i.code == item.code) {
            errors.push("alredy_registered".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        self.items.push(item);
        Ok(())
    }

    pub fn search(&self, key: String) -> Result<Vec<&Item>, String> {
        let validated = self
            .items
            .iter()
            .filter(|item| item.description.contains(&key))
            .collect();

        if self.items.is_empty() {
            return Err("Not Item found".to_string());
        }
        Ok(validated)
    }

    pub fn consult(&self, key: String) -> Result<Vec<&Item>, String> {
        let validated = self
            .items
            .iter()
            .filter(|item| item.name.contains(&key))
            .collect();

        if self.items.is_empty() {
            return Err("Not Item found".to_string());
        }
        Ok(validated)
    }
    pub fn remove(&mut self, code: i64) -> Result<Item, String> {
        self.items
            .iter()
            .position(|item| item.code == code)
            .map(|idx| self.items.remove(idx))
            .ok_or("No Item found".to_string())
    }

    pub fn show(&self) {
        match self.get_all() {
            Ok(items) => {
                print!("\n--------------------------------------------------\n\n");
                for item in items {
                    println!(
                        "C:{} | {} - {}\n ${:.2} ({})",
                        item.code, item.name, item.description, item.price, item.available
                    );
                }
                print!("\n--------------------------------------------------\n\n");
            }
            Err(e) => print!("None :D error: {}", e),
        }
    }
}
