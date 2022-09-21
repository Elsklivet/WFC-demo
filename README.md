# Wave Function Collapse Demo

Demonstration of basic use of wave function collapse for explanatory purposes.

## Version 2.1(in pregress)

### what's new
- Added a skeleton for testing
- Implemented a buggy choose_next() based on entropy

### what needs to be improved
- Still passing the entire map around **!!!**
- More complex adjacency rules and conflict checking
- Implement repick() or avoid using recursion
- More and comprehensive tests
- Support command line argument to aid testing
- Clean up before Version 3.0

## Version 2.0

### what's new
- Improved readability
- Added more choices in the choice list
- Added another pre-set rule

### what needs to be improved
- Still passing the entire map around **!!!**
- More complex adjacency rules and conflict checking
- Choose based on entropy(seems trival)
- tests
- Support command line argument to aid testing


## Version 1.0

This branch is a simple huristic written in python, the rules are, numbers in tiles may not be more than $\pm 1$ away from its predecessor.

If interested, you can:
- make the algo run independently from relative position in map
- apply more complex adjcency rules
- explore iterative solutions
- explore generation of larger areas