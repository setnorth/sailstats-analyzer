<script>
	import Chart from 'chart.js/auto';
	import 'chartjs-adapter-moment';
	import moment from "moment";
	import { sRunChartFields } from './stores';

	import { onMount } from 'svelte';

	export let runChartData; // Vector<Vector<{x,y}>>

	let chartCanvas; //Canvas where the chart is drawn
	let chart;		 //The chart itself
		
	$: if(chart != undefined){
			chart.data.datasets = buildDatasets(runChartData);
			chart.options.scales = buildScales(runChartData);
			chart.update('none');
		}

	//Colors for the graph
	const cMap = new Map();
    cMap.set("aws","#EBAC23"); cMap.set("aws_lp","#EBAC23");
    cMap.set("awa","#006E00"); cMap.set("awa_lp","#006E00");
    cMap.set("tws","#008CF9"); cMap.set("tws_lp","#008CF9");
   	cMap.set("sog","#B80058");
    cMap.set("stw","#D163E6"); cMap.set("stw_lp","#D163E6");
    cMap.set("roll","#5954d6"); cMap.set("roll_lp","#5954d6");
    cMap.set("rudder_angle","#878500"); cMap.set("rudder_angle_lp","#878500");


	const nMap = new Map();
    nMap.set("aws","Apparent Wind Speed (kts)"); nMap.set("aws_lp","Apparent Wind Speed (kts)");
    nMap.set("awa","Apparent Wind Angle (°)"); nMap.set("awa_lp","Apparent Wind Angle (°)");
    nMap.set("tws","True Wind Speed (kts)"); nMap.set("tws_lp","True Wind Speed (kts)");
    nMap.set("sog","SOG (kts)");
    nMap.set("stw","STW (kts)"); nMap.set("stw_lp","STW (kts)");
    nMap.set("roll","Heel (°)"); nMap.set("roll_lp","Heel (°)");
    nMap.set("rudder_angle","Rudder Deflection (°)"); nMap.set("rudder_angle_lp","Rudder Deflection (°)");

	//Quick hack to get same axis for:
	// aws,tws,aws_lp,tws_lp
	// sog,stw,stw_lp
	// roll,rudder_angle,roll_lp,rudder_angle_lp
	function getYAxis(field){
		if(field == "tws" || field == "aws_lp" || field == "tws_lp"){
			return "aws";
		}else if(field == "awa_lp"){
			return "awa";
		}else if(field == "stw" || field == "stw_lp"){
			return "sog";
		}else if(field == "rudder_angle" || field == "roll_lp" || field == "rudder_angle_lp"){
			return "roll";
		}else{
			return field;		
		}
	}
	
	function buildDatasets(mgData){
		let sets = [];
		for(let i = 0; i < mgData.length; i++){
			let dataset = {
				label: nMap.get($sRunChartFields[i]),
				borderColor: cMap.get($sRunChartFields[i]),
				showLine: true,
				data: mgData[i],
				yAxisID: getYAxis($sRunChartFields[i]),
			}
			sets.push(dataset);
		}		
		return sets;
	}


	function buildScales(_mgData){
		//The default format for the x scale
		//let scales = Object.assign({}, chart.options.scales); //Does not work ...
		let scales = {
			x: {     					        
				type: 'time',
				time: {
					parser: (x) => moment(x, "HH:mm:ss.SSS"),
					displayFormats: {
						day: 'HH:mm:ss',
						hour: 'HH:mm:ss',
						second: 'HH:mm:ss',
						millisecond: 'HH:mm:ss.SSS'
					}
				}
			} 
		};
		return scales;
	}

	onMount(() => {
		let ctx = chartCanvas.getContext('2d');

		chart = new Chart(ctx, {
					type: 'scatter',
					data: {
							showtooltips: true,
							datasets: [{
								label: "Apparent Wind Speed",
								borderColor: "rgba(1, 79, 134, 0.7)",
								showLine: true,
								data: runChartData[0]
							}]
					},
					options: {
						responsive: true,
						plugins: {
							legend: true,
							tooltip: {
								enabled: true,
							},
							title: {
								display: true,
								text: "Run Chart"
							}
						},
        				scales: {
        				    x: {
    					        type: 'time',
								time: {
									parser: (x) => moment(x, "HH:mm:ss.SSS"),
									displayFormats: {
										day: 'HH:mm:ss',
										hour: 'HH:mm:ss',
										second: 'HH:mm:ss',
										millisecond: 'HH:mm:ss.SSS'
									}
								},
        			    	},
						}	
    				}
				});
		});
</script>
<canvas bind:this={chartCanvas}></canvas>