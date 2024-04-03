CREATE TABLE account (
    id serial primary key,
    first_name character varying,
    last_name character varying,
    email character varying NOT NULL
);

COPY account(first_name, last_name, email)
FROM
    '/docker-entrypoint-initdb.d/contacts.csv' DELIMITER ',' CSV HEADER;