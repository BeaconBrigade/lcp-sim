<script lang="ts">
	import { QuestionType, type Question } from '$lib/question';

	export let question: Question;
</script>

<div class="main">
	<h3>Explanation ({question.id}/9)</h3>

	{#if question.q.type === QuestionType.MultipleChoice}
		<div class="mc">
			{#each question.q.options as opt, idx}
				<div
					class={`mc-item ${question.q.selected === idx ? 'selected' : ''} ${
						question.q.correct === idx ? 'correct' : ''
					}`}
				>
					<input id={String(idx)} type="radio" disabled checked={question.q.selected === idx} />
					<label for={String(idx)}>{opt}</label><br />
					<p>{question.q.explanations[idx]}</p>
				</div>
			{/each}
		</div>
	{:else}
		<p>interactive</p>
	{/if}
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		align-items: center;

		left: calc(100% / 2 - 314px);

		position: absolute;
		background-color: rgb(59, 56, 56);
		width: 625px;
		height: 515px;

		border: 2px solid darkgrey;
		border-radius: 2rem;
	}

	.mc {
		width: 100%;
	}

	.mc-item > input {
		accent-color: #3475f7;
		width: 20px;
		height: 20px;
		margin-left: 2em;
		position: relative;
		top: 5px;
	}

	.mc-item {
		border-radius: 1rem;
		margin-left: 2em;
		margin-right: 20px;
		padding-bottom: 5px;
		margin-bottom: 7px;
	}

	.mc-item > label {
        position: relative;
        top: 2px;
		font-weight: bold;
	}

	.mc-item > input,
	p {
		margin-left: 2em;
	}

	.mc-item > p {
		color: rgb(217, 217, 217);
	}

	.mc-item.selected {
		background-color: rgba(255, 9, 9, 0.3);
	}

	.mc-item.correct {
		background-color: rgba(44, 255, 9, 0.3);
	}
</style>
