import { World } from "wasm-physics";

const world = World.new();
const width = world.width();
const height = world.height();

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
  
}
