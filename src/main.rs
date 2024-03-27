use thiserror::Error;
use std::path::{Path, PathBuf};

const DIR_NAME: &'static str = "deposito"; 
const DATABASE_URL: &'static str = "sqlite://deposito/deposito.db"; 

#[derive(Debug, Error)]
enum DepositoError {
    #[error("Deposito directory doesn't exist in the current directory `{0}`.")]
    NoDepositoDir(Box<Path>),

    #[error("Database error. {0}")]
    Sqlx(sqlx::Error),
}

struct Deposito {
    db_pool: Option<sqlx::SqlitePool>,
    default_warehouse: Option<String>,
}

enum RTItemOption {
    List,
    All
}

enum RemoveTypes {
    Warehouse,
    Item(RTItemOption)
}

impl Deposito {
    pub fn new(default_warehouse: String) -> Self {
        Self {
            db_pool: Default::default(),
            default_warehouse: Some(default_warehouse),
        }
    }

    pub async fn connect(&mut self, current_directory: &Path) -> Result<(), DepositoError> {
        if !Deposito::does_dir_exist(current_directory) {
            return Err(DepositoError::NoDepositoDir(current_directory.into()));
        }

        match sqlx::SqlitePool::connect(DATABASE_URL).await {
            Ok(pool) => { self.db_pool = Some(pool); Ok(()) }
            Err(err) => { return Err(DepositoError::Sqlx(err)); }
        }
    }

    pub fn get_pool(&mut self) -> Option<&sqlx::SqlitePool> {
        if self.db_pool.is_none() {
            println!("Unable to access the database. No connection pool.");
            return None;
        }
        return self.db_pool.as_ref()
    }

    pub fn help(command: Option<&str>) {
        if let Some(command) = command {
            match command {
                "edit" => {
                    println!("\x1b[1m`edit`\x1b[0m edit an existing item in a warehouse.");
                    println!();
                    println!("\x1b[1mUsage:\x1b[0m");
                    println!("\tdeposito edit [options] <warehouse> <item>");
                    println!();
                    println!("\x1b[1mExample:\x1b[0m");
                    println!("\tdeposito edit --name=Egg --q=6 --description=\"Just a regular egg\" Fridge Eg");
                    println!();
                    println!("\x1b[1mOptions:\x1b[0m");
                    println!("\t-n, --name=<name>         New name");
                    println!("\t-d, --description=<desc>  New description");
                    println!("\t-q, --quantity=<quan>     New quantity");
                }
                "add" => {
                    println!("\x1b[1m`add`\x1b[0m add a new item to a warehouse.");
                    println!();
                    println!("\x1b[1mUsage:\x1b[0m");
                    println!("\tdeposito add <warehouse> <item> [<quantity> | 1]");
                    println!();
                    println!("\x1b[1mExample:\x1b[0m");
                    println!("\tdeposito add Fridge Egg 8");
                }
                "list" => {
                    println!("\x1b[1m`list`\x1b[0m list items in the specified warehouse.");
                    println!();
                    println!("\x1b[1mUsage:\x1b[0m");
                    println!("\tdeposito list <warehouse>");
                    println!();
                    println!("\x1b[1mExample:\x1b[0m");
                    println!("\tdeposito list Fridge");
                }
                "view" => {
                    println!("\x1b[1m`view`\x1b[0m list available warehouses.");
                    println!();
                    println!("\x1b[1mUsage:\x1b[0m");
                    println!("\tdeposito view");
                    println!();
                    println!("\x1b[1mExample:\x1b[0m");
                    println!("\tdeposito view");
                }
                "create" => {
                    println!("\x1b[1m`create`\x1b[0m create a new warehouse.");
                    println!();
                    println!("\x1b[1mUsage:\x1b[0m");
                    println!("\tdeposito create <warehouse>");
                    println!();
                    println!("\x1b[1mExample:\x1b[0m");
                    println!("\tdeposito create Fridge");
                }
                "rules" => {
                    println!("\x1b[1m`rules`\x1b[0m manage rules in a warehouse.");
                    println!();
                    println!("\x1b[1mUsage:\x1b[0m");
                    println!("\tdeposito rules -l <warehouse>");
                    println!("\tdeposito rules -c <warehouse> <item> <quantity>");
                    println!("\tdeposito rules -e <warehouse> <items> <quantity>");
                    println!("\tdeposito rules -r <warehouse> <items>");
                    println!();
                    println!("\x1b[1mExample:\x1b[0m");
                    println!("\tdeposito rules -l Fridge");
                    println!("\tdeposito rules -c Fridge Egg 1");
                    println!("\tdeposito rules -e Fridge Egg Milk Dough 6");
                    println!("\tdeposito rules -r Fridge Milk Dough");
                    println!();
                    println!("\x1b[1mOptions:\x1b[0m");
                    println!("\t-l, --list    Show rules");
                    println!("\t-c, --create  Create a new rule");
                    println!("\t-e, --edit    Edit an existing rule");
                    println!("\t-r, --remove  Remove a rule");
                }
                "raport" => {
                    println!("\x1b[1m`raport`\x1b[0m create a raport.");
                    println!();
                    println!("\x1b[1mUsage:\x1b[0m");
                    println!("\tdeposito raport [options] <warehouse>");
                    println!();
                    println!("\x1b[1mExample:\x1b[0m");
                    println!("\tdeposito raport Fridge");
                    println!("\tdeposito raport -a");
                    println!();
                    println!("\x1b[1mOptions:\x1b[0m");
                    println!("\t-a, --all  Raport all warehouses");
                }
                "remove" => {
                    println!("\x1b[1m`remove`\x1b[0m remove a warehouse or an item.");
                    println!();
                    println!("\x1b[1mUsage:\x1b[0m");
                    println!("\tdeposito remove [options] <to_remove...>");
                    println!();
                    println!("\x1b[1mExample:\x1b[0m");
                    println!("\tdeposito remove -w Fridge");
                    println!("\tdeposito remove -i Fridge Egg");
                    println!("\tdeposito remove -ia Fridge");
                    println!();
                    println!("\x1b[1mOptions:\x1b[0m");
                    println!("\t-w, --warehouse  Remove a warehouse");
                    println!("\t-i, --item       Remove an item");
                    println!("\t-a, --all        Remove all (items only)");
                }
                _ => { println!("No help available for command `\x1b[1medit\x1b[1m`."); }
            }
        } else {
            println!("\x1b[1mdeposito - warehouse inventory management tool.\x1b[0m");
            println!();
            println!("\x1b[1mUsage:\x1b[0m");
            println!("\tdeposito [command] [options] [<args>...]");
            println!();
            println!("\x1b[1mCommands:\x1b[0m");
            println!("\tFor more information about a command run the command with `\x1b[1m-h\x1b[0m`");
            println!();
            println!("\tcreate - create a new warehouse");
            println!("\tview - list available warehouses");
            println!("\tlist - list items in the specified warehouse");
            println!("\tadd - add a new item to a warehouse");
            println!("\tedit - edit an existing item in a warehouse");
            println!("\tremove - remove a warehouse or an item");
            println!("\trules - manage rules in a warehouse");
            println!("\traport - create a raport");
        }
    }

    pub fn does_dir_exist(current_directory: &Path) -> bool {
        let directory_iter = std::fs::read_dir(current_directory).map_err(|err| {
            panic!("Unable to read directory. Reason: {}", err);
        }).unwrap();

        let mut deposito_exists = false;
        for entry_res in directory_iter {
            let entry = entry_res.map_err(|err| {
                panic!("Unable to read directory entry. Reason: {}", err);
            }).unwrap();

            let file_name = entry.file_name();
            let file_type = entry.file_type().map_err(|err| {
                panic!("Unable to read filetype of a directory entry. Reason: {}", err);
            }).unwrap();

            if file_type.is_dir() && file_name == DIR_NAME {
                deposito_exists = true;
                break;
            }
        }

        deposito_exists
    }

    pub async fn view(&mut self) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let warehouses_result = sqlx::query!(r#"SELECT * FROM warehouses;"#)
            .fetch_all(pool)
            .await;

        let warehouses = match warehouses_result {
            Ok(warehouses) => { warehouses }
            Err(err) => { println!("Unable to view warehouses. Reason: {}", err); return; }
        };

        if warehouses.is_empty() {
            println!("No warehouses. You can create a warehouse using \x1b[1m`deposito create <name>`\x1b[0m");
            return;
        }

        println!("\x1b[1mAvailable warehouses:\x1b[0m");
        for warehouse in warehouses {
            println!("\t- {}", warehouse.name);
        }
    }

    pub async fn create(&mut self, name: &str) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let create_result = sqlx::query!(r#"INSERT INTO warehouses (id, name) VALUES (NULL, ?)"#, name)
            .execute(pool)
            .await;

        match create_result {
            Ok(_) => { println!("Warehouse with the name `\x1b[1m{}\x1b[0m` successfully created!", name); }
            Err(err) => {
                // Warehouse already exists
                if let Some(err) = err.as_database_error() {
                if let Some(code) = err.code() {
                if code == "2067" {
                    println!("Warehouse with the name `\x1b[1m{}\x1b[0m` already exists.", name);
                    return;
                }}}

                println!("Unable to create a new warehouse with the name `\x1b[1m{}\x1b[0m`. Reason: {}", name, err);
            }
        }
    }

    pub async fn list(&mut self, warehouse_name: &str) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let items_result = sqlx::query!(r#"SELECT items.* FROM items JOIN warehouses ON items.warehouse_id = warehouses.id WHERE warehouses.name = ?;"#, warehouse_name)
            .fetch_all(pool)
            .await;

        let items = match items_result {
            Ok(items) => { items }
            Err(err) => {
                println!("Unable to list items in the `\x1b[1m{}\x1b[0m` warehouse. Reason: {}", warehouse_name, err);
                return;
            }
        };

        if items.is_empty() {
            println!("Warehouse `\x1b[1m{}\x1b[0m` contains no items.", warehouse_name);
            return;
        }

        println!("\x1b[1mAvailable items in the `{}` warehouse:\x1b[0m", warehouse_name);
        for item in items {
            print!("\t- {} ({})", item.name, item.quantity);
            if let Some(description) = item.description {
                print!(" {}", description);
            }
            print!("\n")
        }
    }

    pub async fn edit(&mut self, warehouse_name: &str, item_name: &str, edit: ItemEdit) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let edit_result = sqlx::query!("
            UPDATE items SET
                name = COALESCE(?, name),
                description = COALESCE(?, description),
                quantity = COALESCE(?, quantity)
            WHERE
                items.warehouse_id = (SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?)
                AND items.name = ?;",
            edit.name, edit.description, edit.quantity, warehouse_name, item_name)
            .execute(pool)
            .await;

        match edit_result {
            Ok(_) => { println!("Item with the name `\x1b[1m{}\x1b[0m` in the `\x1b[1m{}\x1b[0m` warehouse successfully edited.", item_name, warehouse_name); }
            Err(err) => { println!("Unable to edit item with the name `\x1b[1m{}\x1b[0m` in the `\x1b[1m{}\x1b[0m` warehouse. Reason: {}", item_name, warehouse_name, err); }
        }

        println!();
        self.raport_warehouse(warehouse_name).await;
    }

    pub async fn add(&mut self, warehouse_name: &str, item_name: &str, quantity: i64) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let add_result = sqlx::query!(
            r#"INSERT INTO items (id, warehouse_id, name, quantity) VALUES (NULL, (SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?), ?, ?);"#,
            warehouse_name, item_name, quantity)
            .execute(pool)
            .await;

        match add_result {
            Ok(_) => {
                println!("New item `\x1b[1m{}\x1b[0m` successfully added in quantity of `\x1b[1m{}\x1b[0m` into the `\x1b[1m{}\x1b[0m` warehouse.", item_name, quantity, warehouse_name);
            }
            Err(err) => {
                // Item already exists
                if let Some(err) = err.as_database_error() {
                if let Some(code) = err.code() {
                if code == "2067" {

                    // Add quantity to the item instead of adding a new item
                    let add_result = sqlx::query!("
                        UPDATE items SET
                            quantity = (quantity + ?)
                        WHERE
                            items.warehouse_id = (SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?)
                            AND items.name = ?;",
                        quantity, warehouse_name, item_name)
                        .execute(pool)
                        .await;

                    match add_result {
                        Ok(_) => { println!("Successfully added `\x1b[1m{}\x1b[0m` quantity to `\x1b[1m{}\x1b[0m` in the `\x1b[1m{}\x1b[0m` warehouse.", quantity, item_name, warehouse_name) }
                        Err(err) => { println!("Unable to add `\x1b[1m{}\x1b[0m` quantity to `\x1b[1m{}\x1b[0m` in the `\x1b[1m{}\x1b[0m` warehouse. Reason: {}", quantity, item_name, warehouse_name, err) }
                    }

                    return;
                }}}

                println!("Unable to add new item `\x1b[1m{}\x1b[0m` into the `\x1b[1m{}\x1b[0m` warehouse. Reason: {}", item_name, warehouse_name, err);
            }
        }
    }

    pub async fn remove(&mut self, remove_type: RemoveTypes, to_remove: Vec<String>) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let mut query_string: String;
        let mut query: sqlx::query::Query<'_, sqlx::Sqlite, _>;
        match remove_type {
            RemoveTypes::Warehouse => {
                query_string = String::from("DELETE FROM items WHERE items.warehouse_id IN (SELECT warehouses.id FROM warehouses WHERE warehouses.name IN (");
                for index in 0..to_remove.len() {
                    query_string.push_str(&format!("${}", index+1));
                    if index != to_remove.len()-1 { query_string.push(','); }
                }
                query_string.push_str("));");
                query_string.push_str("DELETE FROM warehouses WHERE warehouses.name IN (");
                for index in 0..to_remove.len() {
                    query_string.push_str(&format!("${}", index+1));
                    if index != to_remove.len()-1 { query_string.push(','); }
                }
                query_string.push_str(");");
            }
            RemoveTypes::Item(RTItemOption::List) => {
                query_string = String::from("DELETE FROM items WHERE items.warehouse_id IN (SELECT warehouses.id FROM warehouses WHERE warehouses.name = $1) AND items.name IN (");
                for index in 0..to_remove.len()-1 {
                    query_string.push_str(&format!("${}", index+2));
                    if index != to_remove.len()-2 { query_string.push(','); }
                }
                query_string.push_str(");");
            }
            RemoveTypes::Item(RTItemOption::All) => {
                query_string = String::from("DELETE FROM items WHERE items.warehouse_id IN (SELECT warehouses.id FROM warehouses WHERE warehouses.name = $1);");
            }
        }

        query = sqlx::query(&query_string);
        for remove in to_remove.iter() {
            query = query.bind(remove);
        }

        let remove_result = query.execute(pool).await;
        match remove_result {
            Ok(_) => { println!("Removed successfully."); }
            Err(err) => { println!("Unable to remove. Reason: {}", err); }
        }
    }

    pub async fn raport_warehouse(&mut self, warehouse_name: &str) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let raport_result = sqlx::query!(r#"
            SELECT
                i.name,
                i.quantity,
                i.description,
                r.gets_below_quantity,
                (SELECT COALESCE((SELECT 1 WHERE r.gets_below_quantity > i.quantity), FALSE, TRUE)) AS "is_critical: bool"
            FROM
                items i
                JOIN warehouses ON i.warehouse_id = warehouses.id
                JOIN rules r ON i.id = r.item_id
            WHERE
                warehouses.name = $1;
        "#, warehouse_name)
            .fetch_all(pool)
            .await;

        match raport_result {
            Ok(items) => {
                println!("Raport for the `\x1b[1m{}\x1b[0m` warehouse:", warehouse_name);
                for item in items {
                    print!("\t- {} ({})   ", item.name, item.quantity);

                    if item.is_critical.unwrap() {
                        print!("\x1b[31mCRITICAL\x1b[0m");
                    } else {
                        print!("\x1b[32mOK\x1b[0m");
                    }

                    print!(" (Can't go below {})\n", item.gets_below_quantity);
                }
            }
            Err(err) => { println!("Unable to make a raport for the `\x1b[1m{}\x1b[0m` warehouse. Reason: {}", warehouse_name, err); }
        }
    }

    pub async fn raport_all(&mut self) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let raport_result = sqlx::query!(r#"
            SELECT
                warehouses.name AS warehouse_name,
                i.name AS item_name,
                i.quantity,
                r.gets_below_quantity,
                (SELECT COALESCE((SELECT 1 WHERE r.gets_below_quantity > i.quantity), FALSE, TRUE)) AS "is_critical: bool"
            FROM
                items i
                JOIN warehouses ON i.warehouse_id = warehouses.id
                JOIN rules r ON i.id = r.item_id
            ORDER BY
                warehouses.name ASC;
        "#)
            .fetch_all(pool)
            .await;

        match raport_result {
            Ok(items) => {
                let mut current_warehouse = String::from("");
                for item in items {
                    if current_warehouse != item.warehouse_name {
                        current_warehouse = item.warehouse_name;
                        println!("\nRaport for the `\x1b[1m{}\x1b[0m` warehouse:", current_warehouse);
                    }
                    print!("\t- {} ({})   ", item.item_name, item.quantity);

                    if item.is_critical.unwrap() {
                        print!("\x1b[31mCRITICAL\x1b[0m");
                    } else {
                        print!("\x1b[32mOK\x1b[0m");
                    }

                    print!(" (Can't go below {})\n", item.gets_below_quantity);
                }
            }
            Err(err) => { println!("Unable to make a raport for all the warehouse. Reason: {}", err); }
        }
    }

    pub async fn show_rules(&mut self, warehouse_name: &str) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let rules_result = sqlx::query!("
            SELECT
                i.name,
                r.gets_below_quantity
            FROM
                rules r
                JOIN items i ON i.id = r.item_id
                JOIN warehouses w ON w.id = i.warehouse_id
            WHERE
                w.name = $1;
        ", warehouse_name)
            .fetch_all(pool)
            .await;

        match rules_result {
            Ok(rules) => {
                if rules.len() == 0 {
                    println!("No rules for the `\x1b[1m{}\x1b[0m` warehouse.", warehouse_name);
                    return;
                }

                println!("Rules for the `\x1b[1m{}\x1b[0m` warehouse:", warehouse_name);
                for rule in rules {
                    println!("\t- `\x1b[1m{}\x1b[0m` can't get below `\x1b[1m{}\x1b[0m`", rule.name, rule.gets_below_quantity);
                }
            }
            Err(err) => { println!("Unable get rules for the `\x1b[1m{}\x1b[0m` warehouse. Reason: {}", warehouse_name, err); }
        }
    }

    pub async fn create_rules(&mut self, warehouse_name: &str, item_name: &str, quantity: i64) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let create_rule_result = sqlx::query!("
            INSERT INTO rules (id, item_id, gets_below_quantity) VALUES (NULL, (SELECT items.id FROM items JOIN warehouses ON warehouses.id = items.warehouse_id WHERE items.name = $2 AND warehouses.name = $1), $3)
        ", warehouse_name, item_name, quantity)
            .execute(pool)
            .await;

        match create_rule_result {
            Ok(_) => {
                println!("Rule in the `\x1b[1m{}\x1b[0m` warehouse successfully created for `\x1b[1m{}\x1b[0m`. You will be alerted in the raport whenever the items' quantity gets below `\x1b[1m{}\x1b[0m`.\n", warehouse_name, item_name, quantity);
                self.raport_warehouse(warehouse_name).await;
            }
            Err(err) => {
                if let Some(err) = err.as_database_error() {
                if let Some(code) = err.code() {
                if code == "2067" {
                    println!("Rule in the `\x1b[1m{}\x1b[0m` warehouse for `\x1b[1m{}\x1b[0m` already exists.", warehouse_name, item_name);
                    return;
                }}}

                println!("Unable to create a rule in the `\x1b[1m{}\x1b[0m` warehouse for `\x1b[1m{}\x1b[0m`. Reason: {}", warehouse_name, item_name, err);
            }
        }
    }

    pub async fn edit_rules(&mut self, warehouse_name: &str, item_names: &[String], quantity: i64) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let mut query_string = String::from("UPDATE rules SET gets_below_quantity = $2 WHERE rules.item_id IN (SELECT items.id FROM items JOIN rules ON items.id = rules.item_id JOIN warehouses ON items.warehouse_id = warehouses.id WHERE warehouses.name = $1 AND items.name IN (");

        for index in 0..item_names.len() {
            query_string.push_str(&format!("${}", index+3));
            if index != item_names.len()-1 { query_string.push(','); }
        }
        query_string.push_str("));");

        let mut query: sqlx::query::Query<'_, sqlx::Sqlite, _> = sqlx::query(&query_string);
        query = query.bind(warehouse_name);
        query = query.bind(quantity);
        for item in item_names.iter() {
            query = query.bind(item);
        }

        match query.execute(pool).await {
            Ok(_) => { println!("Successfully edited specified rules in the `\x1b[1m{}\x1b[0m` warehouse.", warehouse_name); }
            Err(err) => { println!("Unable to edit rule(s) in the `\x1b[1m{}\x1b[0m` warehouse for `\x1b[1m{:?}\x1b[0m`. Reason: {}", warehouse_name, item_names, err); }
        }
    }

    pub async fn remove_rules(&mut self, warehouse_name: &str, item_names: &[String]) {
        let pool = match self.get_pool() {
            Some(pool) => { pool }
            None => { return; }
        };

        let mut query_string = String::from("DELETE FROM rules WHERE rules.item_id IN (SELECT items.id FROM items JOIN warehouses ON items.warehouse_id = warehouses.id WHERE warehouses.name = $1 AND items.name IN (");

        for index in 0..item_names.len() {
            query_string.push_str(&format!("${}", index+2));
            if index != item_names.len()-1 { query_string.push(','); }
        }
        query_string.push_str("));");

        let mut query: sqlx::query::Query<'_, sqlx::Sqlite, _> = sqlx::query(&query_string);
        query = query.bind(warehouse_name);
        for item in item_names.iter() {
            query = query.bind(item);
        }

        match query.execute(pool).await {
            Ok(_) => { println!("Successfully removed specified rules from the `\x1b[1m{}\x1b[0m` warehouse.", warehouse_name); }
            Err(err) => { println!("Unable to remove rule(s) in the `\x1b[1m{}\x1b[0m` warehouse for `\x1b[1m{:?}\x1b[0m`. Reason: {}", warehouse_name, item_names, err); }
        }
    }
}

impl Default for Deposito {
    fn default() -> Self {
        Self {
            db_pool: Default::default(),
            default_warehouse: Default::default()
        }
    }
}

struct ItemEdit {
    name: Option<String>,
    description: Option<String>,
    quantity: Option<i64>,
}

impl Default for ItemEdit {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            quantity: Default::default(),
        }
    }
}

#[derive(Debug)]
struct CliOption {
    name: String,
    value: Option<String>,
}

impl Default for CliOption {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default()
        }
    }
}

#[derive(Debug, Error)]
enum CliOptionError {
    #[error("An option must start with a single or a double hyphen ('-').")]
    MustStartWithHyphen,

    #[error("Provided string is too short to be an option.")]
    TooShort,
}

impl TryFrom<String> for CliOption {
    type Error = CliOptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut cli_option = CliOption::default();

        // Count the hyphens (max 2)
        let mut hyphens = 0;
        match value.get(0..2) {
            Some(chars) => {
                for char in chars.chars() {
                    if char == '-' { hyphens += 1; }
                }
            }
            None => { return Err(CliOptionError::TooShort); }
        }

        // Check if the option correctly begins with `-` or `--`
        if hyphens == 0 {
            return Err(CliOptionError::MustStartWithHyphen);
        }

        // Check if conatins a value for the option (separated by `=`)
        if let Some(eq_index) = value.find('=') {
            if let Some(name) = value.get(hyphens..eq_index) {
                cli_option.name = name.to_string();
            }

            if let Some(val) = value.get(eq_index+1..value.len()) {
                cli_option.value = Some(val.to_string());
            }
        } else {
            if let Some(name) = value.get(hyphens..value.len()) {
                cli_option.name = name.to_string();
            }
        }

        Ok(cli_option)
    }
}


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut cli_args = std::env::args();
    // Discard executable path
    cli_args.next();

    let command = match cli_args.next() {
        Some(command) => { command }
        None => { Deposito::help(None); return; }
    };


    let mut args: Vec<String> = Vec::new();
    let mut options: Vec<CliOption> = Vec::new();
    for arg in cli_args.into_iter() {
        if arg.starts_with('-') {
            match CliOption::try_from(arg.clone()) {
                Ok(option) => { options.push(option); }
                Err(_) => { println!("Unknown option `\x1b[1m{}\x1b[0m`.", arg); return; }
                
            }
        } else {
            args.push(arg);
        }
    }

    for option in options.iter() {
        if option.name == "h" || option.name == "help" {
            Deposito::help(Some(&command));
            return;
        }
    }

    // TODO: God, make it work more reasonably
    // Check if a command requires the `deposito` directory to exist
    let does_require_deposito = match command.as_str() {
        "create"
        | "list"
        | "raport"
        | "add"
        | "rules"
        | "edit"
        | "remove"
        | "view" => { true }

        "init" => { false }

        _ => { println!("Unknown command `{}`.", command); return; }
    };

    let mut current_dir = PathBuf::new();
    let mut deposito = Deposito::default();
    if does_require_deposito {
        current_dir = match std::env::current_dir() {
            Ok(dir) => { dir }
            Err(err) => { panic!("Unable to get current working directory. Reason: {}", err); }
        };

        match deposito.connect(&current_dir).await {
            Ok(_) => { }
            Err(err) => {
                if let DepositoError::NoDepositoDir(_) = err {
                    println!("Deposito directory doesn't exist. You can create one using \x1b[1m`deposito init`\x1b[0m");
                    return;
                }

                println!("Unable to connect with the local SQLite deposito database. Reason: {}", err);
            }
        };
    }

    match command.as_str() {
        "view" => { deposito.view().await }
        "create" => {
            if let None = args.get(0) {
                println!("`\x1b[1mcreate\x1b[0m` requires a name for the new warehouse.");
                return;
            }

            deposito.create(&args[0]).await
        }
        "list" => {
            if let None = args.get(0) {
                println!("`\x1b[1mlist\x1b[0m` requires a name of the warehouse to list the items of.");
                return;
            }

            deposito.list(&args[0]).await
        }
        "add" => {
            if let None = args.get(0) {
                println!("`\x1b[1madd\x1b[0m` requires a name of the warehouse to add the item to as the first argument.");
                return;
            }

            if let None = args.get(1) {
                println!("`\x1b[1madd\x1b[0m` requires a name of item to add to the warehouse as the second argument.");
                return;
            }

            let quantity: i64;
            match args.get(2) {
                Some(arg) => {
                    match arg.parse::<i64>() {
                        Ok(result) => { quantity = result }
                        Err(_) => { println!("`\x1b[1madd\x1b[0m` requires a valid quantity as the third argument."); return; }
                    }
                }
                None => { quantity = 1; }
            }

            deposito.add(&args[0], &args[1], quantity).await;
        }
        "edit" => {
            if let None = args.get(0) {
                println!("`\x1b[1medit\x1b[0m` requires a name of the warehouse to edit the item of as the first argument.");
                return;
            }

            if let None = args.get(1) {
                println!("`\x1b[1medit\x1b[0m` requires a name of item to edit in the warehouse as the second argument.");
                return;
            }

            let mut edit_options = ItemEdit::default();
            for option in options.into_iter() {
                match option.name.as_str() {
                    "n" | "name" => { edit_options.name = option.value; }
                    "d" | "description" => { edit_options.description = option.value; }
                    "q" | "quantity" => {
                        edit_options.quantity = match option.value {
                            Some(quantity) => {
                                match quantity.parse::<i64>() {
                                    Ok(quantity) => { Some(quantity) }
                                    Err(_) => { println!("Invalid quantity of `\x1b[1m{}\x1b[0m`.", quantity); return; }
                                }
                            }
                            None => { None }
                        }
                    }
                    _ => { println!("Unknown option `\x1b[1m{}\x1b[0m`.", option.name); return; }
                }
            }

            deposito.edit(&args[0], &args[1], edit_options).await;
        }
        "remove" => {
            let mut has_all_option = false;
            for option in options.iter() {
                if option.name == "a" || option.name == "all" {
                    has_all_option = true;
                    break;
                }
            }

            for option in options.into_iter() {
                match option.name.as_str() {
                    "w" | "warehouse" => {
                        if args.len() < 1 {
                            println!("`\x1b[1mremove\x1b[0m` with the --warehouse option requires at least one argument (which warehouse to remove).");
                            return;
                        }

                        deposito.remove(RemoveTypes::Warehouse, args.clone()).await;
                    }
                    "i" | "item" => {
                        if args.len() < 1 {
                            println!("`\x1b[1mremove\x1b[0m` with the --item option requires at least two arguments (from which warehouse, what item to remove).");
                            return;
                        }

                        if has_all_option {
                            // TODO: Error when item args
                            deposito.remove(RemoveTypes::Item(RTItemOption::All), args.clone()).await;
                        } else {
                            deposito.remove(RemoveTypes::Item(RTItemOption::List), args.clone()).await;
                        }
                    }
                    "a" | "all" => { /* skip */ }
                    _ => { println!("Unknown option `\x1b[1m{}\x1b[0m`.", option.name); return; }
                }
            }
        }
        "raport" => {
            let mut has_all_option = false;
            for option in options.iter() {
                if option.name == "a" || option.name == "all" {
                    has_all_option = true;
                    break;
                }
            }

            if has_all_option {
                deposito.raport_all().await;
            } else {
                deposito.raport_warehouse(&args[0]).await;
            }
        }
        "rules" => {
            for option in options.iter() {
                match option.name.as_str() {
                    "l" | "list" => { deposito.show_rules(&args[0]).await; }
                    "c" | "create" => {
                        if args.len() < 3 {
                            println!("`\x1b[1mrules\x1b[0m` with the --create option requires at least three arguments (warehouse, item, quantity).");
                        }

                        let quantity = match args[2].parse::<i64>() {
                            Ok(quantity) => { quantity }
                            Err(err) => { println!("Could not validate the quantity. Reason: {}", err); return; }
                        };

                        deposito.create_rules(&args[0], &args[1], quantity).await;
                    }
                    "e" | "edit" => {
                        if args.len() < 3 {
                            println!("`\x1b[1mrules\x1b[0m` with the --edit option requires at least three arguments (warehouse, item, quantity).");
                        }

                        let quantity = match args[args.len()-1].parse::<i64>() {
                            Ok(quantity) => { quantity }
                            Err(err) => { println!("Could not validate the quantity. Reason: {}", err); return; }
                        };

                        deposito.edit_rules(&args[0], args.get(1..args.len()-1).unwrap(), quantity).await;
                    }
                    "r" | "remove" => {
                        if args.len() < 2 {
                            println!("`\x1b[1mrules\x1b[0m` with the --remove option requires at least two arguments (warehouse, rule).");
                        }

                        deposito.remove_rules(&args[0], args.get(1..args.len()).unwrap()).await;
                    }
                    _ => { println!("Unknown option `\x1b[1m{}\x1b[0m`.", option.name); return; }
                }
            }
        }
        _ => { unreachable!() }
    }
}
