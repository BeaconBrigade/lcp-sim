<script lang="ts">
	import { Chart } from 'svelte-chartjs';

	import {
		Chart as ChartJS,
		LineController,
		Title,
		Tooltip,
		Legend,
		LineElement,
		LinearScale,
		PointElement,
		CategoryScale,
		type ChartData,
		type Point
	} from 'chart.js';
	import zoomPlugin from 'chartjs-plugin-zoom';

	export let data: ChartData;

	ChartJS.register(
		LineController,
		Title,
		Tooltip,
		Legend,
		LineElement,
		LinearScale,
		PointElement,
		CategoryScale,
		zoomPlugin
	);

	$: length = (data.datasets[0].data[data.datasets[0].data.length - 1] as Point).x;

	// called when data is added to the chart
	// scroll the graph to the right
	export const updateData = () => {
		setTimeout(() => chart.pan({ x: -1 * length * 100 + 100 }, undefined, 'default'), 10);
	};

	let chart: any;
</script>

<div class="wrapper">
	<Chart
		bind:chart
		type="line"
		{data}
		options={{
			responsize: true,
			scales: {
				y: { title: { display: true, text: 'Concentration (mol/L)' }, min: 0.0, max: 3.0 },
				x: { title: { display: true, text: 'Time (s)' }, type: 'linear', max: 5.0 }
			},
			plugins: {
				tooltip: {
					bodySpacing: 15,
					backgroundColor: 'rgba(50, 50, 50, 1)',
					boxWidth: 10,
					boxHeight: 10,
					boxPadding: 10,
					usePointStyle: true,
					callbacks: {
						title: function (items) {
							return `t = ${items[0].label} s`;
						},
						label: function (context) {
							let name = context.dataset.label || '';
							let num = Number(context.parsed.y).toFixed(2);

							return `${name}: ${num} M`;
						}
					}
				},
				zoom: {
					pan: {
						enabled: true,
						mode: 'x',
						scaleMode: 'x'
					},
					limits: { x: { min: -0.5, max: data.datasets[0].data.length - 1.5 } }
				},
				legend: {
					labels: {
						color: 'rgb(255, 255, 255)',
						padding: 30,
						boxHeight: 20,
						pointStyle: 'circle', // could also be 'line'
						usePointStyle: true
					}
				}
			}
		}}
	/>
</div>

<style>
	.wrapper {
		padding: 10px;
		width: 600px;
		height: 300px;

		border: 2px solid darkgrey;
		border-radius: 2rem;

		background-color: #29292b;
	}
</style>
