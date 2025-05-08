import ctypes
from collections.abc import Sequence
from textwrap import indent
from typing import TYPE_CHECKING, Any, overload


class StructureMeta(type):
    _uninitialized_fields: Sequence[
        tuple[str, type["ctypes._CDataType"]]
        | tuple[str, type["ctypes._CDataType"], int]
    ]

    _fields_: Sequence[
        tuple[str, type["ctypes._CDataType"]]
        | tuple[str, type["ctypes._CDataType"], int]
    ]

    def __init__(cls, *args, **kwargs):
        super().__init__(*args, **kwargs)
        if not hasattr(cls, "_fields_") and hasattr(cls, "_uninitialized_fields"):
            cls._fields_ = cls._uninitialized_fields

    def __repr__(cls) -> str:
        fields = []
        for name, *_ in cls._fields_:
            fields.append(f"{name.split("#")[0]}={getattr(cls, name)!r}")
        return f"{cls.__name__}(\n{indent(",\n".join(fields), "    ")}\n)"


if TYPE_CHECKING:

    import _ctypes

    class MixedMeta(StructureMeta, _ctypes._PyCStructType): ...  # type: ignore

else:

    class MixedMeta(StructureMeta, type(ctypes.Structure)): ...


class Structure(ctypes.Structure, metaclass=MixedMeta):
    def __repr__(self) -> str:
        fields = []
        for name, *_ in self._fields_:
            # if name.startswith("_"): # TODO: Specific to my usecase
            #     continue

            value = getattr(self, name)
            name = name.split("#")[0]
            if isinstance(value, ctypes.Array):
                value = f"binary_array({repr(bytes(value))})"
            else:
                value = repr(value)
            fields.append(f"{name}={value}")
        return f"{self.__class__.__name__}(\n{indent(",\n".join(fields), "    ")}\n)"


# This would not have been possible for me to figure out without PyDantic as an example
class CtypesFieldInfo[T: ctypes._CDataType]:
    def __init__(self, type: type[T]):
        self.type = type

    def __set_name__(self, cls, name):
        self.name = name
        self.private_name = f"{name}#{self.__class__.__name__}"

        if not hasattr(cls, "_uninitialized_fields"):
            cls._uninitialized_fields = []

        cls._uninitialized_fields.append((self.private_name, self.type))

    def __get__(self, obj, cls):
        value = getattr(obj, self.private_name)
        if isinstance(value, ctypes.Array):
            value = bytes(value)
        setattr(obj, self.name, value)
        return value


# https://docs.python.org/3/library/ctypes.html#fundamental-data-types
@overload
def ctypes_field(
    type: type[
        ctypes.c_size_t
        | ctypes.c_ssize_t
        | ctypes.c_short
        | ctypes.c_long
        | ctypes.c_longlong
        | ctypes.c_byte
        | ctypes.c_int
        | ctypes.c_int8
        | ctypes.c_int16
        | ctypes.c_int32
        | ctypes.c_int64
        | ctypes.c_ushort
        | ctypes.c_ulong
        | ctypes.c_ulonglong
        | ctypes.c_ubyte
        | ctypes.c_uint
        | ctypes.c_uint8
        | ctypes.c_uint16
        | ctypes.c_uint32
        | ctypes.c_uint64
    ],
) -> int: ...
@overload
def ctypes_field(
    type: type[ctypes.c_float] | type[ctypes.c_double] | type[ctypes.c_longdouble],
) -> float: ...
@overload
def ctypes_field(
    type: type[ctypes.c_bool],
) -> bool: ...
@overload
def ctypes_field(
    type: type[ctypes.Array[ctypes.c_char] | ctypes.c_char],
) -> bytes: ...
@overload
def ctypes_field(
    type: type[ctypes.Array[ctypes.c_wchar] | ctypes.c_wchar],
) -> str: ...
@overload
def ctypes_field(
    type: type[ctypes.c_void_p],
) -> int | None: ...
@overload
def ctypes_field(
    type: type[ctypes.c_char_p],
) -> bytes | None: ...
@overload
def ctypes_field(
    type: type[ctypes.c_wchar_p],
) -> str | None: ...
@overload
def ctypes_field(type: type["ctypes._CDataType"]) -> bytes: ...


# @overload
# def ctypes_field[T: type["ctypes._CDataType"]](type: T) -> T: ...
def ctypes_field(type) -> Any:
    return CtypesFieldInfo(type)


class Example(Structure):
    testing1 = ctypes_field(ctypes.c_byte * 5)
    testing2 = ctypes_field(ctypes.c_char * 5)
    testing3 = ctypes_field(ctypes.c_uint16)


print(Example)

example = Example.from_buffer_copy(b"asd\x00fasd\x00f\xff\xff\xff\xff")
print(example)
print(
    example.testing1,
    example.testing1,
    example.testing1,
    type(example.testing1),
    example.testing2,
    type(example.testing2),
    example.testing3,
    type(example.testing3),
)
