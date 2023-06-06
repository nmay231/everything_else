import { useMemo } from "react";
import "./App.css";

const CELL_SIZE = 64; // Pixel-length of images

const gridNameMap = {
    b: "block",
    e: "empty",
    p: "player",
    w: "wall",
    g: "goal",
} as const;

type GridSlot = (typeof gridNameMap)[keyof typeof gridNameMap];
type Coord = [number, number];

function indexToCoord(index: number, width: number): Coord {
    return [Math.floor(index / width), index % width];
}

class GameState {
    w: number;
    h: number;
    grid: GridSlot[][];
    player: Coord;
    // blocks: Coord[];
    goals: Coord[];

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
        this.w = widths.values().next().value as number;
        this.h = this.grid.length;

        const flatGrid = this.grid.flat();
        let player = null as null | Coord;
        this.goals = [];
        let blocks = 0;

        for (const [i, value] of flatGrid.entries()) {
            if (value === "goal") {
                this.goals.push(indexToCoord(i, this.w));
            } else if (value === "player") {
                if (player) {
                    throw Error("multiple players found, not allowed since you have no friends D:");
                }
                player = indexToCoord(i, this.w);
            } else if (value === "block") {
                blocks += 1;
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
        if (blocks < this.goals.length) {
            throw Error("Must have more blocks than goals");
        }
    }

    images(): string[] {
        return this.grid.flat().map((slot) => `/images/sokoban-${slot}.png`);
    }
}

const puzzle = `
WWWW
WPGW
WEBW
WWWW
`;

const grid = new GameState(puzzle);

function App() {
    const images = useMemo(() => grid.images(), []);
    const width = grid.w;
    const height = grid.h;

    return (
        <>
            <div
                style={{
                    display: "grid",
                    gridTemplateColumns: `repeat(${width}, ${CELL_SIZE}px [col])`,
                    gridTemplateRows: `repeat(${height}, ${CELL_SIZE}px [row])`,
                }}
            >
                {images.map((src) => [
                    <img src={src} style={{ imageRendering: "pixelated", scale: "4" }} />,
                    // <div></div>,
                ])}
            </div>
        </>
    );
}

export default App;
