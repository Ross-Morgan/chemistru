# Chemistru Elements Inner

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Ross-Morgan/chemistru/elements-inner.yml?style=for-the-badge)
![docs.rs](https://img.shields.io/docsrs/chemistru-elements-inner?style=for-the-badge)

![Crates.io Version](https://img.shields.io/crates/v/chemistru-elements-inner?style=for-the-badge)
![Crates.io License](https://img.shields.io/crates/l/chemistru-elements-inner?style=for-the-badge)
![Crates.io Total Downloads](https://img.shields.io/crates/d/chemistru-elements-inner?style=for-the-badge)

![GitHub last commit](https://img.shields.io/github/last-commit/Ross-Morgan/chemistru?display_timestamp=author&style=for-the-badge)
![GitHub commit activity](https://img.shields.io/github/commit-activity/w/Ross-Morgan/chemistru?style=for-the-badge)

> [`chemistru-elements`](https://github.com/Ross-Morgan/chemistru/tree/main/chemistru-elements) is a wrapper of this crate, and should be used instead.

Provides a static vec of all the elements, with data loaded from a JSON file.

## Static Vector

The elements are stored in the [lazily-initialised vector](./src/lib.rs#L94)

### Getting Elements By Atomic (Proton) Number

```rust
# use chemistru_elements_inner as chemistru_elements;
// Atomic (proton) number, in this case, hydrogen
let z = 1;
// Static reference to the struct representing hydrogen
let element = chemistru_elements::utils::element_from_atomic_number(z);
```

### Getting Elements By Name

```rust
# use chemistru_elements_inner as chemistru_elements;
// Name of element
// Case insensitive and accepts multiple spellings
// i.e. 'Cesium', 'Caesium', 'CaEsIuM' will all work
let name_1 = "caesium";
let name_2 = "cesium";
let element_1 = chemistru_elements::utils::element_from_name(name_1);
let element_2 = chemistru_elements::utils::element_from_name(name_2);
assert_eq!(element_1, element_2)
```

### Preloading Elements

Since the vector of `Element`s lazily initialised, the first time it is accessed the cost of initialising it will also be inccured.

`preload_elements` ensures that the static vector of `Element`s is initialised. This is useful if initialising the element vector later would cause some tangible delay for the user.

#### Without

```rust
# const ELEMENTS: [(); 1] = [()];
// May cause a tangible delay (interacting with io)
let element = ELEMENTS[0];
```

#### With

```rust
# const ELEMENTS: [(); 1] = [()];
# use chemistru_elements_inner as chemistru_elements;
// Pre-initialise the vector of elements
chemistru_elements::utils::preload_elements();

// Virually no delay (trivial operation)
let element = ELEMENTS[0];
```
