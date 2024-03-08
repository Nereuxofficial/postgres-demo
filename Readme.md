# Postgres-demo
This is a small demo to demonstrate the capabilities of postgresql and how to use it in Rust.

This is intended for a target audience that has already used SQL and is familiar with the basic concepts of databases.

## Setup
Use the following command to setup the data and database for the demo:
```bash
tar -xvf data.tar.gz
docker-compose up -d
```
This will start a postgresql database and a pgadmin instance to manage the database.

Postgres is commonly known as an relational database but more than that we can of course also use Postgres to store:
## KV-Data via HSTORE
```postgresql
SELECT config-> 'Age' AS age
FROM users
```
Updating them is pretty easy too:
```postgresql
UPDATE
users
SET
config = config || '"bg-color"=>"#00DD66", "language"=>"Italian"' :: hstore
WHERE
id = 4;
```
Or extracting all the keys:
```postgresql
SELECT akeys(config) FROM users WHERE id=1;
```
(And we could also use it for caching via the `pg_prewarm` extension or unlogged tables)

To top it off there is also the possibility to store:
## Json-data via JSONB
```postgresql
SELECT
    data-> 'name' AS product_name, data as raw_data
FROM products
```
And to get all the entries with the category "electronics", where the price is higher than 300$:
```postgresql
SELECT
    id,
    data ->> 'name' product_name
FROM
    products
WHERE
    data @> '{"category": "Electronics"}' AND data @? '$.price ? (@ > 300)';
```
Or remove data from the json easily:
```postgresql
SELECT
    '["PostgreSQL", "API", "Web Dev"]' :: jsonb - ARRAY['API','Web Dev'] result;
```


## Time-series Data using timescale
First we show the capabilities of timescaledb, a time-series database built as an extension of postgresql.
### Listing the energy consumption per hour
```postgresql
WITH per_hour AS (
    SELECT
        time,
        value
    FROM kwh_hour_by_hour
    WHERE "time" at time zone 'Europe/Berlin' > date_trunc('month', time) - interval '1 year'
    ORDER BY 1
), hourly AS (
    SELECT
        extract(HOUR FROM time) * interval '1 hour' as hour,
        value
    FROM per_hour
)
SELECT
    hour,
    approx_percentile(0.50, percentile_agg(value)) as median,
    max(value) as maximum
FROM hourly
GROUP BY 1
ORDER BY 1;
```
and now per day:
```postgresql
WITH per_day AS (
 SELECT
   time,
   value
 FROM kwh_day_by_day
 WHERE "time" at time zone 'Europe/Berlin' > date_trunc('month', time) - interval '1 year'
 ORDER BY 1
), daily AS (
    SELECT
       to_char(time, 'Dy') as day,
       value
    FROM per_day
), percentile AS (
    SELECT
        day,
        approx_percentile(0.50, percentile_agg(value)) as value
    FROM daily
    GROUP BY 1
    ORDER BY 1
)
SELECT
    d.day,
    d.ordinal,
    pd.value
FROM unnest(array['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']) WITH ORDINALITY AS d(day, ordinal)
LEFT JOIN percentile pd ON lower(pd.day) = lower(d.day);
```

And lastly
## Full Text Search
We can create a GIN(Generalized Inverted Index) over text in a table then search for it:
```postgresql
SELECT * FROM documents
WHERE to_tsvector('english', content) @@ to_tsquery('english', 'important & data');
```

## Other things
There is a lot i have not showed here of course, mostly because this took quite a lot of time and i wanted to keep it short.
But you can also use postgres for things like:
- Geospatial data
- Graph data
- Vector data via pgvector
and so much more, however i believe this is enough to show the astonishing capabilities of postgresql not just as a 
relational database but as a general purpose database. And if you do end up needing to use a specialized database, you can
always switch to one easily.