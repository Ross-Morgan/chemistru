# Chemistru Elements

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Ross-Morgan/chemistru-elements/rust.yml?style=for-the-badge)
![docs.rs](https://img.shields.io/docsrs/chemistru-elements?style=for-the-badge)
![Crates.io Version](https://img.shields.io/crates/v/chemistru-elements?style=for-the-badge)
![Crates.io License](https://img.shields.io/crates/l/chemistru-elements?style=for-the-badge)
![Crates.io Total Downloads](https://img.shields.io/crates/d/chemistru-elements?style=for-the-badge)
![GitHub last commit](https://img.shields.io/github/last-commit/Ross-Morgan/chemistru-elements?display_timestamp=author&style=for-the-badge)
![GitHub commit activity](https://img.shields.io/github/commit-activity/w/Ross-Morgan/chemistru-elements?style=for-the-badge)

Provides one of two functions:

1. Provides a static vec of all the elements, with data loaded from a JSON file.
2. Implements `ToTokens` from the `quote` library, allowing use with related chemistru crates.

## Static Vector

The elements are stored in the lazily-initialised vector `chemistru_elements::ELEMENTS`

### Getting Elements By Atomic (Proton) Number

```rust
// Atomic (proton) number, in this case, hydrogen
let z = 1;

// Static reference to the struct representing hydrogen
let element = chemistru_elements::element_from_atomic_number(z);
```

### Getting Elements By Name

```rust
// Name of element
// Case insensitive and accepts multiple spellings
// i.e. 'Cesium', 'Caesium', 'CaEsIuM' will all work
let name_1 = "caesium";
let name_2 = "cesium";

let element_1 = chemistru_elements::element_from_name(name_1)
let element_2 = chemistru_elements::element_from_name(name_2)

assert_eq!(element_1, element_2)
```

### Preloading Elements

Since the static vector of `Element`s is created using `lazy_static`, it will not be initialised until it is used (lazy initialisation)

This ensures that the static vector of `Element`s is initialised. This is useful if initialising the element vector later would cause some tangible delay for the user.

#### Without

```rust
operation_user_sees();

// May cause a tangible delay (interacting with io)
let element = ELEMENT[0];

operation_user_sees();
```

#### With

```rust
// Pre-initialise the vector of elements
chemistru_elements::preload_elements();

operation_user_sees();

// Virually no delay (trivial operation)
let element = ELEMENT[0];

operation_user_sees();
```
