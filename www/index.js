import * as wasm from "rust-ray";

//wasm.greet();


const width = 1000;
const height = 1000;
const canvas = document.getElementById("raytracing-canvas");
const ctx = canvas.getContext('2d');
ctx.canvas.width = width;
ctx.canvas.height = height;

const start = Date.now();

function clamp(num, min, max) {
    return num <= min
        ? min
        : num >= max
            ? max
            : num
}

const toHex = (val, max) => {
    let s = Math.round(clamp(256 * (val / max), 0, 255)).toString(16).toUpperCase();
    while (s.length < 2) {
        s = "0" + s;
    }
    return s;
}

const color = (x, y) => {
    const red = toHex(x, width);
    const green = toHex(y, height);
    return `#${red}${green}00`;
}

const drawPixels = () => {
    ctx.beginPath();
    const time = Date.now() - start;
    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            ctx.fillStyle = color(x, y);
            ctx.fillRect(
                (x + time) % width,
                (y + time) % height,
                1,
                1
            );
        }
    }
    ctx.stroke();
};
const renderLoop = () => {
    drawPixels();
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);