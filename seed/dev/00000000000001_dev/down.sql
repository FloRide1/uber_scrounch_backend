DELETE FROM locations WHERE name IN ('Location 1', 'Location 2', 'Location 3');

DELETE FROM deliveries WHERE time IN ('2023-10-02 10:00:00', '2023-10-03 14:30:00', '2023-10-04 09:15:00');

DELETE FROM users WHERE email IN ('user1@example.com', 'user2@example.com', 'user3@example.com');

DELETE FROM products WHERE sma_id IN (1, 2, 3);

DELETE FROM commands WHERE user_id IN (1, 2, 3) AND location_id IN (1, 2, 3) AND delivery_id IN (1, 2, 3);

DELETE FROM command_products WHERE command_id IN (1, 2, 3);
