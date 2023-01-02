<script lang="ts">
	import { page } from '$app/stores';
	import Padded from '$lib/Padded.svelte';

	let message: string = ($page.error || { message: 'no description' }).message;
</script>

<div class="main">
	<h1>An Error Occured {$page.status}</h1>
	<p>{message}</p>
	<Padded href="/">Main Menu</Padded>
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
