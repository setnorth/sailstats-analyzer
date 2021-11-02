<script lang="ts">
    import StatRow from "./StatRow.svelte";
    export let statsData : Map<String,[number,number,number]>;

</script>

<table class="stats">
    <tr>
        <th>&nbsp;</th>
        <th colspan="2">Min</th>
        <th colspan="2">Avg</th>
        <th colspan="2">Max</th>
    </tr>
    <StatRow name="TWS" values={statsData["tws"]} unit="kt"/>
    <StatRow name="AWS" values={statsData["aws"]} unit="kt"/>
    <StatRow name="SOG" values={statsData["sog"]} unit="kt"/>
    <StatRow name="STW" values={statsData["stw"]} unit="kt"/>
    <StatRow name="TWD" values={statsData["twd"]} unit="&deg;"/>
    <StatRow name="TWA" values={statsData["twa"]} unit="&deg;"/>
    <StatRow name="AWA" values={statsData["awa"]} unit="&deg;"/>
    <StatRow name="COG" values={statsData["cog"]} unit="&deg;"/>
    <StatRow name="HDG" values={statsData["hdg"]} unit="&deg;"/>
    <StatRow name="Pitch" values={statsData["pitch"]} unit="&deg;" padThreeZeroes={false}/>
    <StatRow name="Heel" values={statsData["roll"]} unit="&deg;" padThreeZeroes={false}/>
    <StatRow name="Rudder" values={statsData["rudder_angle"]} unit="&deg;" padThreeZeroes={false}/>
</table>
<style>
    .stats {
        width: 100%;
        margin: 20px;
        margin-right: 20px;
        margin-top: 10%;
        font-size: 1.5vw;
    }
</style>