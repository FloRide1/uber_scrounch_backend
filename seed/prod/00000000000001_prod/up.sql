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

INSERT INTO products (sma_id, name, price, stock) VALUES
  ('25085337', 'Pomme Royal Gala', 0.50, 53),
  ('3243610098112', 'Moelleux au caramel', 0.50, 38),
  ('3259426038495', 'Madeleine chocolat noisette', 0.50, 60),


INSERT INTO products (sma_id, name, image_url, price, stock) VALUES
  ('25085337', 'Pomme Royal Gala', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.50, 53),
  ('3243610098112', 'Moelleux au caramel', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.50, 38),
  ('3259426038495', 'Madeleine chocolat noisette', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.50, 60),
  ('3800020456224', 'KitKak Chunky', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.80, 63),
  ('5000159407236', 'Mars', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 57),
  ('5000159459228', 'Twix', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 85),
  ('5000159461122', 'Snickers', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.80, 89),
  ('7613036257169', 'Crunch Snack', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 23),
  ('7622210144300', 'Prince (x4)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 37),
  ('8000500037560', 'Kinder Bueno', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.80, 111),
  ('80761761', 'Kinder Bueno White', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.80, 101),
  ('84100733', 'Oreo (x6)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.80, 40);
-- Drink
 ('3057640317743', 'Volvic Citron (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.80, 20),
  ('3068320011820', 'Badoit (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.80, 27),
  ('3124480167026', 'Orangina (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 1.10, 21),
  ('3124480186676', 'Oasis (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 111),
  ('3124480186898', 'Schweppes Agrumes (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 93),
  ('3168930159773', '7up (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 44),
  ('3168930159803', '7up Mojito (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 42),
  ('3168930159896', 'Lipton IceTea (33cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.70, 65),
  ('3174780000363', 'Coca-Cola (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 1.00, 66),
  ('3439497019535', 'Eau (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.50, 15),
  ('4000177158319', 'Capri-Sun Multi-Vitamin (20cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 0.50, 75),
  ('5060335632302', 'Monster (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 1.50, 26),
  ('5060517889852', 'Monster JUICED Mango Loco (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 1.50, 33),
  ('5449000131836', 'Coca-Cola Zero (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 1.00, 16),
  ('54492790', 'Coca-Cola Cherry (50cl)', 'https://atelier-lyon.com/sales/assets/uploads/no_image.png', 1.10, 13);
