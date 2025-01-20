from farmers.Carrots import _handle_tile_carrot
from farmers.lib.global_context import Context  # noqa: F401
from farmers.Polyculture import _polyculture_chain
from farmers.Trees import _plant_tree_or_bush
from farmers.Utilities import clear_grid, for_each, for_each_globals

quick_print("start", WHICH)
times = 5
if WHICH == "polyculture":
    ctx = {
        "social_plant_index": 0,
        "social_plants": [
            Entities.Bush,
            Entities.Carrot,
            Entities.Tree,
            Entities.Grass,
        ],
    }  # type: Context

    clear_grid(Grounds.Soil)
    while not (
        num_items(Items.Carrot) > 1000000
        and num_items(Items.Hay) > 1000000
        and num_items(Items.Wood) > 1000000
    ):
        for_each_globals(ctx, _polyculture_chain)
else:

    def tmp():  # type: () -> None
        """The default Hay collector relies on Grasslands naturally regrowing
        Grass instead of having to plant it manually"""
        harvest()
        plant(Entities.Grass)

    quick_print("Carrot start")
    clear_grid(Grounds.Soil, Entities.Carrot)
    while num_items(Items.Carrot) < 1000000:
        while num_items(Items.Wood) < 12 * 100:
            for_each(_plant_tree_or_bush)
        while num_items(Items.Hay) < 12 * 100:
            for_each(tmp)
        for_each(_handle_tile_carrot)

    quick_print("wood start")
    clear_grid(Grounds.Soil, Entities.Bush)
    while num_items(Items.Wood) < 1000000:
        for_each(_plant_tree_or_bush)

    quick_print("hay start")
    clear_grid(Grounds.Grassland)
    while num_items(Items.Hay) < 1000000:
        for_each(harvest)
