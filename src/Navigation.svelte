<script lang="ts">
    import { fly } from 'svelte/transition';
	import { sRunChartFields, sRadarChartFields } from './stores.js';
	import ChooseFile from "./ChooseFile.svelte";
	import About from "./About.svelte";

    let sideNavShow : boolean = false;
	let aboutShow : boolean = false;
	let lowPassFilter : boolean = false;

	//Update stores if "lowPassFilter" was toggled
	let lowPassFields = ["aws","awa","awd","tws","twa","twd","stw","roll","rudder_angle"]; //All fields which can be low passed
	$: if(lowPassFilter){
		for(let i = 0; i < lowPassFields.length; i++){
			const index = $sRunChartFields.indexOf(lowPassFields[i]);
			if(index > -1){
				$sRunChartFields[index] = $sRunChartFields[index]
											.replaceAll(lowPassFields[i],lowPassFields[i].concat("_lp"));
			}
		}
	}else{
		for(let i = 0; i< lowPassFields.length; i++){
			const index = $sRunChartFields.indexOf(lowPassFields[i].concat("_lp"));
			if(index > -1){
				$sRunChartFields[index] = $sRunChartFields[index]
											.replaceAll(lowPassFields[i].concat("_lp"),lowPassFields[i]);				
			}
		}
	}

</script>

{#if sideNavShow}
	<div class="sideNav"
		in:fly="{{ x: 200, duration: 200 }}"
		out:fly="{{ x: 200, duration: 200 }}">
		<span class="closeBtn" on:click={() => sideNavShow = false}>&times;</span>
		
		<div class="menuGroupTitle">Run Chart</div>
		<div>
			<label><input type="checkbox" bind:group={$sRunChartFields} value={lowPassFilter ? "tws_lp" : "tws"}/>True wind speed</label>
			<label><input type="checkbox" bind:group={$sRunChartFields} value={lowPassFilter ? "aws_lp" : "aws"}/>Apparent wind speed</label>
			<label><input type="checkbox" bind:group={$sRunChartFields} value={lowPassFilter ? "awa_lp" : "awa"}/>Apparent wind angle</label>
			<label><input type="checkbox" bind:group={$sRunChartFields} value="sog"/>Speed over ground</label>
			<label><input type="checkbox" bind:group={$sRunChartFields} value={lowPassFilter ? "stw_lp" : "stw"}/>Speed through water</label>
			<label><input type="checkbox" bind:group={$sRunChartFields} value={lowPassFilter ? "roll_lp" : "roll"}/>Angle of heel</label>
			<label><input type="checkbox" bind:group={$sRunChartFields} value={lowPassFilter ? "rudder_angle_lp" : "rudder_angle"}/>Rudder deflection</label>
		</div>

		<div class="menuGroupTitle">Radar Chart</div>
		<div>
			<label><input type="checkbox" bind:group={$sRadarChartFields} value={"avg,tws,twd"}/>Average TWS</label>
			<label><input type="checkbox" bind:group={$sRadarChartFields} value={"avg,aws,awa"}/>Average AWS</label>
			<label><input type="checkbox" bind:group={$sRadarChartFields} value={"avg,sog,cog"}/>Average SOG</label>
			<label><input type="checkbox" bind:group={$sRadarChartFields} value={"avg,stw,hdg"}/>Average STW</label>
			<label><input type="checkbox" bind:group={$sRadarChartFields} value={"dist,twa"}/>Ratio TWA</label>
			<label><input type="checkbox" bind:group={$sRadarChartFields} value={"dist,awa"}/>Ratio AWA</label>
			<label><input type="checkbox" bind:group={$sRadarChartFields} value={"dist,cog"}/>Ratio COG</label>
			<label><input type="checkbox" bind:group={$sRadarChartFields} value={"dist,hdg"}/>Ratio HDG</label>
		</div>

		<div class="menuGroupTitle">Options</div>
		<div>
			<label><input type="checkbox" bind:checked={lowPassFilter} />LPF Run Chart </label>
			<ChooseFile on:fileRead/>
		</div>

		<div class="infoText" on:click={() => aboutShow = !aboutShow}>About</div>
	</div>
{:else}
	<span class="menuBtn" on:click={() => sideNavShow = !sideNavShow}>
		<span>&#9776;</span>
	</span>
{/if}
<About bind:aboutShow/>

<style>
	.infoText{
		font-size: 0.8em;
		padding-top: 1em;
		cursor: pointer;
	}
	.menuBtn{
		top: 0;
		right: 0;
		position: absolute;
		z-index: 100;
		height: 30px;
		width: 30px;
		background-color: rgba(1, 79, 134, 1);
		color: white;
		font-size: 20px;
		cursor: pointer;
	}	
	.sideNav {
  		height: 100%;
  		width: 200px;
  		position: fixed;
  		z-index: 100;
  		top: 0;
  		right: 0;
  		background-color: rgba(1, 79, 134, 1);
		color: white;
		padding-top: 3em;
		text-align: left;
		padding-left: 0.5em;
	}
	.sideNav .closeBtn{
		position: absolute;
		top: 0;
		right: 10px;
		font-size: 36px;
		cursor: pointer;
	}
	.sideNav label input{
		margin-right: 0.5em;
	}
	.sideNav .menuGroupTitle{
		font-size: 20px;
		font-weight: bold;
		margin-bottom: 0.5em;
		margin-top: 0.5em;
	}
</style>