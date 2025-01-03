# What is this?

This is a non-exhaustive list of some of the patterns in unicode that are
helpful for drawing in the terminal, and I might use this in the future for
other helpful character patterns. It's put together in a more consumable fashion
than code-point order (though that's listed too for most of them); I've manually
copy pasted characters from Wikipedia and put them into the "right" order often
enough that this should be helpful. That said, the wiki article is still a
helpful resource: https://en.wikipedia.org/wiki/Box-drawing_characters.

## Rounded corners (aka light arc)
```
╭╮
╰╯
```

## Thin boxes (aka light)
```
─│┌┐└┘├┤┬┴┼╴╵╶╷   # Code point order
╵╶└╷│┌├╴┘─┴┐┤┬┼   # Binary counter order
╵╶╰╷│╭├╴╯─┴╮┤┬┼   # Binary counter order (with rounded corners)

┌─┐ ┌┬┐
│ │ ├┼┤           # Boxes
└─┘ └┴┘
```

## Thick boxes (aka heavy)
```
━┃┏┓┗┛┣┫┳┻╋╸╹╺╻   # Code point order
╹╺┗╻┃┏┣╸┛━┻┓┫┳╋   # Binary counter order

┏━┓ ┏┳┓
┃ ┃ ┣╋┫           # Boxes
┗━┛ ┗┻┛
```

## Double boxes
```
═║╔╗╚╝╠╣╦╩╬       # Code point order (no stubs for this group)


╔═╗ ╔╦╗
║ ║ ╠╬╣           # Boxes
╚═╝ ╚╩╝
```

## Double, Triple, and Quadruple dashed lines
```
╌╍╎╏
┄┅┆┇
┈┉┊┋
```

## Mixed thicknesses
```
╒╓╕╖
╘╙╛╜
╞╟╡╢
╤╥╧╨
╪╫

═║
╒╓╔
╕╖╗
╘╙╚
╛╜╝
╞╟╠
╡╢╣
╤╥╦
╧╨╩
╪╫╬
```

```
─━│┃╼╽╾╿
├┝┞┟┠┡┢┣
┤┥┦┧┨┩┪┫
┬┭┮┯┰┱┲┳
┴┵┶┷┸┹┺┻

┌┍┎┏
┐┑┒┓
└┕┖┗
┘┙┚┛

┼┽┾┿
╀╁╂╃
╄╅╆╇
╈╉╊╋
```

## X's
```
╲ ╱
 ╳
╱ ╲
```
