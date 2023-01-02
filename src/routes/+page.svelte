<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Padded from '$lib/Padded.svelte';

	function quit() {
		invoke('quit', { code: 0 });
	}

	onMount(() => {
		invoke('close_splashscreen');
	});
</script>

<div class="main">
	<h1>Le Chatelier's Principle</h1>
	<p>Learn and test your knowledge of Le Chatelier's Principle</p>
	<Padded href="/learn-more">Learn More</Padded>

	<button on:click={quit} class="quit">Quit</button>
	<img id="beaker" alt="chemistry beaker" src="/chemistry-beaker.svg" />

	<a href="/quiz" class="start">Start Quiz</a>
	<a href="/sandbox" class="sandbox">Sandbox Mode</a>
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		align-items: center;
	}

	#beaker {
		width: 200px;
		height: 200px;
	}

	.start {
		position: absolute;
		bottom: 100px;
		left: 175px;

		margin: 10px;
		padding: 10px;
		padding-left: 40px;
		padding-right: 40px;

		color: white;
		background-color: #70ad47;
		font-size: 1.25em;
		text-decoration: none;
		font-weight: bold;

		border: 2px solid #525151;
		border-radius: 0.75rem;
	}

	.sandbox {
		position: absolute;
		bottom: 100px;
		right: 125px;

		margin: 10px;
		padding: 10px;
		padding-left: 40px;
		padding-right: 40px;

		color: white;
		background-color: #4472c4;
		font-size: 1.25em;
		text-decoration: none;
		font-weight: bold;

		border: 2px solid #525151;
		border-radius: 0.75rem;
	}

	.quit {
		position: absolute;
		right: 10px;
		top: 10px;

		margin: 10px;
		padding: 8px;
		padding-left: 40px;
		padding-right: 40px;

		color: white;
		background-color: #7f7f7f;
		font-size: 0.9em;
		font-weight: bold;

		border: 2px solid #525151;
		border-radius: 0.75rem;
	}

	.quit:hover {
		background-color: #8c8c8c;
		cursor: pointer;
	}

	.quit:active {
		background-color: #a1a1a1;
	}
</style>
