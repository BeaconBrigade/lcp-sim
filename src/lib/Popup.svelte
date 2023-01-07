<script lang="ts">
	export let checked: boolean;
	export let show: boolean;

	let mark = checked ? '✓' : '✘';
</script>

<div class={`main ${show ? 'fade' : ''}`} id={String(checked)}>
	<p>{mark}</p>
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

		opacity: 0;
	}

	#true {
		background-color: rgba(44, 255, 9, 0.3);
		border: 2px solid rgb(55, 255, 0);
	}

	#false {
		background-color: rgba(255, 9, 9, 0.3);
		border: 2px solid rgb(255, 25, 25);
	}

	#false > p {
		left: 20px;
	}

	p {
		font-size: 90px;
		position: relative;
		left: 15px;
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
