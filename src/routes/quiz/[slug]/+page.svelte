<script lang="ts">
	import Padded from '$lib/Padded.svelte';
	import { questions } from '$lib/question';
	import DiagramQuestion from '$lib/DiagramQuestion.svelte';
	import Question from '$lib/Question.svelte';

	export let data: { id: number };

	$: question = questions[data.id - 1];
</script>

<div class="main">
	{#if question.id === 5}
		<DiagramQuestion />
	{:else}
		<div class="home">
			<Padded href="/">Main Menu</Padded>
		</div>
		<span class="floating">{question.id}/5</span>

		<span class="equation">{question.equation}</span>
		<p>{question.prompt}</p>

		<Question {question} />
	{/if}
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
</style>
