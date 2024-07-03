<script>
	import Frame from '$lib/components/Frame.svelte';
	import { Tile, Checkbox } from 'carbon-components-svelte';
	import { writable } from 'svelte/store';

	let checked_values = writable([]);
</script>

<Frame direction="column">
	<Tile light>
		<h1>Welcome to SvelteKit</h1>
		<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
	</Tile>
	<div class="tile-space" />
	<Tile light>
		<p>Settings go here</p>
		<Frame direction="row">
			<Checkbox bind:group={$checked_values} labelText="Special symbols" value="special" />
			<Checkbox bind:group={$checked_values} labelText="Numbers" value="numbers" />
			<Checkbox bind:group={$checked_values} labelText="Lowercase" value="lowercase" />
			<Checkbox bind:group={$checked_values} labelText="Uppercase" value="uppercase" />
		</Frame>
        <Frame>
            <p style="margin-right:var(--cds-spacing-03)">Checked values: </p>
            {#each $checked_values as value}
                <p style="margin-right:var(--cds-spacing-05)">{value}</p>
            {/each}
        </Frame>
	</Tile>
</Frame>

<style>
	.tile-space {
		margin-bottom: 32px;
	}
</style>
