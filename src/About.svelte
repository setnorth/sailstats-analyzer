<script lang="ts">
    import { fly } from 'svelte/transition';
    import Attribution from './Attribution.svelte';
    import License from './License.svelte';

    export let aboutShow : boolean = false;
</script>
{#if aboutShow}
    <div class="info"
        in:fly="{{ x: -200, duration: 200 }}"
		out:fly="{{ x: -200, duration: 200 }}">
	    <span class="closeBtn" on:click={() => aboutShow = false}>&times;</span>
        <p class="programTitle">SailStats Viewer v0.1.0</p>
        <p>Copyright &copy; 2021 Thorsten Ernst Schilling &lt;<a href="thorsten.schilling@gmail.com" target="_blank">thorsten.schilling@gmail.com</a>&gt;</p>
        <p>Built with Svelte, Rust &amp; Tauri in the hope that it is useful to others.</p>
        <p>For more information see: </p>
        <p><b>License:</b>
            <pre class="license">
                <License/>
            </pre>
        <p>
            <b>Third party attribution &amp; licenses:</b>
            <pre class="attribution">
                <Attribution/>
            </pre>
    </div>
{/if}

<style>
    a:link, a:visited, a:hover, a:active{
        color: white;
        text-decoration: underline;
    }
    .license{
        width: 100%;
        height: 256px;
        overflow-y: scroll;
        overflow-x: scroll;
        background-color: white;
        color: black;
        tab-size: 4;
    }
    .attribution{
        width: 100%;
        height: 256px;
        overflow-y: scroll;
        overflow-x: scroll;
        background-color: white;
        color: black;
    }
    .info{
		min-height: 100%;
        max-width: 650px;
		position: fixed;
		top: 0;
		left: 0;
		z-index: 100;
		background-color: rgba(1, 79, 134, 1);
		color: white;
		text-align: left;
		padding: 1em;
		padding-top: 2em;
	}
    .info .closeBtn{
		position: absolute;
		top: 0;
		right: 10px;
		font-size: 36px;
		cursor: pointer;
	}
    .programTitle{
        font-size: +2em;
    }
</style>