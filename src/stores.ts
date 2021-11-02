import { writable, readable, derived, Writable } from 'svelte/store';

/// Handles the file and its status inside the program
function createFileHandle() {
	const { subscribe, update } = writable({name: "", isLoaded: false, isLoading: false, lines: 0});

	return {
		subscribe,
		setName: (name: string) => 
                        update((h) => { h.name = name; 
                                        return h
                        }),
		setLoaded: (loaded: boolean) => 
                        update((h) => { h.isLoaded = loaded;
                                        h.isLoading = false;
                                        return h
                        }),
        isLoading: () =>
                        update((h) => {
                                        h.isLoaded = false;
                                        h.isLoading = true;
                                        return h
                        }),
        setLines: (lines: number) =>
                        update((h) => { h.lines = lines;
                                        return h;
                        })
	};
}

export const sFileHandle = createFileHandle();
export const sRunChartFields = writable(["tws","sog"]);
export const sRadarChartFields = writable(["avg,tws,twd","avg,sog,cog"]);
export const sStatsFields = readable(["tws","twd","twa","aws","awa","cog","sog","hdg","stw","pitch","roll","rudder_angle"]);
