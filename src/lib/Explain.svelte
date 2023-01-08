<script lang="ts">
	import { QuestionType, type Question } from '$lib/question';

	export let question: Question;
	export let show: boolean;
</script>

<div class="main" class:show>
	<h3>Explanation ({question.id}/9)</h3>

	{#if question.q.type === QuestionType.MultipleChoice}
		<div class="mc">
			{#each question.q.options as opt, idx}
				<div
					class="mc-item"
					class:selected={question.q.selected === idx}
					class:correct={question.q.correct === idx}
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
		flex-direction: column;
		align-items: center;

		left: calc(100% / 2 - 314px);

		position: absolute;
		background-color: rgb(63, 63, 63);
		width: 625px;
		height: 515px;

		border: 2px solid darkgrey;
		border-radius: 2rem;

		/* hide by default */
		transition: opacity 0.2s ease-in-out;
		opacity: 0;
		display: none;
	}

	.main.show {
		opacity: 1;
		display: flex;
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
