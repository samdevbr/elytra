# Elytra

A typesafe OGM (Object Graph Mapper) for Bolt + openCypher graph databases in Rust.

## Project Vision

Elytra aims to provide a robust, type-safe interface for interacting with graph databases in Rust applications. By combining a flexible query builder with a powerful macro-based API, we enable developers to work with graph data using familiar Rust patterns while ensuring type safety at compile time.

## Features

- **Type-safe Query Building**: Generate valid Cypher queries with compile-time checking
- **OGM Functionality**: Map between Rust structs and graph nodes/relationships
- **Ergonomic API**: Intuitive interfaces inspired by popular Rust ORMs
- **Performance Focused**: Minimal runtime overhead

## Development Roadmap

### Phase 1: Core Query Builder MVP

The initial focus is on building a fluent query builder that can generate valid Cypher queries.

#### Milestone 1.1: Basic Node Querying
- [ ] Implement `MATCH` clause for nodes
  ```rust
  Query::match_node("Person")
  ```
- [ ] Implement simple `WHERE` conditions
  ```rust
  .where_prop("name", "Ada")
  ```
- [ ] Implement `RETURN` clause
  ```rust
  .return_fields(&["name", "age"])
  ```
- [ ] Add query string generation
  ```rust
  .build() // Returns "MATCH (n:Person) WHERE n.name = 'Ada' RETURN n.name, n.age"
  ```

#### Milestone 1.2: Relationships and Advanced Filtering
- [ ] Implement relationship patterns
  ```rust
  Query::match_node("Person")
      .related_to("Movie", "LIKES", Direction::Outgoing)
  ```
- [ ] Add support for complex WHERE conditions
  ```rust
  .where_prop("age", ">", 30)
  .and_where("n.active = true")
  ```
- [ ] Implement parameter binding for safe query construction

#### Milestone 1.3: Write Operations
- [ ] Implement `CREATE` operations
  ```rust
  Query::create_node("Person", &[("name", "Ada"), ("age", 42)])
  ```
- [ ] Implement `MERGE` operations
  ```rust
  Query::merge_node("Person", &[("name", "Ada")])
  ```
- [ ] Implement `DELETE` operations
  ```rust
  Query::match_node("Person")
      .where_prop("name", "Ada")
      .delete("n")
  ```

#### Milestone 1.4: Results Pagination and Ordering
- [ ] Implement `ORDER BY` clause
  ```rust
  .order_by("n.age", Direction::Descending)
  ```
- [ ] Implement `LIMIT` and `SKIP` for pagination
  ```rust
  .limit(10)
  .skip(20)
  ```

### Phase 2: Macro-Based API MVP

The second phase focuses on building a type-safe, macro-based API that integrates with the query builder.

#### Milestone 2.1: Node and Relationship Definition Macros
- [ ] Implement `Node` derive macro
  ```rust
  #[derive(Node)]
  #[label = "Person"]
  struct Person {
      name: String,
      age: i32,
  }
  ```
- [ ] Implement `Relationship` derive macro
  ```rust
  #[derive(Relationship)]
  #[type_name = "LIKES"]
  struct Likes {
      #[from]
      person: Person,
      #[to]
      movie: Movie,
      rating: i32,
  }
  ```
- [ ] Generate schema modules for defined types

#### Milestone 2.2: Basic Query DSL
- [ ] Implement filtering operations
  ```rust
  person.filter(name.eq("Ada"))
  ```
- [ ] Implement comparison operators
  ```rust
  person.filter(age.gt(30))
  ```
- [ ] Implement results loading
  ```rust
  .load::<Person>(&db)
  ```

#### Milestone 2.3: Relationship Traversal
- [ ] Implement relationship traversal
  ```rust
  person
      .filter(name.eq("Ada"))
      .to(likes::person)
      .filter(rating.gt(3))
  ```
- [ ] Support for reverse traversal
  ```rust
  movie
      .filter(title.eq("Graph Theory 101"))
      .from(likes::movie)
  ```

#### Milestone 2.4: CRUD Operations
- [ ] Implement save/create operations
  ```rust
  let person = Person { name: "Ada".to_string(), age: 42 };
  person.save(&db);
  ```
- [ ] Implement update operations
  ```rust
  person.age = 43;
  person.update(&db);
  ```
- [ ] Implement delete operations
  ```rust
  person.delete(&db);
  ```

### Phase 3: Advanced Features

After establishing the MVP, focus shifts to more advanced capabilities.

#### Milestone 3.1: Complex Pattern Matching
- [ ] Support for variable length paths
  ```rust
  person
      .related_to_depth(friend::person, 1, 3)
      .filter(name.eq("Bob"))
  ```
- [ ] Support for complex path patterns

#### Milestone 3.2: Aggregations and Functions
- [ ] Implement COUNT, SUM, AVG and other aggregations
  ```rust
  person
      .group_by(age)
      .select((age, count()))
  ```
- [ ] Support for built-in Cypher functions

#### Milestone 3.3: Transactions and Batching
- [ ] Support for transactional operations
  ```rust
  db.transaction(|tx| {
      person.save(&tx);
      movie.save(&tx);
      Likes { person, movie, rating: 5 }.save(&tx);
      Ok(())
  })
  ```
- [ ] Implement batch operations for improved performance

#### Milestone 3.4: Advanced Cypher Features
- [ ] Support for UNWIND and list operations
- [ ] Support for calling stored procedures
- [ ] Support for subqueries and complex Cypher expressions

## Usage Examples

### Query Builder API

```rust
let query = Query::match_node("Person")
    .where_prop("name", "Ada")
    .return_fields(&["name", "age"])
    .build();

// Generates: MATCH (n:Person) WHERE n.name = 'Ada' RETURN n.name, n.age
```

### Macro API

```rust
use elytra::prelude::*;

#[derive(Node, Debug)]
#[label = "Person"]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let db = connect("bolt://localhost:7687");

    use schema::person::dsl::*;
    
    let results = person
        .filter(name.eq("Ada"))
        .filter(age.gt(30))
        .limit(5)
        .load::<Person>(&db)
        .unwrap();

    for p in results {
        println!("{} (age {})", p.name, p.age);
    }
}
```

## Contributing

We welcome contributions! Check the milestone tasks above for areas where help is needed.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
