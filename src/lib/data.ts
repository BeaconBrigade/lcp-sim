import type { ChartDataset } from "chart.js";

export function newDataset(label: string, data: number[], colour: string): ChartDataset {
    return (
        {
            label: label,
            fill: true,
            // the curve of the line
            lineTension: 0.5,
            backgroundColor: 'rgba(184, 185, 210, .3)',
            borderColor: colour,
            borderCapStyle: 'butt',
            borderDash: [],
            borderDashOffset: 0.0,
            borderJoinStyle: 'miter',
            pointBorderColor: colour,
            pointBackgroundColor: 'rgb(255, 255, 255)',
            pointBorderWidth: 10,
            pointHoverRadius: 5,
            pointHoverBorderWidth: 2,
            pointRadius: 3,
            pointHitRadius: 10,
            data: data,
        });
}

const colours = [
    'rgb(237, 218, 45)',
    'rgb(33, 60, 237)',
    'rgb(222, 91, 9)',
    'rgb(5, 245, 9)',
    'rgb(245, 5, 209)',
    'rgb(7, 250, 242)',
]

export function nextColour(idx: number): string {
    return colours[idx];
}
