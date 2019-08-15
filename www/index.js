import { Universe, Cell } from "wasm-hello-world";
import { memory} from "wasm-hello-world/wasm_hello_world_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const height = 150;
const width = 150;
const universe = Universe.new(height, width);
const canvas = document.getElementById("wasm-hello-world-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
    return row * width + column;
};

async function renderLoop() {
    await sleep(30);
    universe.tick();

    drawGrid();
    drawCells();
    requestAnimationFrame(renderLoop);
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}


const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, i * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const drawCells = () => {
    const cellsPtr = universe.get_cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            ctx.fillStyle = cells[idx] === Cell.Alive ? ALIVE_COLOR : DEAD_COLOR;
            ctx.fillRect(row * (CELL_SIZE + 1) + 1, col * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
        }
    }
};

requestAnimationFrame(renderLoop);
