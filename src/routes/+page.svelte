<script lang="ts">
	import Frame from '$lib/components/Frame.svelte';
	import InlineButton from '$lib/components/InlineButton.svelte';
	import {
		Tile,
		Checkbox,
		Button,
		RadioButtonGroup,
		RadioButton,
		Slider,
		Tag,
		CopyButton
	} from 'carbon-components-svelte';
	import { writable, derived } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/tauri';
	import { writeText } from '@tauri-apps/api/clipboard';

	import CarbonCopy from '$lib/icons/CarbonCopy.svelte';
	import LucideClipboard from '$lib/icons/LucideClipboard.svelte';
	import MaterialSymbolsContentCopyOutlineSharp from '$lib/icons/MaterialSymbolsContentCopyOutlineSharp.svelte';
	import TdesignCopy from '$lib/icons/TdesignCopy.svelte';
	import FlowbiteClipboardListOutline from '$lib/icons/FlowbiteClipboardListOutline.svelte';
	const checked_values_order = ['special', 'numbers', 'lowercase', 'uppercase'];

	let checked_values = writable<string[]>([]);
	let checked_values_ordered = derived(checked_values, ($checked_values) =>
		[...$checked_values].sort(
			(a, b) => checked_values_order.indexOf(a) - checked_values_order.indexOf(b)
		)
	);
	let passwords = writable<string[]>([]);

	let password_length: number = 8;

	$checked_values = ['lowercase', 'uppercase'];

	async function generate_passwords() {
		let has_symbols = $checked_values.includes('special');
		let has_numbers = $checked_values.includes('numbers');
		let has_lowercase = $checked_values.includes('lowercase');
		let has_uppercase = $checked_values.includes('uppercase');

		passwords.set([]);
		for (let i = 0; i < 10; i++) {
			let password = await invoke<string>('generate_password', {
				length: password_length,
				symbols: has_symbols,
				numbers: has_numbers,
				uppercase: has_uppercase,
				lowercase: has_lowercase
			});

			$passwords = [...$passwords, password];
			console.log(password);
		}

		console.log($passwords.length);
	}

	function get_tag_color(value: string): 'red' | 'blue' | 'gray' | 'warm-gray' | undefined {
		if (value === 'special') {
			return 'red';
		} else if (value === 'numbers') {
			return 'blue';
		} else if (value === 'lowercase') {
			return 'warm-gray';
		} else if (value === 'uppercase') {
			return 'gray';
		}

		return undefined;
	}

	function copy_password_result(value: string) {
		console.log('Copying ' + value);
		// copy with tauri
		writeText(value);

		// copy with browser
		navigator.clipboard
			.writeText(value)
			.then(() => console.log('Copied!', value))
			.catch((err) => console.error('Error', err));
	}
</script>

<Frame direction="column">
	<Tile light>
		<Button on:click={generate_passwords}>Generate password</Button>
	</Tile>
	<!-- <div class="tile-space" /> -->
	<Tile light>
		<p>Settings go here</p>
		<Frame direction="column">
			<RadioButtonGroup
				legendText="Predefined password lengths"
				name="length"
				bind:selected={password_length}
			>
				<RadioButton labelText="5" value={5} />
				<RadioButton labelText="8" value={8} />
				<RadioButton labelText="13" value={13} />
				<RadioButton labelText="21" value={21} />
			</RadioButtonGroup>
			<Slider
				labelText="Custom password length"
				min={2}
				max={55}
				value={password_length}
				on:change={(event) => {
					let value = event.detail;
					// console.log(value);
					password_length = value;
				}}
			/>
		</Frame>

		<Frame direction="row">
			<Checkbox bind:group={$checked_values} labelText="Special symbols" value="special" />
			<Checkbox bind:group={$checked_values} labelText="Numbers" value="numbers" />
			<Checkbox bind:group={$checked_values} labelText="Lowercase" value="lowercase" checked />
			<Checkbox bind:group={$checked_values} labelText="Uppercase" value="uppercase" checked />
		</Frame>

		<Frame>
			<p style="margin-right:var(--cds-spacing-03)">Checked values:</p>
			{#each $checked_values_ordered as value}
				<Tag type={get_tag_color(value)}>{value}</Tag>
			{/each}
		</Frame>
	</Tile>

	<Tile light>
		<InlineButton icon={TdesignCopy} iconDescription="TDesignCopy" tooltipPosition="bottom" tooltipAlignment="center"/>
		<Button kind="ghost" iconDescription="TDesignCopy" icon={TdesignCopy}/>
		<Button kind="ghost" iconDescription="CarbonCopy" icon={CarbonCopy}/>
		<Button kind="ghost" iconDescription="LucideClipboard" icon={LucideClipboard}/>
		<Button kind="ghost" iconDescription="MaterialSymbolsContentCopyOutlineSharp" icon={MaterialSymbolsContentCopyOutlineSharp}/>
		<Button kind="ghost" iconDescription="FlowbiteClipboardListOutline" icon={FlowbiteClipboardListOutline}/>
	</Tile>

	<Tile light>
		<p>Results go here</p>
		{#if $passwords.length === 0}
			<p>No passwords generated</p>
		{:else}
			{#each $passwords as password}
				<div class="horizontal">
					<CopyButton text="" on:click={() => copy_password_result(password)} />
					<Button kind="ghost" iconDescription="Copy ${password}" icon={TdesignCopy} on:click={() => copy_password_result(password)} />
					<p class="password-result">{password}</p>
				</div>
			{/each}
		{/if}
	</Tile>
</Frame>

<style>
	.password-result {
		font-family: 'IBM Plex Mono', monospace;
	}

	.horizontal {
		display: flex;
		flex-direction: row;
		align-items: center;
	}
</style>
