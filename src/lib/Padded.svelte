<script>
	export let href = '/';
</script>

<a class="padded" {href}><slot /></a>

<style>
	.padded {
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

	.padded:hover {
		background-color: #8c8c8c;
	}
</style>
