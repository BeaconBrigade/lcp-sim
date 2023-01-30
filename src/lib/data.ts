import type { ChartDataset } from 'chart.js';

export type Point = {
	x: number;
	y: number;
};

export function newDataset(label: string, data: Point[], colour: string): ChartDataset {
	return {
		label: label,
		cubicInterpolationMode: 'monotone',
		backgroundColor: rgba(colour, 0.7),
		borderColor: rgb(colour),
		pointBorderColor: rgb(colour),
		pointBackgroundColor: rgba(colour, 0.7),
		pointBorderWidth: 2,
		pointHoverRadius: 3,
		pointRadius: 3,
		pointHitRadius: 10,
		data: data
	};
}

function rgb(colour: string): string {
	return `rgb(${colour})`;
}

function rgba(colour: string, transparency: number): string {
	return `rgba(${colour}, ${transparency})`;
}

const colours = [
	'237, 218, 45',
	'33, 60, 237',
	'222, 91, 9',
	'5, 245, 9',
	'245, 5, 209',
	'7, 250, 242'
];

export function nextColour(idx: number): string {
	return colours[idx];
}
