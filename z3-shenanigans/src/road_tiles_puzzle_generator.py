# First write a checker (maybe?) then a generator
# The generator will work by generating all possible domino pairs of compatible road tiles, then a grid of domino slots, and then run the quantum collapse algorithm (aka random assignment and backtracking).

# This was the 0% progress draft for writing a solver about my road-tiles puzzle
# variant. Here's an actual example of a puzzle in that genre:

"""
https://swaroopg92.github.io/penpa-edit/?m=edit&p=7Zdvb9s2EMbf51MQAgpsBRuIsv76XfpnQ4C02NoMRWEEhWIxtlZZ7CQ5aRzku/e5IyXLSooVGIrtxWCY/vnx8e54R9p0+9c2b7RUsUzlLJW+VHjEYSxnYYBnxk/fPc7LrtJz8dbkhTgvK90KJU+23do0c/E8bwCV2egbcXp6KuUzcVJVooFtKzbbthOXWixNXetlp4vjJ8HLZ+KD2YplXtemExtTlFe3YlVe65oniY4DXN6KvCjKeiVMIxq9MdfEbNDq1UbXXWt9vbrWze1+pg25zq+1eKq/5MuuuhWm1k9FWWBOucxtatb2p3JVm8Y67vKuNHX7syhrkdeIilWtTJ1XcJAXf+ZLTBdLXVXH4nytb0VhBOXPkTrzyCKpWq4CKyOuGrPh6RjqTjc0Z/T2WJwiZrfGBzemKdonwYvDqH0tYdMHIhc6X67dPNSp1iWjM6BMy1bkVWvERud167riSj9yg1li1ZSF0MVKH8tAXmGSPlq4DXBxdLfL5rsTuft1vvCUJ70AT+VdyN3v87vd6/nuTO7e4SNPKmhnoMiTAfCVtSd8z58TvbCi8sFvwLGd9gG4LJtlpT+eWeW3+WJ3Lj2K85xnE3rYC9pzedD7pdlcliRc5h32absuP7tP2m1hPm2drbq4l7uTb6c726dLaNMlmqbr1vOD080u7u9R9rdI+ON8Qbn/scd38zuMb+Z3XhT2y5OoI5xE0VSISUDDBiGZCAn7mI0E9oGaDAL7oD71AvsYWWRTH9nURzb1kU19KDVdjFLT1SjFbkaBlGI/YyXM9rNol3oqCvZrdgp7trOskvKscKRkPMtm6BSeZRcBBU1Q3IoPPP7CY8DjOToldzMeX/Lo8xjxeIbWqWgmYzXz5gGlE4OxNMcpVZw482Uc+XtOesbcyM0lTvac9T4z+KTu9zz4jGGDhQxs9SiQ0QwrBiYKqAaMBkwyG5/Rqsgq9VE6h8oaACOXd5zJhGprMaLmUGQVyNTFII4SFzpGaJsQocsfmFDte3QZJ7BF99lFAndugeCIdijlESElbKserQ9gRBuA3IVwZw2AEZ0nMnChZxapchbZQ49WxZYcLIg5DTJBFnTWHFJCFvuJyd6AkFZikddHGNlV94jcLHLy5MxlbJEXCsT+4RICQ6wuttuB0a4OGAUW4xmaYw1iTAtsBQnDAZPUlipF6r5VgVHsKh9Cjl3l4Tl18SzbKMSx7zhAH8Ixu60Ijlwk4th37CPb1NkT9z7BaeBiBTP4cexjYwW0mXAKX/FZfM/jCx5DHmM+hQl9m/79l+w/OuUhCkwVoWagh5Q8oas1MIZKy+7RGsSuRRa5RT1aNcVGowr1aJ0BuXG0FX3XI8epi6cCcG8TYOoBW48K+aXxmJ0NlsB9ccz9HdjpAfpC/R3Y5k7MfWR79MtVRflg6uPA6J3jyNmkaKlv5RQWexwM3L6wmNJWI0RWY3QJ+m6jMWNxQ2HcJp1958ZZ4KpM12b76Ll/jX7wZ2r8JpahL5+hqvQyC/FycbTw3m2bK1whcb04K2stXpdFUWkPtzmvNdXH1n0658serh/Q6u3mUjcHUmXM5wrTD0S+OOtHPyKRrpGP2F/iajvxfoML9oFg/5gcSPaWdSB1Da5Qo/d505ibA2WTd+sDYXTdOvCEG/ZhAl1+mGL+KZ9E2+zXfH/kffH4uQgkTpBS/1+W/6XLMjeCm+L92O/0/8LvyvfmwPvVNI8edsiPnHeo+3PdNdtD+cHJhv7gDFO8h8cY6iMnGer0MEN6eJ4hPjjS0L5xqsnr9GBTVtOzTaEeHG8KNT7hi4uv

**Road Tiles 1**

You draw lines between cell centers. The lines are called *roads*, and the pattern of roads on a cell is called a *road tile*. There are six road tiles ignoring rotations (see image for example).

Fill the grid with road tiles according to the following rules:
- All roads must be connected.
- You cannot modify given road tiles by adding or removing road segments.
- Every road tile must have *exactly one* identical road tile (ignoring rotations) in an orthogonally adjacent cell. They do not have to be connected.
- Roads must go from cell center to cell center. In other words, adjacent cells must both connect to each other or neither connect. This also means roads cannot connect to the grid edge.

**Note:** The last rule would be unnecessary if the controls were "Line > Normal", but I think "Line > Middle" helps you visualize the road tiles easier. Let me know what you think :smile:
**Penpa+:** https://git.io/JDrjv

"""
