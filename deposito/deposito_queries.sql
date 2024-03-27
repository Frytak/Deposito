SELECT * FROM warehouses;
SELECT * FROM items;
SELECT * FROM rules;

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

--- Remove specific warehouses (but first the items it contains)
DELETE FROM items WHERE items.warehouse_id IN (SELECT warehouses.id FROM warehouses WHERE warehouses.name IN (?));
DELETE FROM warehouses WHERE warehouses.name IN (?);

--- Remove specific items in a warehouse
DELETE FROM items WHERE items.warehouse_id IN (SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?) AND items.name IN (?);

--- Remove all items in a warehouse
DELETE FROM items WHERE items.warehouse_id IN (SELECT warehouses.id FROM warehouses WHERE warehouses.name = ?);

--- Select items from a specific warehouse that are below wanted threshold
SELECT
    i.name,
    i.quantity,
    i.description,
    r.gets_below_quantity,
    (SELECT COALESCE((SELECT 1 WHERE r.gets_below_quantity > i.quantity), FALSE, TRUE)) AS is_critical
FROM
    items i
    JOIN warehouses ON i.warehouse_id = warehouses.id
    JOIN rules r ON i.id = r.item_id
WHERE
    warehouses.name = "Fridge";

--- Select items from all warehouses that are below wanted threshold
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

--- Show rules
SELECT
    i.name,
    r.gets_below_quantity
FROM
    rules r
    JOIN items i ON i.id = r.item_id
    JOIN warehouses w ON w.id = i.warehouse_id
WHERE
    w.name = "Fridge";

--- Create a rule
INSERT INTO rules (id, item_id, gets_below_quantity) VALUES (NULL, (SELECT items.id FROM items JOIN warehouses ON warehouses.id = items.warehouse_id WHERE items.name = ? AND warehouses.name = ?), ?)

--- Edit rules
UPDATE rules SET gets_below_quantity = $2 WHERE rules.item_id IN (SELECT items.id FROM items JOIN rules ON items.id = rules.item_id JOIN warehouses ON items.warehouse_id = warehouses.id WHERE warehouses.name = $1 AND items.name IN (?));

--- Remove rules
DELETE FROM rules WHERE rules.item_id IN (SELECT items.id FROM items JOIN warehouses ON items.warehouse_id = warehouses.id WHERE warehouses.name = $1 AND items.name IN (?));
