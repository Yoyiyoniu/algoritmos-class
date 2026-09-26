use crate::list::LinkedList;

pub const PLACES: [&str; 6] = [
    "Walmart",
    "Soriana",
    "Costco",
    "Mercado Local",
    "Tiendita de la esquina",
    "Otro",
];

pub const CATEGORIES: [&str; 5] = [
    "Lácteos",
    "Carnes",
    "Abarrotes",
    "Frutas y verduras",
    "Limpieza",
];

const ERROR_EMPTY_FIELD: &str = "No deje ningún campo en blanco.";
const ERROR_BAD_QUANTITY: &str = "La cantidad debe ser un valor numérico mayor a cero.";
const ERROR_NO_POSITION: &str = "No hay artículos pendientes en esa posición.";
const ERROR_OVER_QUANTITY: &str = "La cantidad comprada supera lo planeado.";

pub struct Article {
    pub name: String,
    pub category: &'static str,
    pub place: &'static str,
    pub planned_quantity: i64,
    pub pending_quantity: i64,
}

pub struct Purchase {
    pub name: String,
    pub category: &'static str,
    pub planned_place: &'static str,
    pub real_place: &'static str,
    pub planned_quantity: i64,
    pub real_quantity: i64,
}

impl Purchase {
    pub fn is_same_place(&self) -> bool {
        self.planned_place == self.real_place
    }

    pub fn is_exact_quantity(&self) -> bool {
        self.real_quantity == self.planned_quantity
    }

    pub fn place_detail(&self) -> String {
        if self.is_same_place() {
            format!("Lugar planeado: {}", self.real_place)
        } else {
            format!(
                "Comprado en {} (planeado {})",
                self.real_place, self.planned_place
            )
        }
    }

    pub fn quantity_detail(&self) -> String {
        if self.is_exact_quantity() {
            format!("Cantidad exacta ({})", self.real_quantity)
        } else {
            format!(
                "Cantidad menor: {} de {}",
                self.real_quantity, self.planned_quantity
            )
        }
    }
}

pub struct ShoppingManager {
    pending: LinkedList<Article>,
    bought: LinkedList<Purchase>,
}

impl ShoppingManager {
    pub fn new() -> Self {
        ShoppingManager {
            pending: LinkedList::new(),
            bought: LinkedList::new(),
        }
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    pub fn bought_len(&self) -> usize {
        self.bought.len()
    }

    pub fn pending(&self) -> impl Iterator<Item = &Article> {
        self.pending.iter()
    }

    pub fn bought(&self) -> impl Iterator<Item = &Purchase> {
        self.bought.iter()
    }

    pub fn register(
        &mut self,
        name: &str,
        quantity_text: &str,
        place_index: usize,
        category_index: usize,
    ) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() || quantity_text.trim().is_empty() {
            return Err(ERROR_EMPTY_FIELD.to_string());
        }
        let quantity = parse_quantity(quantity_text)?;
        let place = place_at(place_index)?;
        let category = category_at(category_index)?;
        self.pending.push_back(Article {
            name: name.to_owned(),
            category,
            place,
            planned_quantity: quantity,
            pending_quantity: quantity,
        });
        Ok(())
    }

    pub fn buy(
        &mut self,
        index: usize,
        quantity_text: &str,
        real_place_index: Option<usize>,
    ) -> Result<(), String> {
        if quantity_text.trim().is_empty() {
            return Err(ERROR_EMPTY_FIELD.to_string());
        }
        let quantity = parse_quantity(quantity_text)?;
        let snapshot = self
            .pending
            .get(index)
            .map(|article| {
                (
                    article.name.clone(),
                    article.category,
                    article.place,
                    article.planned_quantity,
                    article.pending_quantity,
                )
            })
            .ok_or_else(|| ERROR_NO_POSITION.to_string())?;
        let (name, category, planned_place, planned_quantity, pending_quantity) = snapshot;
        if quantity > pending_quantity {
            return Err(ERROR_OVER_QUANTITY.to_string());
        }
        let real_place = match real_place_index {
            Some(place_index) => place_at(place_index)?,
            None => planned_place,
        };
        self.bought.push_back(Purchase {
            name,
            category,
            planned_place,
            real_place,
            planned_quantity,
            real_quantity: quantity,
        });
        if quantity == pending_quantity {
            self.pending.remove_at(index);
        } else {
            self.pending.update_at(index, |article| {
                article.pending_quantity -= quantity;
            });
        }
        Ok(())
    }

    pub fn classified<'a>(
        &'a self,
        category_index: usize,
        place_index: usize,
    ) -> Result<Vec<&'a Article>, String> {
        let category = category_at(category_index)?;
        let place = place_at(place_index)?;
        Ok(self
            .pending
            .iter()
            .filter(|article| article.category == category && article.place == place)
            .collect())
    }

    pub fn pending_label(&self, index: usize) -> Option<String> {
        self.pending.get(index).map(|article| {
            format!(
                "{} | {} ({}) quedan {}",
                article.name, article.place, article.category, article.pending_quantity
            )
        })
    }
}

impl Default for ShoppingManager {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_quantity(text: &str) -> Result<i64, String> {
    match text.trim().parse::<i64>() {
        Ok(quantity) if quantity > 0 => Ok(quantity),
        _ => Err(ERROR_BAD_QUANTITY.to_string()),
    }
}

fn place_at(index: usize) -> Result<&'static str, String> {
    PLACES
        .get(index)
        .copied()
        .ok_or_else(|| ERROR_EMPTY_FIELD.to_string())
}

fn category_at(index: usize) -> Result<&'static str, String> {
    CATEGORIES
        .get(index)
        .copied()
        .ok_or_else(|| ERROR_EMPTY_FIELD.to_string())
}
