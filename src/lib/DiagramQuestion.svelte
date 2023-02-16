<script lang="ts">
	import DiagramChart from './DiagramChart.svelte';
	import { correctDataset, incorrectDataset, originalDataset } from './diagramData';
	import Explain from './Explain.svelte';
	import Padded from './Padded.svelte';
	import Popup from './Popup.svelte';
	import { questions, QuestionType } from './question';

	const question = questions[4];
	if (question.q.type === QuestionType.Interactive) {
		throw new Error('not possible!');
	}
	const mc = question.q;
	const back = '/quiz/4';
	const next = '/quiz';
	let chartData = { datasets: [originalDataset] };
	let isSubmit = false;
	let showPopup = false;
	let showExplanation = false;
	let correct = false;
	let selected: number | null = null;

	function submit() {
		if (selected === null) {
			return;
		}
		isSubmit = true;
		showPopup = true;
		correct = mc.correct == selected;
		if (correct) {
			chartData.datasets.unshift(correctDataset);
		} else {
			chartData.datasets.unshift(incorrectDataset);
		}
		chartData.datasets = chartData.datasets;
		setTimeout(() => (showPopup = false), 1000);
	}
	function reset() {
		isSubmit = false;
		showPopup = false;
		showExplanation = false;
		correct = false;
		selected = null;
		chartData = { datasets: [originalDataset] };
	}
</script>

<div class="main">
	<div class="home">
		<Padded href="/">Main Menu</Padded>
	</div>
	<span class="floating">5/5</span>

	<span class="equation"
		>CH<sub>3</sub>COOH<sub>(aq)</sub> + H<sub>2</sub>O(aq) ↔ CH<sub>3</sub>COO<sup>-</sup><sub
			>(aq)</sub
		>
		+ H<sub>3</sub>O<sup>+</sup><sub>(aq)</sub></span
	>
	<p>Temperature is increased at t<sub>1</sub>, what is this reaction?</p>

	<div class="mc">
		<input
			id="endo"
			bind:group={selected}
			type="radio"
			name="mc-ans"
			value={0}
			disabled={isSubmit}
		/>
		<label for="endo">Endothermic</label><br />
		<input
			id="exo"
			bind:group={selected}
			type="radio"
			name="mc-ans"
			value={1}
			disabled={isSubmit}
		/>
		<label for="exo">Exothermic</label><br />
	</div>

	<DiagramChart data={chartData} />

	<a class="back" href={back}>Back</a>

	<!-- If we're at the end have the button say done -->
	{#if isSubmit}
		<a class="next finish" href={next}>Finish</a>
		<button on:click={reset} class="retry">Retry</button>
	{:else}
		<button on:click={submit} disabled={selected === null} class="next">Submit</button>
	{/if}

	<Explain {question} show={showExplanation} changes={[]} {selected} lastChange={null} />

	<button class="next explain" on:click={() => (showExplanation = !showExplanation)}
		>{showExplanation ? 'Hide' : 'Show'} Explanation</button
	>

	<Popup checked={correct} show={showPopup} />
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.equation {
		/* maybe get a different font... */
		margin-top: 20px;
		background-color: #7f7f7f;
		padding: 10px;
		font-weight: bold;

		border: 2px solid #525151;
		border-radius: 0.75rem;
	}

	.floating {
		position: absolute;
		left: 10px;
		top: 10px;
		font-size: 2.5rem;
	}

	.home {
		position: absolute;
		right: 10px;
		top: 30px;
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

	.retry {
		position: absolute;
		right: 130px;
		bottom: 20px;

		background-color: #bdb000;
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

	.retry:hover {
		background-color: #ccbe00;
	}
</style>
