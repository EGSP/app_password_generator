<script lang="ts">
	import { getContext, SvelteComponent } from 'svelte';

	export let isSelected: boolean = false;

	export let icon: SvelteComponent<any> | any;

	export let iconDescription: string;

	// export let tooltipAlignment: 'start' | 'center' | 'end' = 'center';

	// export let tooltipPosition: 'top' | 'right' | 'bottom' | 'left' = 'bottom';

	$: iconProps = {
		'aria-hidden': true,
		'aria-label': iconDescription
	};

	$: buttonProps = {
		'aria-pressed': isSelected,
		...$$restProps,
		class: [isSelected && 'btn-inline.selected'].filter(Boolean).join(' ')
	};

	/** Obtain a reference to the HTML element */
	export let ref: HTMLButtonElement | null = null;
	const ctx = getContext('ComposedModal') as { declareRef: (ref: any) => void };
	$: if (ctx && ref) {
		ctx.declareRef(ref);
	}
</script>

<button
	bind:this={ref}
	{...buttonProps}
	on:click
	on:focus
	on:blur
	on:mouseover
	on:mouseenter
	on:mouseleave
	class="btn-inline tooltip-ally"
>
	<span class="btn-inline-tooltip">{iconDescription}</span>
	<!-- svelte-ignore empty-block -->
	{#if $$slots.icon}
		<slot name="icon" {...iconProps} class="icon" />
	{:else if icon}
		<svelte:component this={icon} {...iconProps} class="icon" />
	{/if}
</button>

<style>
	.btn-inline {
		position: relative;
		display: inline-flex;
		overflow: visible;
		align-items: center;
		cursor: pointer;

		border-width: 1px;
		border-style: solid;
		border-color: rgba(0, 0, 0, 0);
		background-color: rgba(0, 0, 0, 0);
		color: var(--cds-link-01, #0f62fe);
		/* padding: calc(0.875rem - 3px) 16px; */
		padding: 4px 4px;

		font-size: var(--cds-body-short-01-font-size, 0.875rem);
		font-weight: var(--cds-body-short-01-font-weight, 400);
		line-height: var(--cds-body-short-01-line-height, 1.28572);
		letter-spacing: var(--cds-body-short-01-letter-spacing, 0.16px);

		transition:
			background 70ms cubic-bezier(0, 0, 0.38, 0.9),
			box-shadow 70ms cubic-bezier(0, 0, 0.38, 0.9),
			border-color 70ms cubic-bezier(0, 0, 0.38, 0.9),
			outline 70ms cubic-bezier(0, 0, 0.38, 0.9);
	}

	.btn-inline:hover {
		color: var(--cds-hover-primary-text, #0043ce);
		background-color: var(--cds-hover-ui, #e5e5e5);
	}

	.btn-inline:focus {
		outline: 2px solid var(--cds-focus, #0f62fe);
		outline-offset: -2px;
		outline-color: var(--cds-focus, #0f62fe);
	}

	.btn-inline:active {
		background-color: var(--cds-layer-active, #c6c6c6);
	}

	.btn-inline.tooltip-ally::before {
		display: flex;
		position: absolute;
		z-index: 6000;

		/* animation: tooltip-fade 70ms cubic-bezier(0.2, 0, 0.38, 0.9); */
		opacity: 0;

		border-style: solid;
		border-width: 0 0.25rem 0.3125rem 0.25rem;
		border-color: rgba(0, 0, 0, 0) rgba(0, 0, 0, 0) #4c4848 rgba(0, 0, 0, 0);
		transform: translate(-50%, 100%);

		width: 0;
		height: 0;
		left: 50%;
		/* bottom: -0.5rem; */
		bottom: -0.55rem;

		align-items: center;
		pointer-events: none;

		content: '';
	}

	.btn-inline.tooltip-ally:hover::before {
		opacity: 1;
	}

	.btn-inline .btn-inline-tooltip {
		clip: rect(0 0 0 0);

		bottom: -2.4rem;
		-webkit-transform: translate(-50%, 100%);
		transform: translate(-50%, 100%);

		left: 50%;

		/* pointer-events: all; */

		opacity: 0;

		box-shadow: 0 2px 6px var(--cds-shadow, rgba(0, 0, 0, 0.3));
		z-index: 6000;
		width: -webkit-max-content;
		width: -moz-max-content;
		width: max-content;
		min-width: 1.5rem;
		max-width: 13rem;
		height: auto;
		padding: 0.1875rem 1rem;
		background-color: var(--cds-inverse-02, #393939);
		border-radius: 0.125rem;
		color: var(--cds-inverse-01, #ffffff);
		font-weight: 400;
		text-align: left;
		-webkit-transform: translateX(-50%);
		transform: translateX(-50%);
		font-size: var(--cds-body-short-01-font-size, 0.875rem);
		font-weight: var(--cds-body-short-01-font-weight, 400);
		line-height: var(--cds-body-short-01-line-height, 1.28572);
		letter-spacing: var(--cds-body-short-01-letter-spacing, 0.16px);

		box-sizing: content-box;
		white-space: normal;
		word-break: break-word;

		position: absolute;
		display: flex;
		align-items: center;
	}
	.btn-inline:hover .btn-inline-tooltip {
		clip: auto;
		animation: tooltip-fade 70ms cubic-bezier(0.2, 0, 0.38, 0.9);
		opacity: 1;
	}

	.btn-inline-tooltip::after {
		content: '';
		position: absolute;
		display: block;
		content: '';
		left: 0;
		width: 100%;
		height: 0.90rem;
		top: -0.90rem;
		/* pointer-events: all; */
		pointer-events: none;

	}

	@keyframes tooltip-fade {
		0% {
			opacity: 0;
		}
		100% {
			opacity: 1;
		}
	}
</style>
