import "./App.css";

const puzzle = `
WWWW
WPEW
WBBW
WWWW
`;

const gridImageMap = {
    b: "/images/sokoban-block.png",
    e: "/images/sokoban-empty.png",
    p: "/images/sokoban-player.png",
    w: "/images/sokoban-wall.png",
} as const;

const grid: string[][] = puzzle
    .trim()
    .split("\n")
    .map((row) =>
        row
            .trim()
            .toLowerCase()
            .split("")
            .map((key) => gridImageMap[key as keyof typeof gridImageMap]),
    );

function App() {
    console.log(grid);
    return (
        <>
            <div
                style={{
                    display: "grid",
                    gridTemplateColumns: "repeat(4, 64px [col])",
                    gridTemplateRows: "repeat(4, 64px [row])",
                }}
            >
                {grid.flat().map((src) => (
                    <img src={src} style={{ imageRendering: "pixelated", scale: "4" }} />
                ))}
            </div>
        </>
    );
}

export default App;
