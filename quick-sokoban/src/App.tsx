import { useEffect } from "react";
import { proxy } from "valtio";
import { useProxy } from "valtio/utils";

const IMAGE_SIZE = 16;
const IMAGE_SCALE = 4;

const gridNameMap = {
    b: "block",
    e: "empty",
    p: "player",
    w: "wall",
    g: "goal",
} as const;

function backgroundSlotOnly(slot: GridSlot): BackgroundSlot {
    if (slot === "player" || slot === "block") {
        return "empty";
    }
    return slot;
}

function imageFor(gs: GridSlot) {
    return `/images/sokoban-${gs}.png`;
}

type BackgroundSlot = "empty" | "wall" | "goal";
type ForegroundSlot = "block" | "player";
type GridSlot = BackgroundSlot | ForegroundSlot;
type Coord = [number, number];

function indexToCoord(index: number, width: number): Coord {
    return [index % width, Math.floor(index / width)];
}

function coordToIndex(coord: Coord, width: number) {
    return coord[1] * width + coord[0];
}

function addCoord(a: Coord, b: Coord): Coord {
    return [a[0] + b[0], a[1] + b[1]];
}

enum Direction {
    UP = "UP",
    RIGHT = "RIGHT",
    DOWN = "DOWN",
    LEFT = "LEFT",
}
const keyboardMap = new Map<string, Direction>([
    ["arrowup", Direction.UP],
    ["arrowright", Direction.RIGHT],
    ["arrowdown", Direction.DOWN],
    ["arrowleft", Direction.LEFT],
    ["keyw", Direction.UP],
    ["keyd", Direction.RIGHT],
    ["keys", Direction.DOWN],
    ["keya", Direction.LEFT],
]);

// Direction | undefined so you can chain .get()
const directionVector = new Map<Direction | undefined, Coord>([
    [Direction.UP, [0, -1]],
    [Direction.RIGHT, [1, 0]],
    [Direction.DOWN, [0, 1]],
    [Direction.LEFT, [-1, 0]],
]);

class GameState {
    width: number;
    height: number;
    grid: BackgroundSlot[][];
    player: Coord;
    blocks: Set<number>;
    goals: Set<number>;
    backgroundImages: string[];

    constructor(game: string) {
        if (!game.trim()) {
            throw Error("game string empty");
        }

        const grid = game
            .trim()
            .split("\n")
            .map((row) =>
                row
                    .trim()
                    .toLowerCase()
                    .split("")
                    .map((x) => gridNameMap[x as keyof typeof gridNameMap]),
            );

        this.grid = grid.map((row) => row.map(backgroundSlotOnly));
        this.backgroundImages = this.grid.flat().map(imageFor);

        const widths = new Set(this.grid.map((row) => row.length));
        if (widths.size != 1) {
            throw Error("row lengths must be the same");
        }

        // Why the heck do sets not come with a pop method...
        this.width = widths.values().next().value as number;
        this.height = this.grid.length;
        let player = null as null | Coord;
        this.goals = new Set();
        this.blocks = new Set();

        for (const [i, value] of grid.flat().entries()) {
            if (value === "goal") {
                this.goals.add(i);
            } else if (value === "block") {
                this.blocks.add(i);
            } else if (value === "player") {
                if (player) {
                    throw Error("multiple players found, not allowed since you have no friends D:");
                }
                player = this.indexToCoord(i);
            }
        }

        if (!player) {
            throw Error("Must provide player");
        }
        this.player = player;

        if (!this.goals.size) {
            throw Error("Must have at least one goal");
        }
        if (this.blocks.size < this.goals.size) {
            throw Error("Must have more blocks than goals");
        }
    }

    coordToIndex(coord: Coord) {
        return coordToIndex(coord, this.width);
    }

    indexToCoord(index: number) {
        return indexToCoord(index, this.width);
    }

    gridAt(coord: Coord): GridSlot | undefined {
        return this.grid[coord[1]]?.[coord[0]];
    }

    setGrid(coord: Coord, slot: BackgroundSlot) {
        if (this.grid[coord[1]]?.[coord[0]]) {
            this.grid[coord[1]][coord[0]] = slot;
        }
    }

    topImages(): Map<number, string> {
        const blockImage = imageFor("block");
        return new Map([
            ...[...this.blocks].map((block) => [block, blockImage] as const),
            [this.coordToIndex(this.player), imageFor("player")],
        ]);
    }

    onkeydown(event: KeyboardEvent) {
        const direction = keyboardMap.get(event.code.toLowerCase());
        if (!direction) {
            return;
        }

        // Prepare for branching hell!
        // I could have defined a OOP approach (one of the few uses of OOP) where objects defined the response to certain actions in a systematic way, but I don't have enough time...

        const diff = directionVector.get(direction)!;
        const oneOver = addCoord(this.player, diff);

        if (this.gridAt(oneOver) === "wall") {
            return;
        }
        const twoOver = addCoord(oneOver, diff);
        const oneOverIndex = this.coordToIndex(oneOver);
        if (this.blocks.has(oneOverIndex)) {
            const twoOverIndex = this.coordToIndex(twoOver);
            if (this.gridAt(twoOver) === "wall" || this.blocks.has(twoOverIndex)) {
                return;
            }
            this.blocks.delete(oneOverIndex);
            this.blocks.add(twoOverIndex);
        }
        this.player = oneOver;
    }
}

const puzzle = `
WWWWWWWW
WEPEEBEW
WEEEEEEW
WEEEEBEW
WEEEEEGW
WWWWWWWW
`;

const _grid = proxy(new GameState(puzzle));

function App() {
    const grid = useProxy(_grid);
    useEffect(() => {
        const listener = grid.onkeydown.bind(grid);
        document.body.addEventListener("keydown", listener);
        return () => {
            document.body.removeEventListener("keydown", listener);
        };
    }, [grid]);

    const background = grid.backgroundImages;
    const topImages = grid.topImages();
    const width = grid.width;
    const height = grid.height;

    return (
        <>
            <div
                style={{
                    display: "grid",
                    gridTemplateColumns: `repeat(${width}, ${IMAGE_SIZE * IMAGE_SCALE}px [col])`,
                    gridTemplateRows: `repeat(${height}, ${IMAGE_SIZE * IMAGE_SCALE}px [row])`,
                    justifyItems: "center",
                    alignItems: "center",
                    position: "relative",
                }}
            >
                {background.map((background, i) => {
                    const top = topImages.get(i);
                    return (
                        <div
                            key={i}
                            style={{
                                position: "relative",
                            }}
                        >
                            <img
                                style={{
                                    imageRendering: "pixelated",
                                    scale: `${IMAGE_SCALE}`,
                                    position: "absolute",
                                    top: `-${IMAGE_SIZE / 2}px`,
                                    left: `-${IMAGE_SIZE / 2}px`,
                                }}
                                src={background}
                            />
                            {top && (
                                <img
                                    style={{
                                        imageRendering: "pixelated",
                                        scale: `${IMAGE_SCALE}`,
                                        position: "absolute",
                                        top: `-${IMAGE_SIZE / 2}px`,
                                        left: `-${IMAGE_SIZE / 2}px`,
                                    }}
                                    src={top}
                                />
                            )}
                        </div>
                    );
                })}
            </div>
        </>
    );
}

export default App;
