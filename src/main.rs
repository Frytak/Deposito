// Pattern of `deposito`
// deposito [COMMAND] (OPTIONS) {args}

const DIR_NAME: &'static str = "deposito"; 
const DATABASE_NAME: &'static str = "deposito";
const DATABASE_URL: &'static str = "sqlite://deposito/deposito.db"; 
const MAX_MAGAZINES: u8 = 255;

struct Item {
    name: String,
    quantity: i64,
}

struct Warehouse {
    name: String,
    items: Vec<Item>,
}

struct Deposito {
    db_pool: Option<sqlx::SqlitePool>,
    default_warehouse: Option<String>,
    warehouse: Vec<Warehouse>,
}

impl Deposito {
    pub fn new(default_warehouse: String, warehouse: Vec<Warehouse>) -> Self {
        Self {
            db_pool: Default::default(),
            default_warehouse: Some(default_warehouse),
            warehouse,
        }
    }

    pub fn help() {
        println!("\x1b[1mdeposito - warehouse inventory management tool.\x1b[0m");
        println!();
        println!("\x1b[1mUsage:\x1b[0m");
        println!("\tdeposito [command] [options] [args...]");
        println!();
        println!("\x1b[1mCommands:\x1b[0m");
        println!("\tFor more information about a command run the command with `\x1b[1m-h\x1b[0m`");
        println!();
        println!("\tinit - initializes a new deposito folder");
        println!("\tview - lists available deposito warehouses in the current directory");
        println!("\tlist - lists items in the specified warehouse");
    }

    pub fn does_dir_exist<P>(current_directory: P) -> bool
    where P: AsRef<std::path::Path> {
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

    pub async fn view<P>(&mut self, current_directory: P)
    where P: AsRef<std::path::Path> {
        if !Deposito::does_dir_exist(current_directory) {
            panic!("Deposito directory doesn't exist. You can create one using \x1b[1m`deposito init`\x1b[0m")
        }

        if self.db_pool.is_none() {
            panic!("Unable to get current working directory.")
        }

        let conn = self.db_pool.as_ref().unwrap().acquire().await.map_err(|err| {
            panic!("Unable to connect with the local SQLite deposito database. Reason: {}", err)
        }).unwrap();

        let warehouses = sqlx::query!(r#"SELECT * FROM warehouses;"#)
            .fetch_all(self.db_pool.as_ref().unwrap())
            .await
            .map_err(|err| {
                panic!("Unable to fetch warehouses. Reason: {}", err);
            })
            .unwrap();

        if warehouses.is_empty() {
            println!("No warehouses. You can create a warehouse using \x1b[1m`deposito create <name>`\x1b[0m");
            return;
        }

        println!("\x1b[1mAvailable warehouses:\x1b[0m");
        for warehouse in warehouses {
            println!("\t- {}", warehouse.name);
        }
    }

    pub async fn connect(&mut self) -> Result<(), sqlx::Error> {
        match sqlx::SqlitePool::connect(DATABASE_URL).await {
            Ok(pool) => { self.db_pool = Some(pool); Ok(()) }
            Err(err) => { return Err(err); }
        }
    }
}

impl Default for Deposito {
    fn default() -> Self {
        Self {
            db_pool: Default::default(),
            default_warehouse: Default::default(),
            warehouse: Default::default(),
        }
    }
}



#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();
    // Discard executable path
    args.next();

    // Connect to the database
    let mut deposito = Deposito::default();
    deposito.connect().await.map_err(|err| {
        panic!("Unable to connect with the local SQLite deposito database. Reason: {}", err);
    }).unwrap();

    // Get current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => { dir }
        Err(err) => { panic!("Unable to get current working directory. Reason: {}", err); }
    };

    // Command
    match args.next() {
        Some(command) => {
            match command.as_str() {
                "view" => { deposito.view(current_dir).await }
                _ => { println!("Unknown command `{}`.", command); }
            }
        }
        None => { Deposito::help() }
    }
}
