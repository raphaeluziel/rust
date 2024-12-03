use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works by {}", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();

    table.insert("Gesualdo".to_string(), 
                 vec!["Many Madrigals".to_string(),
                        "Tenabrae Responsario".to_string()]);
    table.insert("Caraviggio".to_string(), 
                 vec!["The Musicians".to_string(),
                         "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), 
                 vec!["Perseus with the head of Medusa".to_string(),
                         "A Salt Cellar".to_string()]);

    sort_works(&mut table);
    show(&table);

    //assert_eq!(table["Gesualdo"][0], "Many many madrigals");
}
