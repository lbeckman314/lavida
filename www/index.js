// Import the WebAssembly memory at the top of the file.
import { memory } from "convida/convida_bg";
import { Universe, Cell } from "convida";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

//Construct the universe, and get its width and height.
const universe = Universe.new(100, 100);
//universe.set_size(100, 100);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

if (ctx === null) {
    alert("unable to initialize WebGL.");
}

let animationId = null;

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");
const pre = document.getElementById("pre");


const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
}

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
}

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

const resetButton = document.getElementById("reset");

resetButton.addEventListener("click", event => {
    universe.reset();
    //universe = Universe.new();
    drawGrid();
    drawCells();
})

const clearButton = document.getElementById("clear");

clearButton.addEventListener("click", event => {
    universe.clear();
    drawGrid();
    drawCells();
})

const step = document.getElementById("step");

step.addEventListener("click", event => {
    universe.tick();
    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(mynull);
    pre.textContent = universe.render();
    console.log(pre.textContent);
})

const mynull = () => {
    return 0;
};

canvas.addEventListener("click", event => {
    let index = idx(canvas);
    let row = index.row;
    let col = index.col;

    if (event.ctrlKey) {
        console.log("ctrl");
        universe.glider(row, col);
    }

    else if (event.shiftKey) {
        console.log("shift");
        universe.pulsar(row, col);
    }

    else {
        universe.toggle_cell(row, col);
    }

    drawGrid();
    drawCells();
});

function idx(canvas) {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    return {
        row: row,
        col: col
    };
}


// The result of 'requestAnimationFrame' is assigned to 'animationId'.
const renderLoop = () => {
    //debugger;
    fps.render();
    for (let i = 0; i < 9; i++) {
        universe.tick();
    }

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);

    pre.textContent = universe.render();
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};


const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Alive) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.fillStyle = DEAD_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Dead) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our last 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
}

drawGrid();
drawCells();
//requestAnimationFrame(renderLoop);
play();
