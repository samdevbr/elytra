# Keys

## Document Key

Document keys are used to store documents into the sled kv store.

```
+---------------+---------------+
| Tag           | Partition Key |
| 0x1 (1 byte)  | 16 bytes      |
+---------------+---------------+
```

### Partition Key

A document partition key is used to embed the collection the document is part of,
and it's unique id (a snowflake).

This provides key locality for documents within the same collection, and a
natural sorting order by id.
> Since it's a snowflake it also sorts by creation time.

```
+-------------------+-----------+
| Collection Tag    | Snowflake |
| 64 bits           | 64 bits   |
+-------------------+-----------+
127               63 62         0
```
