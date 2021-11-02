<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { onMount } from "svelte";

	import RangeSlider from "svelte-range-slider-pips";
	
	import { sFileHandle, sRadarChartFields, sRunChartFields, sStatsFields } from './stores.js';
	
	import ChooseFile from "./ChooseFile.svelte";
	import Navigation from "./Navigation.svelte";
	import SeaMap from "./SeaMap.svelte";
	import RunChart from "./RunChart.svelte";
	import RadarChart from "./RadarChart.svelte";
	import Stats from "./Stats.svelte";

	//Data
	let runChartData : Array<Array<{x: string, y: number}>> = [[]];
	let radarChartData : Array<Array<number>> = [[]];
	let seaMapData : Array<Array<number>> = [[]]; //Lats & longs
	let statsData : Map<String,[number,number,number]>;

	let startTime = "";
	let endTime = "";
	let startIndex = 0;
	let stopIndex = 0;

	//Update the data to the graph when there is a change in the menu
	$: $sRunChartFields, updateSlide()
	$: $sRadarChartFields, updateSlide()

	async function updateSlide() {
		if($sFileHandle.isLoaded){
			handleSlide({detail: {values: [startIndex,stopIndex]}});
		}
	}

	async function handleSlide(event){
		startIndex = event.detail.values[0];
		stopIndex = event.detail.values[1];

		seaMapData = await invoke('get_path', {'start': startIndex, 'stop': stopIndex});
		runChartData = await invoke('get_slices',{
													'fields': $sRunChartFields,
													'start': startIndex,
													'stop' : stopIndex
										});
		radarChartData = await invoke('get_radars',{
													'fields': $sRadarChartFields,
													'start': startIndex,
													'stop': stopIndex

										});
		statsData = await invoke('get_stats',{
												'fields': $sStatsFields,
												'start': startIndex,
												'stop': stopIndex
												});
		startTime = runChartData[0][0].x;
		endTime = runChartData[0][runChartData[0].length-1].x;
	}

	async function readFile(){
		sFileHandle.isLoading();
		await invoke('read_file',{'fileName': $sFileHandle.name });
		await invoke('get_file_lines').then((lines: number) => sFileHandle.setLines(lines));
		await handleSlide({detail: {values: [0,$sFileHandle.lines]}});
		sFileHandle.setLoaded(true);
	}

	//For debugging purposes
	/*onMount(
		() => 	{
				sFileHandle.setName("/Users/thorsten/Projects/Programming/testdata/reader2_output_002.csv");
				readFile();
				}
	);*/
	
</script>

<main>
	{#if $sFileHandle.isLoaded}
		<div class="menuBox">
			<Navigation on:fileRead={readFile}/>
		</div>
		<div class="slideBox">
			{startTime} - {endTime}<br/>
			<RangeSlider on:change={handleSlide} 
						range pushy 
						max={$sFileHandle.lines} 
						values={[0,$sFileHandle.lines]}
						springValues={{stiffness: 1, damping: 1}}
						/>
		</div>
		<div class="contentGrid">
			<div class="runChartBox">
				<RunChart {runChartData} />
			</div>
			<div>
				<Stats {statsData} />
			</div>
			<div class="mapBox">
				<SeaMap path={seaMapData} />
			</div>
			<div class="radarChartBox">
				<RadarChart {radarChartData} />
			</div>
		</div>
	{:else if $sFileHandle.isLoading}
		Loading {$sFileHandle.name} ...
	{:else}
		<ChooseFile on:fileRead={readFile}/>
	{/if}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 1280px;
		margin: 0 auto;
	}
	.mapBox{
		grid-column-start: 1;
		grid-column-end: 3;
		height: 100%;
	}
	.radarChartBox{
		grid-column-start: 3;
		grid-column-end: 5;
	}
	.runChartBox{
		grid-column-start: 1;
		grid-column-end: 4;
		padding-bottom: 25px;
	}
	.contentGrid{
		margin-top: 50px;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr 1fr;
	}
	.menuBox{
		position: fixed;
		top: 0;
		right: 0;
		z-index: 100;
	}
	.slideBox{
		width: 98%;
		position: fixed;
		top: 0px;
		left: 1%;
		padding-top: 10px;
		background-color: white;
		text-align: center;
		z-index: 99;
		margin: auto;
	}
	@media (min-width: 1280px) {
		main {
			max-width: none;
		}
	}
	* {
    	-webkit-touch-callout:none;
    	-webkit-user-select:none;
    	-moz-user-select:none;
    	-ms-user-select:none;
    	user-select:none;
	}
</style>