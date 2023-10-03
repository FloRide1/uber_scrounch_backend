INSERT INTO locations (name) VALUES
  ('Room 201'),
  ('Room 202'),
  ('Room 203'),
  ('Room 503'),
  ('Room 504');

-- Warning this is in UTF
INSERT INTO deliveries (time) VALUES
  ('2023-10-02 10:00:00'),
  ('2023-10-03 14:30:00'),
  ('2023-10-04 09:15:00');

INSERT INTO users (email, admin) VALUES
  ('florian.reimat@epita.fr', true),
  ('valentin.chassignol@epita.fr', true),
  ('annabelle.chevreau@epita.fr', true),
  ('christian.diaconnu@epita.fr', true);

INSERT INTO products (sma_id, name, image_url, price, stock) VALUES
  ('25085337', 'Pomme Royal Gala', 'https://atelier-lyon.com/sales/assets/uploads/fda70b6d49cc44df0cb706c5727de224.jpg', 0.50, 53),
  ('3243610098112', 'Moelleux au caramel', 'https://atelier-lyon.com/sales/assets/uploads/af79f9c3b400e1fcb7a4cbbaf9e4b6e3.png', 0.50, 38),
  ('3259426038495', 'Madeleine chocolat noisette', 'https://atelier-lyon.com/sales/assets/uploads/a7a2e787f9e6d9dd8539aeb7a728cf0c.png', 0.50, 60),
  ('3800020456224', 'KitKak Chunky', 'https://atelier-lyon.com/sales/assets/uploads/819bae57f9ed32102a205e7b01813420.png', 0.80, 63),
  ('5000159407236', 'Mars', 'https://atelier-lyon.com/sales/assets/uploads/1a6eb0574c5a53c7859c1e4369897c55.png', 0.70, 57),
  ('5000159459228', 'Twix', 'https://atelier-lyon.com/sales/assets/uploads/0c4b00e85365da5f09813853ef671a8a.jpg', 0.70, 85),
  ('5000159461122', 'Snickers', 'https://atelier-lyon.com/sales/assets/uploads/c30cfed492e9e62115a8135dd0f46352.png', 0.80, 89),
  ('7613036257169', 'Crunch Snack', 'https://atelier-lyon.com/sales/assets/uploads/819bae57f9ed32102a205e7b01813420.png', 0.70, 23),
  ('7622210144300', 'Prince (x4)', 'https://atelier-lyon.com/sales/assets/uploads/31b2a737558fc87ab9b430c2b9e21c3f.png', 0.70, 37),
  ('8000500037560', 'Kinder Bueno', 'https://atelier-lyon.com/sales/assets/uploads/fc5b8cdf79177a1541b46f97bf95e961.png', 0.80, 111),
  ('80761761', 'Kinder Bueno White', 'https://atelier-lyon.com/sales/assets/uploads/851445d95910ecd346430f62b89767e5.png', 0.80, 101),
  ('84100733', 'Oreo (x6)', 'https://atelier-lyon.com/sales/assets/uploads/3b3c357250a59f13e7f4fa04abdddef6.jpg', 0.80, 40),
   -- Drink
  ('3057640317743', 'Volvic Citron (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/d159b4e97b65cdeaafee9c2aae0b5e03.png', 0.80, 20),
  ('3068320011820', 'Badoit (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/66f01914a2c9662a40594e8fea5d705b.jpg', 0.80, 27),
  ('3124480167026', 'Orangina (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/d8ddc8c6cde669938469419d415c2a12.png', 1.10, 21),
  ('3124480186676', 'Oasis (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/e4cd9aaee9067b750635d3eb5708d45c.png', 0.70, 111),
  ('3124480186898', 'Schweppes Agrumes (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/67758156b86b109abb61442180a3e467.png', 0.70, 93),
  ('3168930159773', '7up (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/8daa8099cf8bace61ceac5f6503d3250.png', 0.70, 44),
  ('3168930159803', '7up Mojito (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/c4241e5a32ad5a013ea2d03aef806d17.jpg', 0.70, 42),
  ('3168930159896', 'Lipton IceTea (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/dfa32031f7d2b869851838ec42c105f5.jpg', 0.70, 65),
  ('3174780000363', 'Coca-Cola (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/4b22a1b0e149fb6aec64065a974e653c.png', 1.00, 66),
  ('3439497019535', 'Eau (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/e58e64db80ba4513195548f1005bfe5a.png', 0.50, 15),
  ('4000177158319', 'Capri-Sun Multi-Vitamin (20cl)', 'https://atelier-lyon.com/sales/assets/uploads/64e49633c2c3864daf150699ae7db8d4.png', 0.50, 75),
  ('5060335632302', 'Monster (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/8f1017647087adbf87bd0c9533746b06.png', 1.50, 26),
  ('5060517889852', 'Monster JUICED Mango Loco (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/657a84e59c2ef69d9ac50e4c3222d5dd.jpg', 1.50, 33),
  ('5449000131836', 'Coca-Cola Zero (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/59c47e99134110c8c9b3a91698f126e1.png', 1.00, 16),
  ('54492790', 'Coca-Cola Cherry (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/efa84a4585409e015251f58c004bd777.png', 1.10, 13);



INSERT INTO commands (user_id, location_id, delivery_id, confirmed, delivered, canceled) VALUES
  (1, 1, 1, true, false, false),
  (2, 2, 2, false, false, false),
  (3, 3, 3, true, true, false);

INSERT INTO command_products (command_id, product_id, amount) VALUES
  (1, 1, 10),
  (1, 2, 5),
  (2, 1, 20),
  (3, 3, 15);
