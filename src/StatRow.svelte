<script lang="ts">
    export let name : String;
    export let values : [number,number,number];
    export let unit : String = "kt";
    export let padThreeZeroes : boolean = true;

   const zeroPad = (num, places) => String(num).padStart(places, '0')
</script>
{#if unit == "kt"}
<tr>
    <td>{name}</td>
    <td class="data">{values[0].toFixed(1)}</td>
    <td class="unit">kt</td>
    <td class="data">{values[1].toFixed(1)}</td>
    <td class="unit">kt</td>
    <td class="data">{values[2].toFixed(1)}</td>
    <td class="unit">kt</td>
</tr>
{:else if unit == "&deg;" && padThreeZeroes}
<tr>
    <td>{name}</td>
    <td class="data">{zeroPad(Math.floor(values[0]),3)}</td>
    <td class="unit">&deg;</td>
    <td class="data">{zeroPad(Math.floor(values[1]),3)}</td>
    <td class="unit">&deg;</td>
    <td class="data">{zeroPad(Math.floor(values[2]),3)}</td>
    <td class="unit">&deg;</td>
</tr>
{:else}
<tr>
    <td>{name}</td>
    <td class="data">{Math.floor(values[0])}</td>
    <td class="unit">&deg;</td>
    <td class="data">{Math.floor(values[1])}</td>
    <td class="unit">&deg;</td>
    <td class="data">{Math.floor(values[2])}</td>
    <td class="unit">&deg;</td>
</tr>
{/if}
<style>
    td {
        text-align: left;
    }
    .data{
        text-align: right;
    }
    .unit{
        text-align: left;
    }
</style>