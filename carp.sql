-- Enable extesion
CREATE EXTENSION postgres_fdw;

-- Link carp database
CREATE SERVER carp
    FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (dbname 'carp_mainnet');

-- Map user
CREATE USER MAPPING for postgres
    SERVER carp 
    OPTIONS (user 'postgres');


-- Conect tables
-- address
CREATE FOREIGN TABLE carp_address
(
  id bigint,
  payload bytea,
  first_tx bigint
)
 SERVER carp OPTIONS (schema_name 'public', table_name 'Address');

-- transaction
-- view
CREATE VIEW "export" AS 
SELECT 
   T.id,
   T.hash,
   array_agg("TI"."address_id") as inputs,
   array_agg("TO"."address_id") as outputs
FROM
 "Transaction" T
  LEFT JOIN "TransactionInput" "TI"
    ON "TI".tx_id = T.id
  LEFT JOIN "TransactionOutput" "TO"
    ON "TO".tx_id = T.id

GROUP BY T.id, T.hash

-- inserts
INSERT INTO address SELECT (id, 1, payload, NULL, ARRAY[], ARRAY[]) FROM carp_address;
INSERT INTO transaction SELECT
    id, 1, hash, NULL, "inputs", "outputs" FROM carp_export

-- 
create or replace function array_unique (a integer[]) returns integer[] as $$
  select array (
    select distinct v from unnest(a) as b(v)
  )
$$ language sql;

CREATE AGGREGATE array_join (anycompatiblearray)
(
    sfunc = array_cat,
    stype = anycompatiblearray
);