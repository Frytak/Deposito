CREATE TABLE IF NOT EXISTS warehouses (
    id INTEGER UNIQUE NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS items (
    id INTEGER UNIQUE NOT NULL PRIMARY KEY,
    warehouse_id INTEGER,
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    FOREIGN KEY (warehouse_id) REFERENCES warehouses(id)
);