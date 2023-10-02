DROP TABLE IF EXISTS command_products;
DROP TABLE IF EXISTS commands;
DROP TABLE IF EXISTS deliveries;
DROP TABLE IF EXISTS locations;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS products;

DROP DOMAIN IF EXISTS email;

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();
