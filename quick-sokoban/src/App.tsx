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

function imageFor(gs: GridSlot) {
    return `/images/sokoban-${gs}.png`;
}

type GridSlot = (typeof gridNameMap)[keyof typeof gridNameMap];
type Coord = [number, number];

function indexToCoord(index: number, width: number): Coord {
    return [Math.floor(index / width), index % width];
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
    grid: GridSlot[][];
    player: Coord;
    blocks: Coord[];
    goals: Coord[];
    backgroundImages: string[];

    constructor(game: string) {
        if (!game.trim()) {
            throw Error("game string empty");
        }

        this.grid = game
            .trim()
            .split("\n")
            .map((row) =>
                row
                    .trim()
                    .toLowerCase()
                    .split("")
                    .map((x) => gridNameMap[x as keyof typeof gridNameMap]),
            );

        const widths = new Set(this.grid.map((row) => row.length));
        if (widths.size != 1) {
            throw Error("row lengths must be the same");
        }

        // Why the heck do sets not come with a pop method...
        this.width = widths.values().next().value as number;
        this.height = this.grid.length;

        const flatGrid = this.grid.flat();
        let player = null as null | Coord;
        this.goals = [];
        this.blocks = [];

        for (const [i, value] of flatGrid.entries()) {
            if (value === "goal") {
                this.goals.push(this.indexToCoord(i));
            } else if (value === "player") {
                if (player) {
                    throw Error("multiple players found, not allowed since you have no friends D:");
                }
                player = this.indexToCoord(i);
            } else if (value === "block") {
                this.blocks.push(this.indexToCoord(i));
            }
        }

        if (!player) {
            throw Error("Must provide player");
        } else {
            this.player = player;
        }
        if (!this.goals.length) {
            throw Error("Must have at least one goal");
        }
        if (this.blocks.length < this.goals.length) {
            throw Error("Must have more blocks than goals");
        }

        const background_filter = new Set<GridSlot>(["empty", "goal", "wall"]);
        this.backgroundImages = this.grid
            .flat()
            .map((slot) => imageFor(background_filter.has(slot) ? slot : "empty"));
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

    topImages(): Map<number, string> {
        const blockImage = imageFor("block");
        return new Map([
            ...this.blocks.map((block) => [this.coordToIndex(block), blockImage] as const),
            [this.coordToIndex(this.player), imageFor("player")],
        ]);
    }

    onkeydown(event: KeyboardEvent) {
        const direction = keyboardMap.get(event.code.toLowerCase());
        if (!direction) {
            return;
        }

        const diff = directionVector.get(direction)!;
        this.player = addCoord(this.player, diff);
    }
}

const puzzle = `
WWWW
WPGW
WEBW
WWWW
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
    }, []);

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
