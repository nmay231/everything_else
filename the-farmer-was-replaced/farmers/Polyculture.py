from farmers.lib.global_context import Context  # noqa: F401
from farmers.Utilities import clear_grid, for_each_globals, lazy_to_pos


def _polyculture_chain(ctx):  # type: (Context) -> None
    next_plant = ctx["social_plants"][ctx["social_plant_index"]]
    # TODO: Am I okay with hardcoded values like this instead of len(social_plants)?
    ctx["social_plant_index"] = (ctx["social_plant_index"] + 1) % 4

    while get_entity_type() == None or can_harvest():
        harvest()
        plant(next_plant)
        if get_water() < 0.3:
            use_item(Items.Water)

        next_plant, (x, y) = get_companion()
        lazy_to_pos(x, y)

    use_item(Items.Water)


def infinite_polyculture():  # type: () -> None
    clear_grid(Grounds.Soil)

    ctx = {
        "social_plant_index": 0,
        "social_plants": [
            Entities.Bush,
            Entities.Carrot,
            Entities.Tree,
            Entities.Grass,
        ],
    }  # type: Context
    while True:
        for_each_globals(ctx, _polyculture_chain)
