<script>
	import Padded from '$lib/Padded.svelte';
</script>

<div class="main">
	<h1>Le Chatelier's Principle</h1>
	<div class="main-menu">
		<Padded href="/">Main Menu</Padded>
	</div>
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.main-menu {
		position: absolute;
		right: 10px;
		top: 30px;
	}
</style>
