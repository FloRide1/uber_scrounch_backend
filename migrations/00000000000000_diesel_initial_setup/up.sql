CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE DOMAIN email AS varchar(254)
  CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email email UNIQUE NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT false,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    sma_id SERIAL UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR,
    -- This is an indication and should never be used as a currency in active exchange system
    price FLOAT NOT NULL DEFAULT 0,
    stock INTEGER NOT NULL CHECK(stock >= 0) DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE locations (
    id SERIAL PRIMARY KEY,

    name VARCHAR(255) UNIQUE NOT NULL,

    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE deliveries (
    id SERIAL PRIMARY KEY,

    time TIMESTAMP NOT NULL,

    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE commands (
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id) NOT NULL,
    location_id SERIAL REFERENCES locations(id) NOT NULL,
    delivery_id INTEGER REFERENCES deliveries(id),

    confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    delivered BOOLEAN NOT NULL DEFAULT FALSE,
    canceled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE command_products ( 
    id SERIAL PRIMARY KEY,
    command_id SERIAL REFERENCES commands(id) NOT NULL,
    product_id SERIAL REFERENCES products(id) NOT NULL,

    amount SERIAL NOT NULL CHECK(amount > 1),

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- CREATE OR REPLACE FUNCTION check_product_quantity()
-- RETURNS TRIGGER AS $$
-- BEGIN
--    DECLARE product_stock INTEGER;
--    BEGIN
--        SELECT stock INTO product_stock FROM products WHERE id = NEW.product_id;
--        IF (
--            (SELECT SUM(cp.amount)
--             FROM command_products cp
--             JOIN commands c ON cp.command_id = c.id
--             WHERE cp.product_id = NEW.product_id
--               AND NOT c.canceled
--               AND NOT c.delivered
--            ) > product_stock
--        ) THEN
--            RAISE EXCEPTION 'The total amount of products can t exceed the stock of the product';
--        END IF;
-- 
--        RETURN NEW;
--    END;
-- END;
-- $$ LANGUAGE plpgsql;


-- CREATE TRIGGER before_insert_update_command_products
-- BEFORE INSERT OR UPDATE ON command_products
-- FOR EACH ROW
-- EXECUTE FUNCTION check_product_quantity();

SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('products');
SELECT diesel_manage_updated_at('commands');
SELECT diesel_manage_updated_at('command_products');
SELECT diesel_manage_updated_at('locations');
SELECT diesel_manage_updated_at('deliveries');
