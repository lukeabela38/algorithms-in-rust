# Algorithms Implemented in Rust

## Inversions

## Multiplication

## Sorting 

## Testing

Tests are functions to verify that non-test code is functioning in the expected manner. Test function bodies typically perform three actions:

1. Set up any needed data or state
2. Run the code to test
3. Assert the expected results

### Test Function Anatomy

A test in Rust is a function that is annotated with the test attribute. Attributes are metadata about pieces of Rust code - one example is the dervice attribute. To change a function into a test function, we need to add #[test] on the line above the function signature. 

You can then execute using cargo test.