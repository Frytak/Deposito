INSERT INTO warehouses (id, name) VALUES
(NULL, "Fridge"),
(NULL, "Death Star Lego Set"),
(NULL, "Garage");

INSERT INTO items (id, warehouse_id, name, description, quantity) VALUES
(NULL, 1, "Egg", NULL, 6),
(NULL, 1, "Ketchup", "Kotlin, straight from Piątnica.", 32),
(NULL, 1, "Milk", NULL, 1),
(NULL, 1, "Carrot", NULL, 1),
(NULL, 2, "Human Figures", NULL, 12),
(NULL, 2, "Gray Bricks", NULL, 344);

INSERT INTO rules (id, item_id, gets_below_quantity) VALUES
(NULL, 1, 4),
(NULL, 4, 8),
(NULL, 5, 8);
