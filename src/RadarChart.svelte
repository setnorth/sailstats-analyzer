<script>
	import Chart from 'chart.js/auto';
	import 'chartjs-adapter-moment';

	import { onMount } from 'svelte';
	import { sRadarChartFields } from './stores';

	export let radarChartData;

	let chartCanvas; //Canvas where the chart is drawn
	let chart;		 //The chart itself

	$: if(chart != undefined){
		chart.data.datasets = buildDatasets(radarChartData);
		chart.update('none');
	}

	//Colors for the graph
	const cMap = new Map();
    cMap.set("avg,tws,twd","#008CF9");
    cMap.set("avg,aws,awa","#EBAC23");
    cMap.set("avg,sog,cog","#B80058");
    cMap.set("avg,stw,hdg","#D163E6");
	cMap.set("dist,twa","#00C6F8");
	cMap.set("dist,awa","#006E00");
	cMap.set("dist,cog","#00A76C");
	cMap.set("dist,hdg","#B24502");

	const nMap = new Map()    
	nMap.set("avg,tws,twd","Average TWS");
    nMap.set("avg,aws,awa","Average AWS");
	nMap.set("avg,sog,cog","Average SOG");
	nMap.set("avg,stw,hdg","Average STW");
	nMap.set("dist,twa","Ratio TWA");
	nMap.set("dist,awa","Ratio AWA");
	nMap.set("dist,cog","Ratio COG");
	nMap.set("dist,hdg","Ratio HDG");
	
	function buildDatasets(rData){
		let sets = [];
		for(let i = 0; i < rData.length; i++){
			let dataset = {
				label: nMap.get($sRadarChartFields[i]),
				borderColor: cMap.get($sRadarChartFields[i]),
				showLine: true,
				data: rData[i],
			}
			sets.push(dataset);
		}
		return sets;		
	}



	onMount(() => {
		let ctx = chartCanvas.getContext('2d');
		chart = new Chart(ctx,{
			type: 'radar',
  			data:{  
				labels: [...Array(360)].map((_,i) => (i % 45 == 0) ? i.toString() : " "),
  				datasets: [{
    				label: 'My First Dataset',
    				data: [...Array(360).keys()],
    				fill: true,
    				backgroundColor: 'rgba(255, 99, 132, 0.2)',
    				borderColor: 'rgb(255, 99, 132)',
    				pointBackgroundColor: 'rgb(255, 99, 132)',
    				pointBorderColor: '#fff',
    				pointHoverBackgroundColor: '#fff',
    				pointHoverBorderColor: 'rgb(255, 99, 132)'
  				}]
			},
  			options: {
				responsive: true,
						plugins: {
							legend: true,
							tooltip: {
								enabled: true,
								callbacks: {
                    				title: function(context) {
                        				return context[0].dataIndex.toString().concat("°");
                    				}
                				}
							},
							title: {
								display: true,
								text: "Radar Chart"
							}
						},				  
    			elements: {
      				line: {
        				borderWidth: 3
      				}		
    			}
				
  			}
		});
	});
</script>
<canvas bind:this={chartCanvas}></canvas>