<script lang="ts">
    export let display: string|null = null;
    export let direction: string ='row';

    export let fixed_width:string|null = null;
    export let fixed_height:string|null = null;
</script>

<div class="frame" style='flex-direction:{direction};display:{display};
    width:{fixed_width};
    max-width:{fixed_width};
    min-width:{fixed_width};
    height:{fixed_height};
    max-height:{fixed_height};
    min-height:{fixed_height};'>
<slot/>
</div>

<style>
    .frame {
        display: flex;
        width: 100%;
        height: 100%;

        overflow: auto;
    }
</style>