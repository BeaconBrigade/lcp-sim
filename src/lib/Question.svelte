<script lang="ts">
	import { findChange, QuestionType, type Question } from '$lib/question';
	import Chart from '$lib/Chart.svelte';
	import type { ChartDataset } from 'chart.js';
	import { newDataset, nextColour, type Point } from './data';
	import Explain from '$lib/Explain.svelte';
	import Popup from '$lib/Popup.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Interactive from './Interactive.svelte';

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
	// the changes for interactive questions
	$: changes = [...question.defaults];
	let selected: number | undefined;

	// need to tell svelte that we only depend on question.id
	$: id = question.id;
	$: {
		isSubmit = false;
		showExplanation = false;
		correct = false;
		selected = undefined;
		// we depend on id (technically all of the question, but
		// we only need to react when there's a change to id)
		id;
	}

	// the names of each compound
	$: compounds = question.equation.split(' ').filter((x) => x !== '+' && x !== '↔');

	// data to show on the graph
	$: datasets = [] as ChartDataset[];
	$: {
		datasets = [];
		for (const [idx, elm] of compounds.entries()) {
			datasets.push(
				newDataset(
					elm,
					[
						{ x: 0, y: question.defaults[idx] },
						{ x: 1, y: question.defaults[idx] }
					],
					nextColour(idx)
				)
			);
		}
	}

	$: chartData = {
		labels: [0, 1, 2, 3, 4, 5],
		datasets: datasets
	};

	// initialize system
	// don't want this to run all the time, but it has to run everything else
	$: {
		invoke('add_system', {
			eqStr: question.equation.replace('↔', '<->'),
			idx: question.id - 1,
			concentrations: question.defaults,
			reset: false
		}).catch((e) => console.error(e));
	}

	// check if question was correct
	async function submit() {
		if (question.q.type == QuestionType.MultipleChoice) {
			// show concentration that the user set
			const action = question.q.actions[selected || 0];
			const changeIdx = compounds.indexOf(action.Concentration[0]);
			for (let i = 0; i < datasets.length; i++) {
				datasets[i].data.push({ x: 5, y: (datasets[i].data[0] as Point).y });
			}
			datasets[changeIdx].data[datasets[changeIdx].data.length - 1] = action.Concentration[1];
			console.log(JSON.stringify(datasets, null, 2));

			correct = question.q.correct == selected;
			try {
				await invoke('update_system', {
					idx: question.id - 1,
					// the || 0 doesn't do anything since to submit, selected can't
					// be undefined by this point
					adjust: action
				});
			} catch (e) {
				console.error(e);
				return;
			}
		} else {
			let change = findChange(changes, question.defaults, compounds);
			// no change has been made
			if (change[0] === '') {
				return;
			}
			try {
				await invoke('set_sys_concentration', {
					idx: question.id - 1,
					concentrations: question.defaults
				});
				await invoke('update_system', {
					idx: question.id - 1,
					adjust: { Concentration: change }
				});
			} catch (e) {
				console.error(e);
				return;
			}

			// update questions
			try {
				changes = await invoke('get_sys_concentration', { idx: question.id - 1 });
				correct = question.q.isRight(changes);
			} catch (e) {
				console.error(e);
				return;
			}
		}
		isSubmit = true;

		// update graph
		try {
			const concentrations: number[] = await invoke('get_sys_concentration', {
				idx: question.id - 1
			});
			let setLength = datasets[0].data.length;
			for (let i = 0; i < datasets.length; i++) {
				datasets[i].data.push({ x: setLength, y: concentrations[i] });
				datasets[i].data.push({ x: setLength + 1, y: concentrations[i] });
			}
			console.log(JSON.stringify(datasets, null, 2));
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
				<input id={String(idx)} bind:group={selected} type="radio" name="mc-ans" value={idx} />
				<label for={String(idx)}>{opt}</label><br />
			{/each}
		</div>
	{:else}
		<Interactive {question} {isSubmit} {changes} />
	{/if}

	<Chart data={chartData} />

	<!-- if we aren't on the first question go back to the quiz -->
	<a class="back" href={back}>Back</a>

	<!-- If we're at the end have the button say done -->
	{#if isSubmit}
		{#if question.id < 9}
			<a class="next" href={next}>Next</a>
		{:else}
			<a class="next finish" href="/quiz">Finish</a>
		{/if}
	{:else}
		<button
			on:click={submit}
			disabled={question.q.type == QuestionType.MultipleChoice && !(selected !== undefined)}
			class="next">Submit</button
		>
	{/if}

	<Explain {question} show={showExplanation} {changes} {selected} />

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
