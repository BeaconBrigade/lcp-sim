<script lang="ts">
	import { increaseAndCompound, QuestionType, type Question } from '$lib/question';
	import { invoke } from '@tauri-apps/api/tauri';

	export let question: Question;
	export let show: boolean;
	export let changes: number[];
	export let selected: number | null;
	export let lastChange: [string, number] | null;
	let interactiveMsg: string;
	let interactiveCorrect: boolean;
	let changedNothing = true;

	$: compounds = question.equation.split(' ').filter((x) => x !== '+' && x !== '↔');

	async function updateMsg() {
		if (question.q.type !== QuestionType.Interactive) {
			return;
		}
		const [increase, compound] = increaseAndCompound(changes, question.defaults, compounds);
		changedNothing = !increase || !compound;
		if (changedNothing || lastChange === null || lastChange === undefined) {
			interactiveMsg = 'You made no changes, so the system will not adjust.';
			return;
		}

		// no change has been made
		if (lastChange[0] === '') {
			return;
		}
		let testConcentrations: number[];
		try {
			testConcentrations = await invoke('test_adjustment', {
				idx: question.id - 1,
				adjust: { Concentration: lastChange }
			});
		} catch (e) {
			console.error(e);
			return;
		}
		interactiveCorrect = question.q.isRight(testConcentrations);

		interactiveMsg = `You ${increase}d ${lastChange[0]}. ${
			interactiveCorrect ? question.q.correctMsg : question.q.incorrectMsg
		}`;
	}

	$: {
		show;
		updateMsg();
	}
</script>

<div class="main" class:show>
	<h3>Explanation ({question.id}/5)</h3>

	{#if question.q.type === QuestionType.MultipleChoice}
		<div class="mc">
			{#each question.q.options.filter((s) => s !== '') as opt, idx}
				<div
					class="mc-item"
					class:selected={selected === idx}
					class:correct={question.q.correct === idx}
				>
					<input id={String(idx)} type="radio" disabled checked={selected === idx} />
					<label for={String(idx)}>{opt}</label><br />
					<p>{@html question.q.explanations[idx]}</p>
				</div>
			{/each}
		</div>
	{:else}
		<p
			class="interactive"
			class:correct={interactiveCorrect}
			class:incorrect={!interactiveCorrect && !changedNothing}
		>
			{interactiveMsg}
		</p>
	{/if}
</div>

<style>
	.main {
		flex-direction: column;
		align-items: center;

		left: calc(100% / 2 - 314px);
		top: 120px;

		position: absolute;
		background-color: rgb(63, 63, 63);
		width: 625px;
		height: 475px;

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
		margin-right: 10px;
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

	.interactive.correct {
		background-color: rgba(44, 255, 9, 0.3);
	}

	.interactive.incorrect {
		background-color: rgba(255, 9, 9, 0.3);
	}

	p.interactive {
		border-radius: 1rem;
		margin-left: 2em;
		margin-right: 20px;
		padding: 20px;
	}
</style>
