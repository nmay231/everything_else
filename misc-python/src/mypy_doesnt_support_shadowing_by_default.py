"""
This is one of my favorite (shallow) features from rust. You have to enable
`allow_redefinition` before this works.
"""

from typing import reveal_type

x: str = "42"
x: int = int(x)


def func() -> None:
    local: str = "42"
    reveal_type(local)
    local: int = int(local)
    reveal_type(local)
