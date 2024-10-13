from typing import override
from weakref import WeakKeyDictionary


# I was thinking I needed a weakref dictionary that allowed multiple objects as keys and only drop an item pair when both are GCed.
# I don't need it now, but I also think it might be better to just have an explicitly nested dictionary, i.e. WeakKeyDictionary[K, WeakKeyDictionary[K, V]]
# In any case, here is an untested mockup.
class WeakMultiKeyDictionary[K: tuple, V](WeakKeyDictionary[K, V]):
    @override
    def __setitem__(self, key: K, value: V, /, _recurse=False) -> None:
        if isinstance(key, tuple):
            assert len(key) > 1, "A single item tuples are not supported"
            *key_list, last = key
            for k in key_list:
                if k not in self:
                    map = WeakKeyDictionary()
                    super().__setitem__(k, map)  # type: ignore
                    self = map
                else:
                    self = self[k]  # type: ignore
            super().__setitem__(last, map)  # type: ignore
        else:
            super().__setitem__(key, value)

    @override
    def __getitem__(self, key: K) -> V:
        assert isinstance(key, tuple), "Only tuples as keys are supported"
        for k in key:
            self = self[k]  # type: ignore
        return self  # type: ignore


