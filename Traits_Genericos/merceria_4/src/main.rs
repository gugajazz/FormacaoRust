/*
Defina todas as estruturas necessárias para fazer track do inventário de uma mercearia, organizado por fileiras, prateleiras e zonas dentro de cada prateleira.

Deve guardar informação relativa ao produto:
    identificador
    nome
    data de validade
    preço
    quantidade

Ter a capacidade de adicionar e remover produtos, movê-los de local dentro da mercearia, mudar o preço e o nome.
Ter a capacidade de adicionar e remover quantidade de produtos (restock).

Estruturas de dados e iteradores
Augmente a Merceria para podermos encontrar eficientemente os produtos dentro de uma fileira, prateleira e zona.
Devemos também ser capazes de encontrar eficientemente um produto e a sua posição.

Nulls and errors
Augmente a merceria desenvolvida com a utilização de Options e Results.

Traits and Generics
Incremente a merceria ao definir uma trait que defina os comportamentos necessários de um artigo.
A nossa merceria deve ser capaz de ser utilizada para um tipo de item genérico.
Devemos manter todas as capacidades anteriores.
*/

use ordered_float::OrderedFloat;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub row_id: u32,
    pub rack_id: u32,
    pub zone_id: u32,
}

#[derive(Debug, Clone)]
pub struct GroceryShop<T: Item> {
    pub rows: HashMap<u32, Row<T>>,
    pub name_index: HashMap<String, Vec<Location>>,
}
impl<T: Item> GroceryShop<T> {
    pub fn new() -> Self {
        GroceryShop {
            rows: HashMap::new(),
            name_index: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        // GroceryShop with 3 rows, each with 2 racks and 2 zones
        // All the Racks and Rows have max capacity of 2
        for row in 0..3 {
            let mut racks = HashMap::new();
            for rack in 0..2 {
                let mut zones = HashMap::new();
                for zone in 0..2 {
                    zones.insert(zone, Zone { item: Option::None });
                }
                racks.insert(
                    rack,
                    Rack {
                        zones,
                        max_capacity: 2,
                    },
                );
            }
            self.rows.insert(
                row,
                Row {
                    racks,
                    max_capacity: 2,
                },
            );
        }
    }

    pub fn add_item(&mut self, item: T, location: &Location) -> Result<(), String> {
        // Add item to the specified location in the warehouse
        if let Some(zone) = self.get_zone_mut(location) {
            zone.item = Some(item.clone());
            self.add_item_to_name_index(&item, location);
            Ok(())
        } else {
            Err("Could not add item".to_string())
        }
    }

    pub fn remove_item(&mut self, location: &Location) -> Result<(), String> {
        // Remove item from the specified location in the warehouse

        if let Some(zone) = self.get_zone_mut(location) {
            // zone.item = None;
            // Remove item from the name index
            if let Some(item) = zone.item.take() {
                self.remove_item_from_name_index(&item, location);
            }
            Ok(())
        } else {
            Err("Could not remove item".to_string())
        }
    }

    pub fn get_item(&self, location: &Location) -> Option<&T> {
        // Get item from the specified location in the warehouse
        if let Some(zone) = self.get_zone(location) {
            return zone.item.as_ref();
        }
        None
    }

    pub fn get_item_mut(&mut self, location: &Location) -> Option<&mut T> {
        // Get mutable reference to item from the specified location in the warehouse
        if let Some(zone) = self.get_zone_mut(location) {
            return zone.item.as_mut();
        }

        None
    }

    pub fn get_zone(&self, location: &Location) -> Option<&Zone<T>> {
        // Get zone from the specified location in the warehouse
        if let Some(row) = self.rows.get(&location.row_id) {
            if let Some(rack) = row.racks.get(&location.rack_id) {
                return rack.zones.get(&location.zone_id);
            }
        }
        None
    }

    pub fn get_zone_mut(&mut self, location: &Location) -> Option<&mut Zone<T>> {
        // Get mutable reference to zone from the specified location in the warehouse
        if let Some(row) = self.rows.get_mut(&location.row_id) {
            if let Some(rack) = row.racks.get_mut(&location.rack_id) {
                return rack.zones.get_mut(&location.zone_id);
            }
        }
        None
    }

    pub fn get_location_of_item_linear_time(&self, item: &T) -> Option<Location> {
        // Get location of the specified item in the warehouse
        for (row_id, row) in &self.rows {
            for (rack_id, rack) in &row.racks {
                for (zone_id, zone) in &rack.zones {
                    if let Some(ref zone_item) = zone.item {
                        if zone_item == item {
                            return Some(Location {
                                row_id: *row_id,
                                rack_id: *rack_id,
                                zone_id: *zone_id,
                            });
                        }
                    }
                }
            }
        }
        None
    }

    pub fn move_item(
        &mut self,
        from_location: Location,
        to_location: &Location,
    ) -> Result<(), String> {
        // First check if the item exists at the location
        let item_exists = self.get_item(&from_location).is_some();

        if item_exists {
            // If item exists, get a clone of it before removing
            let item = self.get_item(&from_location).unwrap().clone();

            // Then remove it and add it to the new location
            if let Err(err) = self.remove_item(&from_location) {
                println!("Failed to remove item from the original location: {}", err);
                return Err("Failed to move item due to removal error".to_string());
            }
            if let Ok(_) = self.add_item(item.clone(), &to_location) {
                // Successfully moved the item
                Ok(())
            } else {
                // If adding to the new location fails, add it back to the original location
                self.add_item(item.clone(), &from_location)?;
                println!("Failed to move item to the new location");
                Err("Failed to move item to the new location".to_string())
            }
        } else {
            println!("Item not found at the specified location");
            Err("Item not found at the specified location".to_string())
        }
    }

    pub fn get_items_by_name(&self, name: &str) -> Vec<&T> {
        // Get items by name from the name index
        let mut items = Vec::new();
        if let Some(locations) = self.name_index.get(name) {
            for location in locations {
                if let Some(item) = self.get_item(location) {
                    items.push(item);
                }
            }
        }
        items
    }

    pub fn add_item_to_name_index(&mut self, item: &T, location: &Location) {
        // Add item to the name index
        let name = item.name().to_string();
        let locations = self.name_index.entry(name).or_insert_with(Vec::new);
        locations.push(location.clone());
    }

    pub fn remove_item_from_name_index(&mut self, item: &T, location: &Location) {
        // Remove item from the name index
        let name = item.name().to_string();
        if let Some(locations) = self.name_index.get_mut(&name) {
            locations.retain(|loc| loc != location);
            if locations.is_empty() {
                self.name_index.remove(&name);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Row<T: Item> {
    pub racks: HashMap<u32, Rack<T>>,
    pub max_capacity: u32,
}
impl<T: Item> Row<T> {
    pub fn new(max_capacity: u32) -> Self {
        Row {
            racks: HashMap::new(),
            max_capacity,
        }
    }

    pub fn add_rack(&mut self, rack_id: u32, max_capacity: u32) {
        self.racks.insert(rack_id, Rack::new(max_capacity));
    }
}

#[derive(Debug, Clone)]
pub struct Rack<T: Item> {
    pub zones: HashMap<u32, Zone<T>>,
    pub max_capacity: u32,
}
impl<T: Item> Rack<T> {
    pub fn new(max_capacity: u32) -> Self {
        Rack {
            zones: HashMap::new(),
            max_capacity,
        }
    }

    pub fn add_zone(&mut self, zone_id: u32) {
        self.zones.insert(zone_id, Zone { item: None });
    }
}

#[derive(Debug, Clone)]
pub struct Zone<T: Item> {
    pub item: Option<T>,
}

pub trait Item: PartialEq + Eq + std::fmt::Debug + Clone {
    fn name(&self) -> &str;
    fn quantity(&self) -> u32;
    fn uuid(&self) -> Uuid;
    fn price(&self) -> OrderedFloat<f32>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpirableItem {
    pub name: String,
    pub quantity: u32,
    pub uuid: Uuid,
    pub price: OrderedFloat<f32>,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}
impl Item for ExpirableItem {
    fn name(&self) -> &str {
        &self.name
    }

    fn quantity(&self) -> u32 {
        self.quantity
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn price(&self) -> OrderedFloat<f32> {
        self.price
    }
}

fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_add_items() {
        let mut shop = GroceryShop::new();
        shop.initialize();

        let item = ExpirableItem {
            name: "Milk".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(2.5),
            expiration_date: chrono::Utc::now(),
        };

        let location = Location {
            row_id: 1,
            rack_id: 1,
            zone_id: 1,
        };

        if let Ok(_) = shop.add_item(item.clone(), &location) {
            println!("Item added successfully");
            // assert!(true);
        } else {
            println!("Failed to add item");
            // assert!(false);
        }
        let received_item = shop.get_item(&location);
        assert_eq!(received_item, Some(&item));
        println!("Item in location: {:#?}", received_item);

        // assert_eq!(shop.get_item(location), Some(&item));
    }

    #[test]
    fn test_remove_items() {
        let mut shop = GroceryShop::new();
        shop.initialize();

        let item = ExpirableItem {
            name: "Milk".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(2.5),
            expiration_date: chrono::Utc::now(),
        };

        let location = Location {
            row_id: 1,
            rack_id: 1,
            zone_id: 1,
        };

        if let Ok(_) = shop.add_item(item.clone(), &location) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
        }

        if let Err(err) = shop.remove_item(&location) {
            println!("Failed to remove item: {}", err);
            assert!(false);
        }
        assert_eq!(shop.get_item(&location), None);
    }

    #[test]
    fn test_move_items() {
        let mut shop = GroceryShop::new();
        shop.initialize();

        let item = ExpirableItem {
            name: "Milk".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(2.5),
            expiration_date: chrono::Utc::now(),
        };

        let from_location = Location {
            row_id: 1,
            rack_id: 1,
            zone_id: 1,
        };
        let to_location = Location {
            row_id: 0,
            rack_id: 0,
            zone_id: 0,
        };

        if let Ok(_) = shop.add_item(item.clone(), &from_location) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
            assert!(false);
        }

        if let Ok(_) = shop.move_item(from_location, &to_location) {
            println!("Item moved successfully");
        } else {
            println!("Failed to move item");
            assert!(false);
        }

        assert_eq!(shop.get_item(&to_location), Some(&item));
    }

    #[test]
    fn test_edit_items() {
        // Change price
        // Change name
        let mut shop = GroceryShop::new();
        shop.initialize();
        let mut item = ExpirableItem {
            name: "Milk".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(2.5),
            expiration_date: chrono::Utc::now(),
        };
        let location = Location {
            row_id: 1,
            rack_id: 1,
            zone_id: 1,
        };
        if let Ok(_) = shop.add_item(item.clone(), &location) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
        }

        item.price = OrderedFloat(3.5);
        item.name = "Milk 2".to_string();
        if let Some(item_ref) = shop.get_item_mut(&location) {
            *item_ref = item.clone();
            println!("Item edited successfully");
        } else {
            println!("Failed to edit item");
        }
        assert_eq!(shop.get_item(&location), Some(&item));
        println!("Item in location: {:#?}", shop.get_item(&location));
    }

    #[test]
    fn test_decrease_stock_items() {
        // Decrease stock
        let mut shop = GroceryShop::new();
        shop.initialize();
        let mut item = ExpirableItem {
            name: "Milk".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(2.5),
            expiration_date: chrono::Utc::now(),
        };
        let location = Location {
            row_id: 1,
            rack_id: 1,
            zone_id: 1,
        };
        if let Ok(_) = shop.add_item(item.clone(), &location) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
        }

        item.quantity -= 1;
        if let Some(item_ref) = shop.get_item_mut(&location) {
            *item_ref = item.clone();
            println!("Item edited successfully");
        } else {
            println!("Failed to edit item");
        }
        assert_eq!(shop.get_item(&location), Some(&item));
    }

    #[test]
    fn test_increase_stock_items() {
        // Increase stock
        let mut shop = GroceryShop::new();
        shop.initialize();
        let mut item = ExpirableItem {
            name: "Milk".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(2.5),
            expiration_date: chrono::Utc::now(),
        };
        let location = Location {
            row_id: 1,
            rack_id: 1,
            zone_id: 1,
        };
        if let Ok(_) = shop.add_item(item.clone(), &location) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
        }

        item.quantity += 1;
        if let Some(item_ref) = shop.get_item_mut(&location) {
            *item_ref = item.clone();
            println!("Item edited successfully");
        } else {
            println!("Failed to edit item");
        }
        assert_eq!(shop.get_item(&location), Some(&item));
    }

    #[test]
    fn test_get_items_by_name() {
        let mut shop = GroceryShop::new();
        shop.initialize();

        let item1 = ExpirableItem {
            name: "Milk".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(2.5),
            expiration_date: chrono::Utc::now(),
        };
        let item2 = ExpirableItem {
            name: "Milk".to_string(),
            quantity: 10,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(3.0),
            expiration_date: chrono::Utc::now(),
        };

        let location1 = Location {
            row_id: 1,
            rack_id: 0,
            zone_id: 0,
        };
        let location2 = Location {
            row_id: 1,
            rack_id: 1,
            zone_id: 0,
        };

        if let Ok(_) = shop.add_item(item1.clone(), &location1) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
        }
        if let Ok(_) = shop.add_item(item2.clone(), &location2) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
        }

        let items = shop.get_items_by_name("Milk");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], &item1);
        assert_eq!(items[1], &item2);
    }
}
