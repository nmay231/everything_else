def measure(_direction: Direction | None = None) -> tuple[int, int]:
    """
    NOTE: This function is typed so we assume we always get tuple[int, int]
    Can measure some values on some entities. The effect of this depends on the entity.

    overloads:
    `measure()`: measures the entity under the drone.
    `measure(direction)`: measures the neighboring entity in the `direction` of the drone.

    Sunflower: returns the number of petals.
    Treasure: returns the next position.
    Cactus: returns the size.
    Dinosaur: returns the number corresponding to the type.
    All other entities: returns `None`.

    takes `1` tick to execute.

    example usage:
    ```
    num_petals = measure()
    ```
    """
    raise NotImplementedError
