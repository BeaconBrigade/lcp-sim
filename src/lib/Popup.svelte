<script lang="ts">
	export let show: boolean;
	export let checked: boolean;
</script>

<div class="main" class:fade={show} class:correct={checked}>
	<p>{checked ? '✓' : '✘'}</p>
</div>

<style>
	.main {
		position: absolute;
		top: calc(100% / 2 - 80px);
		left: calc(100% / 2 - 50px);

		width: 100px;
		height: 100px;

		border: 2px solid darkgrey;
		border-radius: 2rem;

		background-color: rgba(255, 9, 9, 0.3);
		border: 2px solid rgb(255, 25, 25);

		opacity: 0;
	}

	.correct > p {
		left: 15px;
	}

	.correct {
		background-color: rgba(44, 255, 9, 0.3);
		border: 2px solid rgb(55, 255, 0);
	}

	p {
		font-size: 90px;
		position: relative;
		left: 20px;
		bottom: 90px;
	}

	.fade {
		animation: fade-in-out 1s ease-in-out;
	}

	@keyframes fade-in-out {
		0%,
		100% {
			opacity: 0;
		}
		25% {
			opacity: 1;
		}
		75% {
			opacity: 1;
		}
	}
</style>
