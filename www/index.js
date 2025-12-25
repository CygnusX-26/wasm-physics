import { World } from "wasm-physics";
import { memory } from "wasm-physics/wasm_physics_bg.wasm";

// const AVOID_FACTOR: f32 = 0.09;
// const VISIBLE_RANGE: f32 = 30.0;
// const MATCHING_FACTOR: f32 = 0.04;
// const TURN_FACTOR: f32 = 0.05;
// const MAX_SPEED: f32 = 10.0;
// const MIN_SPEED: f32 = 1.0;
// const CENTERING_FACTOR: f32 = 0.00045;
// const PROTECTED_RANGE: f32 = 10.0;
// const PREDATOR_TURN_FACTOR: f32 = 0.3;
// const PREDATOR_RANGE: f32 = 45.0;
const avoidFactor = document.getElementById("avoid-factor");
const avoidFactorValue = document.getElementById("avoid-factor-value");
const centeringFactor = document.getElementById("centering-factor");
const centeringFactorValue = document.getElementById("centering-factor-value");
const protectedRange = document.getElementById("protected-range");
const protectedRangeValue = document.getElementById("protected-range-value");
const world = World.new(
    700,
    700,
    600,
    Number(avoidFactor.value),
    30.0, //VISIBLE_RANGE
    0.04, //MATCHING_FACTOR
    0.05, //TURN_FACTOR
    1.0, // MIN_SPEED
    10.0, // MAX_SPEED
    Number(centeringFactor.value), //CENTERING_FACTOR
    Number(protectedRange.value), //PROTECTED_RANGE
    0.3, //PREDATOR_TURN_FACTOR
    45.0, //PREDATOR_RANGE
);
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

canvas.addEventListener("mousemove", (e) => {
    const rect = canvas.getBoundingClientRect();

    const cssX = e.clientX - rect.left;
    const cssY = e.clientY - rect.top;
    const x = cssX * (canvas.width / rect.width);
    const y = cssY * (canvas.height / rect.height);

    world.set_predator_loc(x, y);
});

const drawBoids = () => {
    const boidsPtr = world.render_xy_ptr();
    const boids = new Float32Array(memory.buffer, boidsPtr, numBoids);

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (let i = 0; i < numBoids / 2; i++) {
        ctx.fillRect(boids[i * 2], boids[i * 2 + 1], 4, 4);
    }
};

const updateAvoidFactor = () => {
    let v = (Number(avoidFactor.value) / 10) * 0.18;
    avoidFactorValue.textContent = avoidFactor.value;
    world.update_avoid_factor(v);
};

const updateCenteringFactor = () => {
    let v = (Number(centeringFactor.value) / 10) * 10 * 0.000045;
    centeringFactorValue.textContent = centeringFactor.value;
    world.update_centering_factor(v);
};

const updateProtectedRange = () => {
    let v = (Number(protectedRange.value) / 10) * 20.0;
    protectedRangeValue.textContent = protectedRange.value;
    world.update_protected_range(v);
};

avoidFactor.addEventListener("input", updateAvoidFactor);
centeringFactor.addEventListener("input", updateCenteringFactor);
protectedRange.addEventListener("input", updateProtectedRange);

updateAvoidFactor();
updateCenteringFactor();
updateProtectedRange();
drawBoids();
renderLoop();
