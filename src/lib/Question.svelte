<script lang="ts">
	import { QuestionType, type Question } from '$lib/question';
	import Chart from '$lib/Chart.svelte';
	import type { ChartDataset } from 'chart.js';
	import { newDataset, nextColour } from './data';
	import Explain from '$lib/Explain.svelte';
	import Popup from '$lib/Popup.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	export let question: Question;

	// the forward and back links
	$: back = question.id > 1 ? `/quiz/${question.id - 1}` : '/quiz';
	$: next = `/quiz/${Number(question.id) + 1}`;

	// should we show 'next'/'finish', 'submit' or a disabled version
	let isSubmit = false;
	// show the explanation of the question
	let showExplanation = false;
	// if the user's guess was correct
	let correct = false;

	$: {
		isSubmit = false;
		showExplanation = false;
		correct = false;
		// reset local vars, when question changes
		question;
	}

	// the names of each compound
	$: compounds = question.equation.split(' ').filter((x) => x !== '+' && x !== '↔');

	// data to show on the graph
	$: datasets = [] as ChartDataset[];
	$: {
		datasets = [];
		for (const [idx, elm] of compounds.entries()) {
			datasets.push(
				newDataset(elm, [question.defaults[idx], question.defaults[idx]], nextColour(idx))
			);
		}
	}

	$: chartData = {
		labels: [0, 1, 2, 3, 4, 5],
		datasets: datasets
	};

	$: {
		// initialize system
		invoke('add_system', {
			eqStr: question.equation.replace('↔', '<->'),
			idx: question.id - 1,
			concentrations: question.defaults
		}).catch((e) => console.error(e));
	}

	// check if question was correct
	async function submit() {
		if (question.q.type == QuestionType.MultipleChoice) {
			correct = question.q.correct == question.q.selected;
			invoke('update_system', {
				idx: question.id - 1,
				// the || 0 doesn't do anything since to submit, selected can't
				// be undefined
				adjust: question.q.actions[question.q.selected || 0]
			}).catch((e) => console.error(e));
		} else {
			// TODO: implement checking if correct for interactive questions
			correct = question.q.isRight([-1, 2.0]);
		}
		isSubmit = true;

		// update graph
		try {
			const concentrations: number[] = await invoke('get_sys_concentration', {
				idx: question.id - 1
			});
			for (let i = 0; i < datasets.length; i++) {
				datasets[i].data.push(concentrations[i]);
				datasets[i].data.push(concentrations[i]);
			}
			chartData.datasets = datasets;
		} catch (e) {
			console.error(e);
		}
	}
</script>

<div class="main">
	{#if question.q.type === QuestionType.MultipleChoice}
		<div class="mc">
			{#each question.q.options as opt, idx}
				<input
					id={String(idx)}
					bind:group={question.q.selected}
					type="radio"
					name="mc-ans"
					value={idx}
				/>
				<label for={String(idx)}>{opt}</label><br />
			{/each}
		</div>
	{:else}
		<div class="interactive">
			{#each question.q.change as val, idx}
				<input id={String(idx)} bind:value={val} type="range" min="0" max="3" step="0.01" />
				<label for={String(idx)}>{compounds[idx]}: {val.toFixed(2)}</label>
			{/each}
		</div>
	{/if}

	<Chart data={chartData} />

	<!-- if we aren't on the first question go back to the quiz -->
	<a class="back" href={back}>Back</a>

	<!-- If we're at the end have the button say done -->
	{#if isSubmit}
		{#if question.id < 9}
			<!--
			I don't know why I need to manually set the href... but it doesn't
			work without it
			-->
			<a class="next" href={next}>Next</a>
		{:else}
			<a class="next finish" href="/quiz">Finish</a>
		{/if}
	{:else}
		<button
			on:click={submit}
			disabled={question.q.type == QuestionType.MultipleChoice &&
				!(question.q.selected !== undefined)}
			class="next">Submit</button
		>
	{/if}

	<Explain {question} show={showExplanation} />

	<button class="next explain" on:click={() => (showExplanation = !showExplanation)}
		>{showExplanation ? 'Hide' : 'Show'} Explanation</button
	>

	<Popup checked={correct} show={isSubmit} />
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.interactive {
		display: flex;
	}

	div.interactive > label {
		margin-right: 20px;
	}

	.mc {
		display: block;
	}

	.mc > input {
		accent-color: #3475f7;
		width: 20px;
		height: 20px;
		margin-bottom: 15px;
		position: relative;
		top: 5px;
	}

	.back {
		position: absolute;
		left: 20px;
		bottom: 20px;

		background-color: #7f7f7f;
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
	}

	.back:hover {
		background-color: #8c8c8c;
	}

	.next {
		position: absolute;
		right: 20px;
		bottom: 20px;

		background-color: #4472c4;
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
	}

	.next:hover {
		background-color: #537bc2;
	}

	.next:disabled {
		filter: grayscale(0.5);
		color: lightgray;
	}

	.finish {
		background-color: #70ad47;
	}

	.finish:hover {
		background-color: #88c95d;
	}

	.explain {
		bottom: 65px;
		background-color: #7f7f7f;
	}

	.explain:hover {
		background-color: #8c8c8c;
	}
</style>
