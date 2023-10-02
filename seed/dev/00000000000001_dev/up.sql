INSERT INTO locations (name) VALUES
  ('Room 201'),
  ('Room 202'),
  ('Room 203'),
  ('Room 503'),
  ('Room 504');

INSERT INTO deliveries (time) VALUES
  ('2023-10-02 10:00:00'),
  ('2023-10-03 14:30:00'),
  ('2023-10-04 09:15:00');

INSERT INTO users (email, admin) VALUES
  ('user1@epita.fr', true),
  ('user2@epita.fr', false),
  ('user3@epita.fr', false);

INSERT INTO products (sma_id, name, description, price, stock) VALUES
  (1, 'Sel', 'Description 1', 0.99, 100),
  (2, 'Niveau a bulle fixe', 'Description 2', 2.50, 50),
  (3, 'Marteau a bomber le verre', 'Description 3', 50.0, 200);

INSERT INTO commands (user_id, location_id, delivery_id, confirmed, delivered, canceled) VALUES
  (1, 1, 1, true, false, false),
  (2, 2, 2, false, false, false),
  (3, 3, 3, true, true, false);

INSERT INTO command_products (command_id, product_id, amount) VALUES
  (1, 1, 10),
  (1, 2, 5),
  (2, 1, 20),
  (3, 3, 15);
