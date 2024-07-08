<script>
	import '../app.css';
	// All themes
	import 'carbon-components-svelte/css/all.css';
</script>

<slot></slot>

<style>

	/* Fixing ghost button color */

	:global(.bx--btn--ghost.bx--btn--icon-only .bx--btn__icon path:not([data-icon-path]):not([fill=none])){
		fill: var(--cds-link-01, #0f62fe) !important;
	}

	:global(.bx--btn--ghost:hover.bx--btn--icon-only .bx--btn__icon path:not([data-icon-path]):not([fill=none])){
		fill:var(--cds-hover-primary-text, #0043ce) !important;
	}

	:global(.icon) {
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
	}
</style>
