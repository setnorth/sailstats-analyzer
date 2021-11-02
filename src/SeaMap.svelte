<script lang="ts">
    import 'leaflet/dist/leaflet.css';    
    import L from 'leaflet';
    import { onMount, tick } from 'svelte';

    let map = undefined;        //the map object itself
    let polyLine = undefined;   //the polyline drawing the path
    
    export let path = [];

    $: if(map){
        polyLine.setLatLngs(path);
        polyLine.redraw();
    }

    async function createMap(){ 
        map = L.map("map").setView([60.421352,5.2431026], 13);
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            }).addTo(map);

        polyLine = L.polyline([[0,0]]).addTo(map);
        
        await tick();
        map.invalidateSize(false);
    };

    onMount(() => {
        createMap();
    });
</script>
<svelte:window on:scroll={() => map.invalidateSize(false)} />

<div id="map"></div>
<style>
    #map {
        height: 100%;
        z-index: 1;
        }
</style>