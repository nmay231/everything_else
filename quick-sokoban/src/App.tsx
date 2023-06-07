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
    s: "stacked",
    _: "true-empty",
} as const;

function backgroundSlotOnly(slot: GridSlot | "stacked"): BackgroundSlot {
    if (slot === "player" || slot === "block") {
        return "empty";
    } else if (slot === "stacked") {
        return "goal";
    }
    return slot;
}

function imageFor(gs: GridSlot) {
    return `/images/sokoban-${gs}.png`;
}

type BackgroundSlot = "empty" | "wall" | "goal" | "true-empty";
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
const _oppositeDirection = new Map<Direction, Direction>([
    [Direction.UP, Direction.DOWN],
    [Direction.DOWN, Direction.UP],
    [Direction.LEFT, Direction.RIGHT],
    [Direction.RIGHT, Direction.LEFT],
]);
function oppositeDirection(dir: Direction) {
    return _oppositeDirection.get(dir)!;
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
const _directionVector = new Map<Direction, Coord>([
    [Direction.UP, [0, -1]],
    [Direction.RIGHT, [1, 0]],
    [Direction.DOWN, [0, 1]],
    [Direction.LEFT, [-1, 0]],
]);

function dirToCoord(dir: Direction) {
    return _directionVector.get(dir)!;
}

type History = {
    dir: Direction;
    blockPushed: boolean;
};

class GameState {
    width: number;
    height: number;
    grid: BackgroundSlot[][];
    player: Coord;
    blocks: Set<number>;
    goals: Set<number>;
    backgroundImages: string[];
    history = { forward: [] as History[], backward: [] as History[] };
    alreadySolved = false;

    constructor(
        game: string,
        public onWin: () => void,
        public keybinds: Record<string, () => void>,
    ) {
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
            } else if (value === "stacked") {
                this.goals.add(i);
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

        // These are only useful if I add a puzzle editor
        // if (!this.goals.size) {
        //     throw Error("Must have at least one goal");
        // }
        // if (this.blocks.size < this.goals.size) {
        //     throw Error("Must have more blocks than goals");
        // }
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
        const key = event.key.toLowerCase();

        if (key in this.keybinds) {
            this.keybinds[key]();
            return;
        }

        if (key === "z") {
            this.tryUndo();
            return;
        }

        if (key === "y") {
            this.tryRedo();
            return;
        }

        const direction = keyboardMap.get(event.code.toLowerCase());
        if (!direction) {
            return;
        }

        // Prepare for branching hell!
        // I could have defined a OOP approach (one of the few uses of OOP) where objects defined the response to certain actions in a systematic way, but I don't have enough time...

        const diff = dirToCoord(direction);
        const oneOver = addCoord(this.player, diff);

        if (this.gridAt(oneOver) === "wall") {
            return;
        }
        const history: History = { dir: direction, blockPushed: false };
        const twoOver = addCoord(oneOver, diff);
        const oneOverIndex = this.coordToIndex(oneOver);
        if (this.blocks.has(oneOverIndex)) {
            history.blockPushed = true;
            const twoOverIndex = this.coordToIndex(twoOver);
            if (this.gridAt(twoOver) === "wall" || this.blocks.has(twoOverIndex)) {
                return;
            }
            this.blocks.delete(oneOverIndex);
            this.blocks.add(twoOverIndex);
        }
        this.player = oneOver;

        this.history.backward.push(history);
        if (this.history.forward.length) {
            this.history.forward = [];
        }

        for (const goal of this.goals) {
            if (!this.blocks.has(goal)) {
                return;
            }
        }

        if (!this.alreadySolved) {
            this.alreadySolved = true;
            // All goals covered. Player solved it
            // Allow react to render before calling
            setTimeout(() => this.onWin(), 0);
        }
    }

    tryUndo() {
        const action = this.history.backward.pop();
        if (!action) return;
        if (action.blockPushed) {
            const blockIndex = this.coordToIndex(addCoord(this.player, dirToCoord(action.dir)));
            this.blocks.delete(blockIndex);
            this.blocks.add(this.coordToIndex(this.player));
        }
        this.player = addCoord(this.player, dirToCoord(oppositeDirection(action.dir)));
        this.history.forward.push(action);
    }

    tryRedo() {
        const action = this.history.forward.pop();
        if (!action) return;
        if (action.blockPushed) {
            const vec = dirToCoord(action.dir);
            const blockCoord = addCoord(this.player, vec);
            this.blocks.delete(this.coordToIndex(blockCoord));
            this.blocks.add(this.coordToIndex(addCoord(blockCoord, vec)));
        }
        this.player = addCoord(this.player, dirToCoord(action.dir));
        this.history.backward.push(action);
    }
}

const puzzles = [
    `
    WWWWWWW
    WEEPEEW
    WEEEEEW
    WEEBEEW
    WEEEEEW
    WEEGEEW
    WWWWWWW`,
    `
    WWWWWWW
    WEEPEEW
    WEEBEEW
    WEEBEEW
    WEEGEEW
    WEEGEEW
    WWWWWWW`,
    `
    WWWWWWW
    WEEPEEW
    WEEEBGW
    WGEEBEW
    WEEEBGW
    WEEEEEW
    WWWWWWW`,
    `
    ___WWWW_
    WWWWEEW_
    WPBEEEW_
    WWWWWEWW
    WEEEWEEW
    WEEEEEGW
    WWWWWWWW`,
    `
    WWWWWWW
    WEEGEEW
    WEEBEEW
    WGBWBGW
    WEEPEEW
    WWWWWWW`,
    `
    WWWWWWW
    WESEEEW
    WPWBEEW
    WEWEEWW
    WEWEWW_
    WGBEW__
    WWWWW__`,
    `
    WWWWWW_
    WEEGEWW
    WEEWEEW
    WEEBPGW
    WWEWBEW
    _WEEEWW
    _WWWWW_`,
    `
    _______WWWW_
    ______WWPEW_
    WWWWWWWEBEW_
    WEEEEEWESEW_
    WEEEEEEESEW_
    WWWWWWEWSWWW
    _____WEESEEW
    _____WWWGEEW
    _______WWWWW`,
];

// Yay for pseudo-global state?
const onWin = () => {
    if (_grid.currentLevel === _grid.puzzleProgress) {
        if (_grid.currentLevel < puzzles.length - 1) {
            _grid.puzzleProgress += 1;
            document.getElementById("next-level")?.focus();
        } else {
            document.getElementById("win-message")!.style.display = "grid";
            const jsConfetti = new (window as any).JSConfetti();
            jsConfetti.addConfetti();
        }
    }
};

const otherKeybinds = {
    n: () => {
        if (_grid.currentLevel < _grid.puzzleProgress) {
            _grid.currentLevel += 1;
            _grid.game = new GameState(puzzles[_grid.currentLevel], onWin, otherKeybinds);
        }
    },
    p: () => {
        if (_grid.currentLevel > 0) {
            _grid.currentLevel -= 1;
            _grid.game = new GameState(puzzles[_grid.currentLevel], onWin, otherKeybinds);
        }
    },
};

const _grid = proxy({
    puzzleProgress: 0,
    currentLevel: 0,
    game: new GameState(puzzles[0], onWin, otherKeybinds),
});

function App() {
    const grid = useProxy(_grid);
    const game = grid.game;
    useEffect(() => {
        const listener = _grid.game.onkeydown.bind(_grid.game);
        document.body.addEventListener("keydown", listener);
        return () => {
            document.body.removeEventListener("keydown", listener);
        };
    }, [game]);

    const background = game.backgroundImages;
    const topImages = game.topImages();
    const width = game.width;
    const height = game.height;

    return (
        <div>
            <h1>
                Level {grid.currentLevel + 1} of {puzzles.length}
            </h1>
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
            <div
                style={{
                    display: "grid",
                    width: "16rem",
                    margin: "auto",
                    marginTop: "1rem",
                    gridTemplateColumns: "8rem [col] 8rem [col]",
                    gridTemplateRows: "2rem [col] 2rem [col]",
                    justifyItems: "center",
                    alignItems: "center",
                }}
            >
                <button
                    id="undo"
                    onClick={() => grid.game.tryUndo()}
                    disabled={grid.game.history.backward.length <= 0}
                >
                    Undo (z)
                </button>
                <button
                    id="redo"
                    onClick={() => grid.game.tryRedo()}
                    disabled={grid.game.history.forward.length <= 0}
                >
                    Redo (y)
                </button>
                <button id="prev-level" onClick={otherKeybinds.p} disabled={grid.currentLevel <= 0}>
                    Previous level (p)
                </button>
                <button
                    id="next-level"
                    onClick={otherKeybinds.n}
                    disabled={grid.currentLevel >= grid.puzzleProgress}
                >
                    Next level (n)
                </button>
            </div>
        </div>
    );
}

export default App;
