import type { ChartDataset } from "chart.js";

export const data = {
    labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
    datasets: [
        {
            label: 'My First dataset',
            fill: true,
            lineTension: 0.3,
            backgroundColor: 'rgba(225, 204,230, .3)',
            borderColor: 'rgb(205, 130, 158)',
            borderCapStyle: 'butt',
            borderDash: [],
            borderDashOffset: 0.0,
            borderJoinStyle: 'miter',
            pointBorderColor: 'rgb(205, 130, 158)',
            pointBackgroundColor: 'rgb(255, 255, 255)',
            pointBorderWidth: 10,
            pointHoverRadius: 5,
            pointHoverBackgroundColor: 'rgb(0, 0, 0)',
            pointHoverBorderColor: 'rgba(220, 220, 220, 1)',
            pointHoverBorderWidth: 2,
            pointRadius: 1,
            pointHitRadius: 10,
            data: [65, 59, 80, 81, 56, 55, 40],
        },
        {
            label: 'My Second dataset',
            fill: true,
            lineTension: 0.3,
            backgroundColor: 'rgba(184, 185, 210, .3)',
            borderColor: 'rgb(35, 26, 136)',
            borderCapStyle: 'butt',
            borderDash: [],
            borderDashOffset: 0.0,
            borderJoinStyle: 'miter',
            pointBorderColor: 'rgb(35, 26, 136)',
            pointBackgroundColor: 'rgb(255, 255, 255)',
            pointBorderWidth: 10,
            pointHoverRadius: 5,
            pointHoverBackgroundColor: 'rgb(0, 0, 0)',
            pointHoverBorderColor: 'rgba(220, 220, 220, 1)',
            pointHoverBorderWidth: 2,
            pointRadius: 1,
            pointHitRadius: 10,
            data: [28, 48, 40, 19, 86, 27, 90],
        },
    ],
};

export function newDataset(label: string, data: number[], colour: string): ChartDataset {
    return (
        {
            label: label,
            fill: true,
            // lineTension: 0.3,
            backgroundColor: 'rgba(184, 185, 210, .3)',
            borderColor: colour,
            borderCapStyle: 'butt',
            borderDash: [],
            borderDashOffset: 0.0,
            borderJoinStyle: 'miter',
            pointBorderColor: 'rgb(35, 26, 136)',
            pointBackgroundColor: 'rgb(255, 255, 255)',
            pointBorderWidth: 10,
            pointHoverRadius: 5,
            pointHoverBackgroundColor: 'rgb(0, 0, 0)',
            pointHoverBorderColor: 'rgba(220, 220, 220, 1)',
            pointHoverBorderWidth: 2,
            pointRadius: 1,
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
