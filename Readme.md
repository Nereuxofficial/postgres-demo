# Postgres-demo
This is a small demo to demonstrate the capabilities of postgresql and how to use it in Rust.

This is intended for a target audience that has already used SQL and is familiar with the basic concepts of databases.

## Setup
Use the following command to setup the data and database for the demo:
```bash
tar -xvf data.tar.gz
docker-compose up -d
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
