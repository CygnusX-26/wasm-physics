import { World } from "wasm-physics";
import { memory } from "wasm-physics/wasm_physics_bg.wasm";

const world = World.new(500, 500, 150);
const width = world.width();
const height = world.height();
const numBoids = world.render_xy_len();

const canvas = document.getElementById("boid-canvas");
canvas.height = height + 1;
canvas.width = width + 1;

const ctx = canvas.getContext("2d");

const renderLoop = () => {
    world.tick();

    drawBoids();

    requestAnimationFrame(renderLoop);
};

const drawBoids = () => {
    const boidsPtr = world.render_xy_ptr();
    const boids = new Float32Array(memory.buffer, boidsPtr, numBoids);
    console.log("first", boids[0], boids[1]);

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (let i = 0; i < numBoids / 2; i++) {
        ctx.fillRect(boids[i * 2], boids[i * 2 + 1], 2, 2);
    }
};

drawBoids();
renderLoop();
