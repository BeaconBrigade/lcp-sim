<script lang="ts">
	import Padded from '$lib/Padded.svelte';
	import { questions } from '$lib/question';
	import Question from '$lib/Question.svelte';

	export let data;
	let id = data.id;

	let question = questions[id - 1];
	let back = id > 1 ? `/quiz/${id - 1}` : '/quiz';
	let next = `/quiz/${Number(id) + 1}`;

	console.log(`ran... id: ${id}`);
</script>

<div class="main">
	<div class="home">
		<Padded href="/">Main Menu</Padded>
	</div>
	<span class="floating">{question.id}/9</span>

	<span class="equation">{question.equation}</span>
	<p>{question.prompt}</p>

	<Question {question} />

	<!-- if we aren't on the first question go back to the quiz -->
	<a class="back" on:click={() => (location.href = back)} href={back}>Back</a>

	<!-- If we're at the end have the button say done -->
	{#if id < 9}
		<!--
            I don't know why I need to manually set the href... but it doesn't
            work without it
        -->
		<a class="next" on:click={() => (location.href = next)} href={next}>Next</a>
	{:else}
		<a class="next finish" href="/quiz">Finish</a>
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
		bottom: 0px;

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

	.finish {
		background-color: #70ad47;
	}

	.finish:hover {
		background-color: #88c95d;
	}
</style>
