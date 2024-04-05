CREATE TABLE account (
    id serial primary key,
    first_name character varying,
    last_name character varying,
    email character varying NOT NULL
);

CREATE TABLE images (id serial PRIMARY KEY, image bytea);

COPY account(first_name, last_name, email)
FROM
    '/docker-entrypoint-initdb.d/contacts.csv' DELIMITER ',' CSV HEADER;

INSERT INTO images (image) VALUES (pg_read_binary_file('/docker-entrypoint-initdb.d/test.png'));
