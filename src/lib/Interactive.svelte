<script lang="ts">
	import { QuestionType, type Question } from '$lib/question';

	export let isSubmit = false;
	export let question: Question;
	export let changes: number[];

	$: compounds = question.equation.split(' ').filter((x) => x !== '+' && x !== '↔');
	$: {
		if (question.q.type == QuestionType.MultipleChoice) {
			throw new Error('MC question passed to Interactive');
		}
	}
	function update(idx: number) {
		let tmp = changes[idx];
		for (let i = 0; i < changes.length; i++) {
			changes[i] = question.defaults[i];
		}
		changes[idx] = tmp;
	}
</script>

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
			disabled={isSubmit}
		/>
		<label for={String(idx)}>{compounds[idx]}: {val.toFixed(2)}</label>
	{/each}
</div>

<style>
	.interactive {
		display: flex;
		margin-bottom: 70px;
	}

	div.interactive > label {
		margin-right: 20px;
	}
</style>
