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

Eu imagino que este exercicio não é suposto ter Option e Results, mas não consegui fazer algo com o qual estivesse satisfeito sem os utilizar,
visto que rust depende tão fortemente destes conceitos.
*/

use ordered_float::OrderedFloat;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub row_id: usize,
    pub rack_id: usize,
    pub zone_id: usize,
}

#[derive(Debug, Clone)]
pub struct GroceryShop {
    pub rows: Vec<Row>,
}
impl GroceryShop {
    pub fn new() -> Self {
        GroceryShop { rows: Vec::new() }
    }

    pub fn initialize(&mut self) {
        // GroceryShop with 3 rows, each with 2 racks and 2 zones
        // All the Racks and Rows have max capacity of 2
        for row in 0..3 {
            let mut racks = Vec::new();
            for rack in 0..2 {
                let mut zones = Vec::new();
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

    pub fn add_item(&mut self, item: Item, location: &Location) -> Result<(), String> {
        // Add item to the specified location in the warehouse
        if let Some(zone) = self.get_zone_mut(location) {
            zone.item = Some(item.clone());

            Ok(())
        } else {
            Err("Could not add item".to_string())
        }
    }

    pub fn remove_item(&mut self, location: &Location) -> Result<(), String> {
        // Remove item from the specified location in the warehouse

        if let Some(zone) = self.get_zone_mut(location) {
            // zone.item = None;
            zone.item = None;
            Ok(())
        } else {
            Err("Could not remove item".to_string())
        }
    }

    pub fn get_item(&self, location: &Location) -> Option<&Item> {
        // Get item from the specified location in the warehouse
        if let Some(zone) = self.get_zone(location) {
            return zone.item.as_ref();
        }
        None
    }

    pub fn get_item_mut(&mut self, location: &Location) -> Option<&mut Item> {
        // Get mutable reference to item from the specified location in the warehouse
        if let Some(zone) = self.get_zone_mut(location) {
            return zone.item.as_mut();
        }

        None
    }

    pub fn get_zone(&self, location: &Location) -> Option<&Zone> {
        // Get zone from the specified location in the warehouse
        if let Some(row) = self.rows.get(location.row_id) {
            if let Some(rack) = row.racks.get(location.rack_id) {
                return rack.zones.get(location.zone_id);
            }
        }
        None
    }

    pub fn get_zone_mut(&mut self, location: &Location) -> Option<&mut Zone> {
        // Get mutable reference to zone from the specified location in the warehouse
        if let Some(row) = self.rows.get_mut(location.row_id) {
            if let Some(rack) = row.racks.get_mut(location.rack_id) {
                return rack.zones.get_mut(location.zone_id);
            }
        }
        None
    }

    pub fn get_location_of_item_linear_time(&self, item: &Item) -> Option<Location> {
        for (row_id, row) in self.rows.iter().enumerate() {
            for (rack_id, rack) in row.racks.iter().enumerate() {
                for (zone_id, zone) in rack.zones.iter().enumerate() {
                    if let Some(ref zone_item) = zone.item {
                        if zone_item == item {
                            return Some(Location {
                                row_id,
                                rack_id,
                                zone_id,
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
}

#[derive(Debug, Clone)]
pub struct Row {
    pub racks: Vec<Rack>,
    pub max_capacity: u32,
}
impl Row {
    pub fn new(max_capacity: u32) -> Self {
        Row {
            racks: Vec::new(),
            max_capacity,
        }
    }

    pub fn add_rack(&mut self, max_capacity: u32) -> Result<(), String> {
        // Check if the row has space for a new rack
        if self.racks.len() as u32 >= self.max_capacity {
            return Err("Row is full".to_string());
        }
        self.racks.push(Rack::new(max_capacity));
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Rack {
    pub zones: Vec<Zone>,
    pub max_capacity: u32,
}
impl Rack {
    pub fn new(max_capacity: u32) -> Self {
        Rack {
            zones: Vec::new(),
            max_capacity,
        }
    }

    pub fn add_zone(&mut self) -> Result<(), String> {
        // Check if the rack has space for a new zone
        if self.zones.len() as u32 >= self.max_capacity {
            return Err("Rack is full".to_string());
        }
        self.zones.push(Zone { item: None });
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Zone {
    pub item: Option<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
    pub uuid: Uuid,
    pub price: OrderedFloat<f32>,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}

fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_max_capacity() {
        let mut shop = GroceryShop::new();
        shop.initialize();

        // try to add more zones than the max capacity
        let row = 0;
        let rack = 0;
        let zone = 0;
        let location = Location {
            row_id: row,
            rack_id: rack,
            zone_id: zone,
        };

        // This should work because the rack is not full
        let item = Item {
            name: "Milk".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(2.5),
            expiration_date: chrono::Utc::now(),
        };
        if let Ok(_) = shop.add_item(item.clone(), &location) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
            assert!(false);
        }

        // This should work because the rack is not full
        let item2 = Item {
            name: "Bread".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(1.5),
            expiration_date: chrono::Utc::now(),
        };
        let location2 = Location {
            row_id: row,
            rack_id: rack,
            zone_id: zone + 1,
        };
        if let Ok(_) = shop.add_item(item2.clone(), &location2) {
            println!("Item added successfully");
        } else {
            println!("Failed to add item");
            assert!(false);
        }

        // This should fail because the rack is full
        let item3 = Item {
            name: "Eggs".to_string(),
            quantity: 5,
            uuid: Uuid::new_v4(),
            price: OrderedFloat(3.0),
            expiration_date: chrono::Utc::now(),
        };
        let location3 = Location {
            row_id: row,
            rack_id: rack,
            zone_id: zone + 2,
        };
        if let Err(err) = shop.add_item(item3.clone(), &location3) {
            println!("Failed to add item: {}", err);
            assert!(true);
        } else {
            println!("Item added successfully");
            assert!(false);
        }
    }

    #[test]
    fn test_get_items_that_dont_exist() {
        let mut shop = GroceryShop::new();
        shop.initialize();

        let location = Location {
            row_id: 1,
            rack_id: 1,
            zone_id: 3,
        };

        if let Some(item) = shop.get_item(&location) {
            println!("Item found: {:#?}", item);
            assert!(false);
        } else {
            println!("Item not found");
            assert!(true);
        }
    }

    #[test]
    fn test_add_items() {
        let mut shop = GroceryShop::new();
        shop.initialize();

        let item = Item {
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

        let item = Item {
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

        let item = Item {
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
        let mut item = Item {
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
        let mut item = Item {
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
        let mut item = Item {
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
}
