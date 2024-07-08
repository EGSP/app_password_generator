<script lang="ts">
	import InlineButton from "./InlineButton.svelte";
    import CarbonCopy from '$lib/icons/CarbonCopy.svelte';
	import { writeText } from "@tauri-apps/api/clipboard";

    export let feedbackTimeout: number = 2000
    export let valueToCopy: string;

    let iconDescription = 'Copy'

    function handleClick() {
        writeText(valueToCopy);
        
        let temporary_iconDescription = iconDescription.slice()
        iconDescription = 'Copied!'
        setTimeout(() => {
            iconDescription = temporary_iconDescription
        }, feedbackTimeout)
    }
</script>




<InlineButton icon={CarbonCopy} iconDescription={iconDescription} 
on:click={handleClick}/>