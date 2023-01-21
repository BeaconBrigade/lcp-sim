<script lang="ts">
	import Padded from '$lib/Padded.svelte';

	let concentration: HTMLDivElement;
	let temperature: HTMLDivElement;
	let volume: HTMLDivElement;

	function select(toggle: string) {
		if (toggle === 'concentration') {
			concentration.classList.toggle('shown');
			temperature.classList.remove('shown');
			volume.classList.remove('shown');
		} else if (toggle === 'temperature') {
			temperature.classList.toggle('shown');
			volume.classList.remove('shown');
			concentration.classList.remove('shown');
		} else {
			volume.classList.toggle('shown');
			temperature.classList.remove('shown');
			concentration.classList.remove('shown');
		}
	}
</script>

<div class="main">
	<h1>Le Chatelier's Principle</h1>
	<div class="main-menu">
		<Padded href="/">Main Menu</Padded>
	</div>

	<p>
		Le Chatelier's principle predicts the effect of a change in the conditions of an equilibrium.
		According to the principle: when an equilibrium experiences a change in concentration,
		temperature or volume it will shift to create a new equilibrium which counteracts the initial
		change.
	</p>

	<p>
		In an equilibrium system, there is an equation which can react in both directions. When the
		system is at equilibrium the rate of forward reaction and rate of the reverse reaction are the
		same, or the concentrations of the different compounds remains mostly constant. When a change
		occurs, the system will modify its forward or reverse reaction rates to relieve the stress.
		After the system has adjusted the concentrations return to being constant again. See examples
		below:
	</p>

	<button class="open" on:click={() => select('concentration')}>Concentration</button>
	<div class="collapsible" bind:this={concentration}>
		<span class="equation">N2 + O2 ↔ N2O2</span>
		<p>
			The system will shift to oppose changes in concentration. So, in the above system, if N2 was
			added the eqauation's forward reaction rate would increase (N2 + O2 → N2O2). The system tries
			to react away the excess N2 consuming more O2 and producing more N2O2. If N2 were removed, the
			opposite would occur. The system would try to regain lost N2 and increase the rate of reverse
			reaction (N2O2 → N2 + O2). This will consume N2 and O2 producing more N2O2.
		</p>
	</div>

	<button class="open" on:click={() => select('temperature')}>Temperature</button>
	<div class="collapsible" bind:this={temperature}>
		<span class="equation">N2O4 + 57 kJ ↔ 2 NO2</span>
		<p>
			The system will shift to oppose changes in temperature. In the above system energy is a
			reactant, so when the forward reaction occurs, thermal energy is consumed to produce NO2.
			Changes in temperature work just like changes in concentration. If temperature is added, since
			temperature is a reactant a shift to the products will occur decreasing temperature, consuming
			some N2O4 and producing more NO2. The opposite will happen if temperature is decreased. If no
			energy is in the reaction or ∆H is 0 kJ, then changes in temperature will have no effect on an
			equilibrium.
		</p>
	</div>

	<button class="open" on:click={() => select('volume')}>Volume and Pressure</button>
	<div class="collapsible" bind:this={volume}>
		<span class="equation">3 H2<sub>(g)</sub> + N2<sub>(g)</sub> ↔ 2 NH3<sub>(g)</sub></span>
		<p>
			Volume and pressure are linked, an increase in pressure decreases volume and vice versa. When
			volume is decreased (or pressure increased) an equilibrium will shift to take up less space.
			We use the molar ratio to determine which way the equilibrium shifts. In the above example the
			molar ratio is 4 to 2 (the sum of the coefficients which are gaseous or aqueous). Therefore,
			when the volume is decreased the equilibrium will shift right to take less space, the side
			with 2 mol, instead of 4. If pressure were relieved the system would react back to the left to
			take up more space with the 4 mol of H2 and N2. It's important to remember that only gaseous
			or aqueous compounds count in the molar ratio, and if the ratio is 1:1, volume won't affect
			the equilibrium system.
		</p>
	</div>

	<!--This is to avoid unused styles-->
	<p class="shown" style="display: none;" />
	<p>
		Want more information? Visit <a
			class="source"
			href="https://en.wikipedia.org/wiki/Le_Chatelier%27s_principle"
			target="_blank"
			rel="noreferrer">Wikipedia</a
		>
	</p>
</div>

<style>
	.main {
		display: flex;
		flex-direction: column;
		align-items: center;
		margin-left: 15px;
		margin-right: 10px;
	}

	.main > p {
		font-size: 1rem;
	}

	.main-menu {
		position: absolute;
		right: 10px;
		top: 30px;
	}

	.source {
		color: lightgray;
		text-decoration: none;
		font-weight: bold;
	}

	.open {
		background-color: #313131;
		color: white;
		border: none;
		padding: 10px;
		padding-left: 2rem;
		width: 100%;
		font-size: 1rem;
		overflow: hidden;
		text-align: left;
		margin-bottom: 10px;
		border-radius: 5px;
		border-width: 1px solid #2f2f2f;
	}

	.open:hover {
		cursor: pointer;
		background-color: #343434;
	}

	.collapsible {
		max-height: 0px;
		transition: max-height 1s ease-in-out;
		display: none;
	}

	.shown {
		display: flex;
		flex-direction: column;

		max-height: 15rem;
		padding: 10px;
		margin-bottom: 10px;
		width: 95vw;
		border-radius: 1rem;
		border-width: 1px solid #3f3f3f;

		background-color: #414141;
		font-size: 0.9rem;
	}

	.collapsible > p {
		font-size: 1rem;
		padding-left: 10px;
	}

	.equation {
		text-align: center;
		background-color: #7f7f7f;
		padding: 10px;
		font-weight: bold;
		width: max-content;
		margin: 0px auto;

		border: 2px solid #525151;
		border-radius: 0.75rem;
	}
</style>
