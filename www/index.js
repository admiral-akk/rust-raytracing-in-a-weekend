import { Display } from "rust-ray";
import { memory } from "rust-ray/rust_ray_bg";

const width = 1000;
const height = 1000;

const display = Display.new(width, height);
const canvas = document.getElementById("raytracing-canvas");
const ctx = canvas.getContext('2d');
ctx.canvas.width = display.width();
ctx.canvas.height = display.height();

const start = Date.now();

function clamp(num, min, max) {
    return num <= min
        ? min
        : num >= max
            ? max
            : num
}

const toHex = (val) => {
    let s = Math.round(clamp(val, 0, 256)).toString(16).toUpperCase();
    while (s.length < 2) {
        s = "0" + s;
    }
    return s;
}

const color = (r, g, b) => {
    return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

const drawPixels = () => {
    console.log("rendering");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.beginPath();
    display.tick();
    const pixelsPtr = display.pixels();
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, 3 * width * height);
    for (let x = 0; x < display.width(); x++) {
        for (let y = 0; y < display.height(); y++) {
            const index = 3 * (x * width + y);
            ctx.fillStyle = color(pixels[index], pixels[index + 1], pixels[index + 2]);
            ctx.fillRect(x, y, 1, 1);
        }
    }
    ctx.stroke();
};

const renderLoop = () => {
    drawPixels();
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);