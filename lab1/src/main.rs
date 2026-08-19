use std::io::{Write, stdin, stdout};
mod little_store;

use little_store::InventoryStore;

use crate::little_store::Item;

fn main() {
    let mut store = InventoryStore::new();

    loop {
        print!(
            "
            1. Mostrar productos
            2. Registrar productos
            3. Consultar producto
            3. Buscar producto
            4. Consulatar producto
            5. Eliminar producto
            6. salir
            "
        );

        let input = request("->");

        match input.trim().parse::<i64>() {
            Ok(num) => match num {
                1 => {
                    store.show();
                }
                2 => {
                    let mut code = request("Codigo:").parse::<i64>().unwrap();
                    let mut name = request("Nombre:");
                    let mut description = request("Descripcion:");
                    let mut price = request("Precio:").parse::<f64>().unwrap();
                    let mut avaible = request("Disponibles:").parse::<i64>().unwrap();

                    loop {
                        let item = Item {
                            code,
                            name: name.clone(),
                            description: description.clone(),
                            price,
                            available: avaible,
                        };

                        match store.register(item) {
                            Ok(()) => {
                                println!("Item Registrado !");
                                break;
                            }
                            Err(errors) => {
                                println!("Corrige los siguientes campos:");
                                for e in &errors {
                                    match e.as_str() {
                                        "description" => {
                                            println!("- Descripción vacía");
                                            description = request("Descripcion:");
                                        }
                                        "name" => {
                                            println!("Nombre vacío");
                                            name = request("Nombre:");
                                        }
                                        "negative_price" => {
                                            println!("Precio negativo");
                                            price = request("Precio").parse::<f64>().unwrap();
                                        }
                                        "negative_available" => {
                                            println!("Disponibles no puede ser negativo");
                                            avaible =
                                                request("Disponibles:").parse::<i64>().unwrap();
                                        }
                                        "code" => {
                                            println!("Código no puede ser negativo");
                                            code = request("Codigo:").parse::<i64>().unwrap();
                                        }
                                        "alredy_registered" => {
                                            println!("Ese código ya está registrado");
                                            code = request("Codigo:").parse::<i64>().unwrap();
                                        }
                                        _ => println!("Error desconocido: {}", e),
                                    }
                                }
                            }
                        }
                    }
                }
                3 => {
                    let querry = request("Coloca DESCRIPCION de un producto a buscar:");

                    match store.search(querry) {
                        Ok(items) => {
                            items.iter().for_each(|item| {
                                println!(
                                    "C:{} | {} - {}\n ${:.2} ({})",
                                    item.code,
                                    item.name,
                                    item.description,
                                    item.price,
                                    item.available
                                );
                            });
                        }
                        Err(e) => print!("Error: {}", e),
                    }
                }
                4 => {
                    let querry = request("Coloca el NOMBRE de un producto a buscar:");

                    match store.consult(querry) {
                        Ok(items) => {
                            items.iter().for_each(|item| {
                                println!(
                                    "C:{} | {} - {}\n ${:.2} ({})",
                                    item.code,
                                    item.name,
                                    item.description,
                                    item.price,
                                    item.available
                                );
                            });
                        }
                        Err(e) => print!("Error: {}", e),
                    }
                }
                5 => {
                    let querry = request("Coloca el CODIGO del producto para eliminar")
                        .parse::<i64>()
                        .unwrap();

                    match store.remove(querry) {
                        Ok(item) => println!("Articulo {} eliminado correctamente", item.code),
                        Err(e) => print!("Fallo al eliminar el producto: {}", e),
                    }
                }
                6 | _ => {
                    print!("Saliendo =>>");
                    break;
                }
            },
            Err(_) => println!("Error: That was not a valid integer."),
        }
    }
}

fn request(prompt: &str) -> String {
    print!("{} ", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input.trim().to_string()
}
