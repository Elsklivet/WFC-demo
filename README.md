# Wave Function Collapse Demo

## Update Note Sep. 27

What if we make everything part of the WorldMap struct
- WorldMap keeps the rule set
    - Allows to contorl different rules for different world(areas)?
    - collapse() is a method of world
    - in the while loop
        - keep a copy of current world in stack
        - collapse this one
            - on success add this to stack
            - on failure pop the previous one
                - remove previous choice from the choices
                - (MUST) at some point give all of its choices back, maybe after we depleted the choices? AKA when we are going two steps back.

Also I feel more and more that we should keep all tiles seen in the the rule generation process so that we preserve the freqency. The original WFC also store waves in an array instead of a set. This is trivial to implement after we have a working version, which I believe is very close.

