<script lang="ts">
	import type { Question } from '$lib/question';
	import { QuestionType } from '$lib/question';

	export let question: Question;

	let elements = question.equation.split(' ').filter((x) => x !== '+' && x !== '↔');
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
				<label for={String(idx)}>{elements[idx]}: {val.toFixed(2)}</label>
			{/each}
		</div>
	{/if}
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
</style>
