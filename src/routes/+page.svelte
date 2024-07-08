<script lang="ts">
	import Frame from '$lib/components/Frame.svelte';
	import {
		Tile,
		Checkbox,
		Button,
		RadioButtonGroup,
		RadioButton,
		Slider,
		Tag,
		InlineNotification
	} from 'carbon-components-svelte';
	import { writable, derived } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/tauri';
	import { writeText } from '@tauri-apps/api/clipboard';

	import CopyButton from '$lib/components/CopyButton.svelte';
	import Row from '$lib/components/Row.svelte';
	import Column from '$lib/components/Column.svelte';

	const checked_values_order = ['special', 'numbers', 'lowercase', 'uppercase'];

	let checked_values = writable<string[]>([]);
	let checked_values_ordered = derived(checked_values, ($checked_values) =>
		[...$checked_values].sort(
			(a, b) => checked_values_order.indexOf(a) - checked_values_order.indexOf(b)
		)
	);

	let password_length = writable<number>(8);

	let passwords = writable<string[]>([]);

	let is_generation_allowed_charset = derived(checked_values, ($checked_values) => {
		return $checked_values.length > 0;
	});

	let is_generation_allowed_length = derived(
		[password_length, checked_values],
		([$password_length, $checked_values]) => {
			return $password_length > $checked_values.length - 1;
		}
	);

	let is_generation_allowed = derived(
		[is_generation_allowed_charset, is_generation_allowed_length],
		([$is_generation_allowed_charset, $is_generation_allowed_length]) =>
			$is_generation_allowed_charset && $is_generation_allowed_length
	);

	// Observe any changes to reset password array
	$: {
		$checked_values, $password_length;
		passwords.set([]);
	}

	$: {
		if($password_length < $checked_values.length) {
			$password_length = $checked_values.length
		}
	}
	$checked_values = ['lowercase', 'uppercase'];

	async function generate_passwords() {
		let has_symbols = $checked_values.includes('special');
		let has_numbers = $checked_values.includes('numbers');
		let has_lowercase = $checked_values.includes('lowercase');
		let has_uppercase = $checked_values.includes('uppercase');

		passwords.set([]);
		for (let i = 0; i < 5; i++) {
			let password = await invoke<string>('generate_password', {
				length: $password_length,
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

<Column>
	<!-- <div class="tile-space" /> -->
	<Tile light>
		<p>Settings go here</p>
		<Row>
			<RadioButtonGroup
				legendText="Predefined password lengths"
				name="length"
				bind:selected={$password_length}
			>
				<RadioButton labelText="5" value={5} />
				<RadioButton labelText="8" value={8} />
				<RadioButton labelText="13" value={13} />
				<RadioButton labelText="21" value={21} />
			</RadioButtonGroup>
			<Slider
				labelText="Custom password length"
				min={$checked_values.length}
				max={55}
				value={$password_length}
				on:change={(event) => {
					let value = event.detail;
					// console.log(value);
					$password_length = value;
				}}
			/>
		</Row>
	</Tile>
	<Tile>
		<Row>
			<Checkbox bind:group={$checked_values} labelText="Special symbols" value="special" />
			<Checkbox bind:group={$checked_values} labelText="Numbers" value="numbers" />
			<Checkbox bind:group={$checked_values} labelText="Lowercase" value="lowercase" checked />
			<Checkbox bind:group={$checked_values} labelText="Uppercase" value="uppercase" checked />
		</Row>
		<Row>
			<p class="tags-label">Checked values:</p>
			{#each $checked_values_ordered as value}
				<Tag type={get_tag_color(value)}>{value}</Tag>
			{/each}
		</Row>
	</Tile>

	{#if !$is_generation_allowed}
		<Tile light>
			{#if !$is_generation_allowed_length}
				<InlineNotification
					hideCloseButton
					kind="warning"
					title="Not enough length:"
					subtitle="Please select a length higher than checked values"
				/>
			{/if}
			{#if !$is_generation_allowed_charset}
				<InlineNotification
					hideCloseButton
					kind="warning"
					title="No characters selected:"
					subtitle="Please select at least one character option"
				/>
			{/if}
		</Tile>
	{/if}

	<Tile light>
		<Button disabled={!$is_generation_allowed} on:click={generate_passwords}
			>Generate password</Button
		>
	</Tile>

	{#if $passwords.length > 0}
		<Tile light>
			<p>Results go here</p>
			<Column>
				{#if $passwords.length === 0}
					<p>No passwords generated</p>
				{:else}
					{#each $passwords as password}
						<div class="horizontal">
							<CopyButton valueToCopy={password} />
							<p class="password-result">{password}</p>
						</div>
					{/each}
				{/if}
			</Column>
		</Tile>
	{/if}
</Column>

<style>
	.password-result {
		font-family: 'IBM Plex Mono', monospace;
	}

	.horizontal {
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.tags-label {
		margin-right: var(--cds-spacing-03);
		margin-top: var(--cds-spacing-02);
		margin-bottom: var(--cds-spacing-02);
	}
</style>
