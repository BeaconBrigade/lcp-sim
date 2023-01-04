<script lang="ts">
	import { Line } from 'svelte-chartjs';

	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		LineElement,
		LinearScale,
		PointElement,
		CategoryScale,
		type ChartData
	} from 'chart.js';

	export let data: ChartData;

	ChartJS.register(Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale);
</script>

<div class="main">
	<Line
		{data}
		options={{
			responsize: true,
			scales: { y: { beginAtZero: true } },
			plugins: {
				tooltip: {
					bodySpacing: 15,
					backgroundColor: 'rgba(50, 50, 50, 1)',
					boxWidth: 10,
					boxHeight: 10,
					boxPadding: 10,
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
				}
			}
		}}
	/>
</div>

<style>
	.main {
		margin-top: 40px;
		padding: 10px;
		width: 600px;
		height: 300px;

		border: 2px solid darkgrey;
		border-radius: 2rem;

		background-color: #29292b;
	}
</style>
