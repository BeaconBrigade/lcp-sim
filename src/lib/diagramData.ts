import { newDataset } from './data';

const originalData = [
	{ x: 0, y: 2 },
	{ x: 1, y: 2 },
	{ x: 1.1, y: 1.8 },
	{ x: 1.2, y: 1.6 },
	{ x: 1.3, y: 1.4 },
	{ x: 1.4, y: 1.25 },
	{ x: 1.5, y: 1.1 },
	{ x: 1.6, y: 1.0 },
	{ x: 1.7, y: 0.9 },
	{ x: 1.8, y: 0.8 },
	{ x: 1.9, y: 0.75 },
	{ x: 2.0, y: 0.7 },
	{ x: 2.1, y: 0.67 },
	{ x: 2.2, y: 0.65 },
	{ x: 3, y: 0.65 }
];

export const originalDataset = newDataset('H3O', originalData, '255, 255, 0');

export const correctDataset = newDataset('H3O', originalData, '0, 255, 0');

const wrongData = [
	{ x: 0, y: 2 },
	{ x: 1, y: 2 },
	{ x: 1.1, y: 2.2 },
	{ x: 1.2, y: 2.4 },
	{ x: 1.3, y: 2.6 },
	{ x: 1.4, y: 2.75 },
	{ x: 1.5, y: 2.9 },
	{ x: 1.6, y: 3.0 },
	{ x: 1.7, y: 3.1 },
	{ x: 1.8, y: 3.2 },
	{ x: 1.9, y: 3.25 },
	{ x: 2.0, y: 3.3 },
	{ x: 2.1, y: 3.33 },
	{ x: 2.2, y: 3.35 },
	{ x: 3, y: 3.35 }
];

export const incorrectDataset = newDataset('H3O', wrongData, '255, 0, 0');
