<script lang="ts">
	import { QuestionType, type InteractiveQuestion, type Question } from '$lib/question';

	export let isSubmit = false;
	export let question: Question;
	$: compounds = question.equation.split(' ').filter((x) => x !== '+' && x !== '↔');
	let q: InteractiveQuestion;
	$: {
		if (question.q.type == QuestionType.Interactive) {
			q = question.q;
		} else {
			throw new Error('MC question passed to Interactive');
		}
	}

	function update(idx: number) {
		let tmp = q.change[idx];
		for (let i = 0; i < q.change.length; i++) {
			q.change[i] = question.defaults[i];
		}
		q.change[idx] = tmp;
	}
</script>

<div class="interactive">
	{#each q.change as val, idx}
		<input
			id={String(idx)}
			bind:value={val}
			on:input={() => update(idx)}
			type="range"
			min="0"
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
	}

	div.interactive > label {
		margin-right: 20px;
	}
</style>
