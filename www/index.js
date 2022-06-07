import * as wasm from "rust-ray";

//wasm.greet();


const width = 100;
const height = 100;
const canvas = document.getElementById("raytracing-canvas");
const ctx = canvas.getContext('2d');
ctx.canvas.width = width;
ctx.canvas.height = height;

function color(x, y) {


}

const drawPixels = () => {
    ctx.beginPath();

    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            ctx.fillStyle = "#00FF00";
            ctx.fillRect(
                x,
                y,
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