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

<Line {data} options={{ responsize: true, scales: { y: { beginAtZero: true } } }} />
