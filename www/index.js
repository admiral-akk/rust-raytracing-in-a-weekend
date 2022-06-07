import { Display } from "rust-ray";
import { memory } from "rust-ray/rust_ray_bg";

const width = 1000;
const height = 1000;

const display = Display.new(width, height);
const canvas = document.getElementById("raytracing-canvas");
const ctx = canvas.getContext('2d');
ctx.canvas.width = display.width();
ctx.canvas.height = display.height();


var data = ctx.createImageData(ctx.canvas.width, ctx.canvas.height);
var buf = new Uint32Array(data.data.buffer);

const drawPixel = (x, y, r, g, b) => {
    const index = x * width + y;
    buf[index] = r + (g << 8) + (b << 16) + (255 << 24);
}

const start = Date.now();

const drawPixels = () => {
    display.tick();
    const pixelsPtr = display.pixels();
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, 3 * width * height);
    for (let x = 0; x < display.width(); x++) {
        for (let y = 0; y < display.height(); y++) {
            const index = 3 * (x * width + y);
            drawPixel(x, y, pixels[index], pixels[index + 1], pixels[index + 2]);
        }
    }
    ctx.putImageData(data, 0, 0);
};

const renderLoop = () => {
    drawPixels();
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);