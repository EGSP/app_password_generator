<script lang="ts">
	import Frame from '$lib/components/Frame.svelte';
	import { Tile, Checkbox, Button } from 'carbon-components-svelte';
	import { writable } from 'svelte/store';
    import { invoke } from '@tauri-apps/api/tauri';

	let checked_values = writable<String[]>([]);

	$checked_values = ['lowercase', 'uppercase'];

    async function generate_password(){
        let has_symbols = $checked_values.includes('special');
		let has_numbers = $checked_values.includes('numbers');
		let has_lowercase = $checked_values.includes('lowercase');
		let has_uppercase = $checked_values.includes('uppercase');	
		
		let password = await invoke<String>('generate_password', {
			length: 16,
			symbols: has_symbols,
			numbers: has_numbers,
			uppercase: has_uppercase,
			lowercase: has_lowercase
		});
    }
</script>

<Frame direction="column">
	<Tile light>
		<Button on:click={generate_password}>Generate password</Button>
	</Tile>
	<!-- <div class="tile-space" /> -->
	<Tile light>
		<p>Settings go here</p>
		<Frame direction="row">
			<Checkbox bind:group={$checked_values} labelText="Special symbols" value="special" />
			<Checkbox bind:group={$checked_values} labelText="Numbers" value="numbers" />
			<Checkbox bind:group={$checked_values} labelText="Lowercase" value="lowercase" checked />
			<Checkbox bind:group={$checked_values} labelText="Uppercase" value="uppercase" checked />
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
