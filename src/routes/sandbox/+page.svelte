<script lang="ts">
	import Chart from '$lib/Chart.svelte';
	import { newDataset, nextColour, type Point } from '$lib/data';
	import Padded from '$lib/Padded.svelte';
	import { findChange } from '$lib/question';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { ChartDataset } from 'chart.js';

	const simulation = { defaults: [0.83, 0.72, 0.32, 0.56] };
	const editCompounds = ['CH3COOH(aq)', 'NH3(aq)', 'CH3COO(aq)', 'NH4(aq)'];
	const strCompounds = [
		'CH3COOH<sub>(aq)</sub>',
		'NH3<sub>(aq)</sub>',
		'CH3COO<sup>-</sup><sub>(aq)</sub>',
		'NH4<sup>+</sup><sub>(aq)</sub>'
	];
	const eqStr = 'CH3COOH(aq) + NH3(aq) <-> CH3COO(aq) + NH4(aq)';
	const idx = 9;
	let direction = 'No shift';
	let current = [...simulation.defaults];
	let changes = [...simulation.defaults];
	let show = false;

	let datasets = [] as ChartDataset[];
	for (const [idx, elm] of editCompounds.entries()) {
		datasets.push(
			newDataset(
				elm,
				[
					{ x: 0, y: simulation.defaults[idx] },
					{ x: 1, y: simulation.defaults[idx] }
				],
				nextColour(idx)
			)
		);
	}
	let chartData = { datasets: datasets };
	invoke('add_system', {
		eqStr: eqStr,
		idx: idx,
		concentrations: simulation.defaults,
		reset: false
	}).catch((e) => console.error(e));

	function update(idx: number) {
		let tmp = changes[idx];
		for (let i = 0; i < changes.length; i++) {
			changes[i] = current[i];
		}
		changes[idx] = tmp;
	}

	function getDirection(dir: string): string {
		if (dir === 'Forward') {
			return 'Shift to the right';
		} else if (dir === 'Reverse') {
			return 'Shift to the left';
		} else {
			return 'No shift';
		}
	}

	async function submit() {
		const change = findChange(changes, current, editCompounds);
		// no change has been made
		if (change[0] === '') {
			return;
		}

		// get the direction the equilibrium shifted
		try {
			direction = await invoke('get_shift_direction', {
				idx: idx,
				adjust: { Concentration: change }
			});
		} catch (e) {
			console.error(e);
			return;
		}

		try {
			await invoke('set_sys_concentration', {
				idx: idx,
				concentrations: current
			});
			await invoke('update_system', {
				idx: idx,
				adjust: { Concentration: change }
			});
		} catch (e) {
			console.error(e);
			return;
		}

		// update questions
		try {
			changes = await invoke('get_sys_concentration', { idx: idx });
			current = [...changes];
		} catch (e) {
			console.error(e);
			return;
		}

		const changeIdx = editCompounds.indexOf(change[0]);
		let setLength = (datasets[0].data[datasets[0].data.length - 1] as Point).x + 1;
		for (let i = 0; i < datasets.length; i++) {
			let y = i === changeIdx ? change[1] : (datasets[i].data[setLength - 1] as Point).y;
			datasets[i].data.push({ x: setLength - 0.7, y: y });
		}
		for (let i = 0; i < datasets.length; i++) {
			datasets[i].data.push({ x: setLength, y: current[i] });
			datasets[i].data.push({ x: setLength + 1, y: current[i] });
		}
		chartData.datasets = datasets;
	}

	async function reset() {
		changes = [...simulation.defaults];
		current = [...simulation.defaults];
		datasets = [] as ChartDataset[];

		try {
			await invoke('set_sys_concentration', {
				idx: idx,
				concentrations: simulation.defaults
			});
		} catch (e) {
			console.error(e);
		}
		for (const [idx, elm] of editCompounds.entries()) {
			datasets.push(
				newDataset(
					elm,
					[
						{ x: 0, y: simulation.defaults[idx] },
						{ x: 1, y: simulation.defaults[idx] }
					],
					nextColour(idx)
				)
			);
		}
		chartData = { datasets: datasets };
	}
</script>

<div class="main">
	<h1>Sandbox</h1>
	<div class="home">
		<Padded href="/">Main Menu</Padded>
	</div>

	<span class="equation"
		>CH3COOH<sub>(aq)</sub> + NH3<sub>(aq)</sub> ↔ CH3COO<sup>-</sup><sub>(aq)</sub> + NH4<sup
			>+</sup
		><sub>(aq)</sub></span
	>

	<div class="interactive">
		{#each changes as val, idx}
			<input
				id={String(idx)}
				bind:value={val}
				on:input={() => update(idx)}
				type="range"
				min="0.01"
				max="3"
				step="0.01"
			/>
			<label for={String(idx)}>{@html strCompounds[idx]}: {val.toFixed(2)} M</label>
		{/each}
	</div>

	<Chart data={chartData} />

	<button class="button reset" on:click={reset}>Reset</button>

	<button class="button submit" on:click={submit}>Update System</button>

	<button class="button b-help" on:click={() => (show = !show)}
		>{show ? 'Hide Help' : 'Help'}</button
	>
	<p>{getDirection(direction)}</p>

	<div class="help" class:show>
		<p>
			Use the sliders to modify concentrations of the different compounds. Click 'Update System' to
			apply a change to the system and see the results. Click and drag on the graph to scroll left
			and right.
		</p>
	</div>
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.home {
		position: absolute;
		right: 10px;
		top: 30px;
	}

	.equation {
		background-color: #7f7f7f;
		padding: 10px;
		font-weight: bold;

		border: 2px solid #525151;
		border-radius: 0.75rem;
	}

	.interactive {
		display: grid;
		margin-top: 2rem;
		margin-bottom: 2rem;
		column-gap: 2rem;
		row-gap: 0.5rem;
	}

	.interactive label {
		text-align: center;
		grid-row: 1;
	}

	.interactive input {
		grid-row: 0;
	}

	.button {
		font-size: 0.9em;
		color: white;
		text-decoration: none;
		font-weight: bold;

		margin: 10px;
		padding: 8px;
		padding-left: 30px;
		padding-right: 30px;

		border: 2px solid #525151;
		border-radius: 0.75rem;
		background-color: #537bc2;
	}

	.reset {
		position: absolute;
		left: 20px;
		bottom: 20px;

		background-color: #7f7f7f;
		border: 2px solid #525151;
	}

	.reset:hover {
		background-color: #8c8c8c;
	}

	.submit {
		position: absolute;
		right: 20px;
		bottom: 20px;
		background-color: #4472c4;
	}

	.submit:hover {
		background-color: #537bc2;
	}

	.b-help {
		position: absolute;
		top: 20px;
		left: 20px;

		background-color: #7f7f7f;
		border: 2px solid #525151;
	}

	.b-help:hover {
		background-color: #8c8c8c;
	}

	.help {
		flex-direction: column;
		align-items: center;

		position: absolute;
		left: calc(100% / 2 - 314px);
		top: calc(100% / 2 - 100px);

		background-color: rgb(63, 63, 63);
		width: 625px;

		border: 2px solid darkgrey;
		border-radius: 2rem;

		/* hide by default */
		transition: opacity 0.2s ease-in-out;
		opacity: 0;
		display: none;
	}

	.help > p {
		padding: 30px;
	}

	.show {
		opacity: 1;
		display: flex;
	}
</style>
