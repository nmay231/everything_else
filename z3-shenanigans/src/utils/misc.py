import z3


def is_datatype_sort(s) -> bool:
    return s.kind() == z3.Z3_DATATYPE_SORT


def is_datatype_constructor(x) -> bool:
    s = x.sort()
    if is_datatype_sort(s):
        n = s.num_constructors()
        f = x.decl()
        for i in range(n):
            c = s.constructor(i)
            if z3.eq(c, f):
                return True
    return False


# Return True if x is a constant constructor, that is, a constructor without arguments.
def is_datatype_const_value(x) -> bool:
    return z3.is_const(x) and is_datatype_constructor(x)


def is_uninterpreted_constant(a) -> bool:
    # https://stackoverflow.com/questions/12253088/how-to-check-if-a-const-in-z3-is-a-variable-or-a-value
    return (
        z3.is_const(a)
        and a.decl().kind() == z3.Z3_OP_UNINTERPRETED
        and not is_datatype_const_value(a)
    )
