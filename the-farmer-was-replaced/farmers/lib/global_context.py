from typing import TypedDict


class Context(TypedDict):
    social_plants: list[Entity]
    social_plant_index: int
