import { Display } from "rust-ray";
import { memory } from "rust-ray/rust_ray_bg";

const width = 4 * 160;
const height = 4 * 90;
const sample_count = 200;

const display = Display.new(width, height, sample_count);
const canvas = document.getElementById("raytracing-canvas");
const ctx = canvas.getContext('2d');
ctx.canvas.width = display.width();
ctx.canvas.height = display.height();


var data = ctx.createImageData(ctx.canvas.width, ctx.canvas.height);
var buf = new Uint32Array(data.data.buffer);

const drawPixel = (x, y, r, g, b) => {
    const index = x + y * width;
    buf[index] = r + (g << 8) + (b << 16) + (255 << 24);
}

const start = Date.now();

const drawPixels = () => {
    const time = Date.now() - start;
    console.log(time);
    display.tick(time);
    const pixelsPtr = display.pixels();
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, 3 * width * height);
    for (let y = 0; y < display.height(); y++) {
        for (let x = 0; x < display.width(); x++) {
            const index = 3 * (x + y * width);
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