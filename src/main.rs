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
                    println!("\x1b[1mUsage:\x1b[0m deposito edit [options] <warehouse> <item>");
                    println!("\x1b[1mExample:\x1b[0m deposito edit --name=Egg --description=\"Just a regular egg\" Fridge Eg");
                    println!("\x1b[1mOptions:\x1b[0m");
                    println!("\t-n, --name=<name>         New name");
                    println!("\t-d, --description=<desc>  New description");
                    println!("\t-q, --quantity=<quan>     New quantity");
                }
                "add" => { todo!("Add help") }
                "list" => { todo!("List help") }
                "view" => { todo!("View help") }
                "create" => { todo!("Create help") }
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
            println!("\tview - list available warehouses");
            println!("\tlist - list items in the specified warehouse");
            println!("\tcreate - create a new warehouse");
            println!("\tadd - add a new item to a warehouse");
            println!("\tedit - edit an existing item in a warehouse");
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
        | "add"
        | "edit"
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
        _ => { unreachable!() }
    }
}
