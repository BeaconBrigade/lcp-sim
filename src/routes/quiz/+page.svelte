<script>
	import Padded from '$lib/Padded.svelte';
</script>

<div class="main">
	<h1>Quiz</h1>
	<div class="home">
		<Padded href="/">Main Menu</Padded>
	</div>

	<div class="question-grid">
		<a href="/quiz/1" class="question">1</a>
		<a href="/quiz/2" class="question">2</a>
		<a href="/quiz/3" class="question">3</a>
		<a href="/quiz/4" class="question">4</a>
		<a href="/quiz/5" class="question">5</a>
		<a href="/quiz/6" class="question">6</a>
		<a href="/quiz/7" class="question">7</a>
		<a href="/quiz/8" class="question">8</a>
		<a href="/quiz/9" class="question">9</a>
	</div>
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.home {
		position: absolute;
		right: 10px;
		top: 30px;
	}

	.question-grid {
		display: grid;
		grid-template-columns: auto auto auto;
		grid-template-rows: auto auto auto;
	}

	.question {
		min-width: 50px;
		min-height: 50px;
		padding: 20px;
		margin: 10px;
		border: 2px solid #525151;
		border-radius: 4rem;
		font-size: 3rem;
		text-align: center;
		background-color: #636363;
		text-decoration: none;
		color: white;
	}
</style>
