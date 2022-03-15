import { Simulation } from "wasm-game-of-life";

const scale = 100;
const rodLength = 2;
const radius = 10;

const canvas = document.getElementById("canvas");
canvas.height = 700;
canvas.width = 1000;
const ctx = canvas.getContext("2d");

const sim = Simulation.new(
	9.8, rodLength, 0, Math.PI / 4,
);

const render = (time) => {
	const position = sim.theta((time || 0) / 1000);
	ctx.clearRect(0, 0, canvas.width, canvas.height);
	
	ctx.beginPath();
	ctx.moveTo(canvas.width / 2, canvas.height / 2);
	ctx.lineTo(canvas.width / 2 + position.x * scale, position.y * scale + canvas.height / 2);
	ctx.stroke();

	ctx.beginPath();
	ctx.arc(
		canvas.width / 2 + position.x * scale,
		position.y * scale + canvas.height / 2, 
		radius, 
		0, 
		Math.PI * 2
	);
	ctx.fill();

	requestAnimationFrame(render);
};

render();
