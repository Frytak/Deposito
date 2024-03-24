SELECT * FROM warehouses;
SELECT * FROM items;

--- Select items from a specific warehouse
SELECT items.* FROM items JOIN warehouses ON items.warehouse_id = warehouses.id WHERE warehouses.name = ?;

--- Select warehouses id by its' name
SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?;

--- Insert a new item into a warehouse
INSERT INTO items (id, warehouse_id, name, quantity) VALUES (NULL, (SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?), ?, ?);

--- Edit an item in the specified warehouse
UPDATE items SET
    name = COALESCE(?, name),
    description = COALESCE(?, description),
    quantity = COALESCE(?, quantity)
WHERE
    items.warehouse_id = (SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?)
    AND items.name = ?;

--- Add specified quantity to an item
UPDATE items SET
    quantity = (quantity + ?)
WHERE
    items.warehouse_id = (SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?)
    AND items.name = ?;
