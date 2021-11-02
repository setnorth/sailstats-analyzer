<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { createEventDispatcher } from "svelte";
    import { sFileHandle } from './stores.js'

    const dispatch = createEventDispatcher();

    function chooseFile(){
        open().then(
            (fName) =>  {   if (fName != null) { 
                                sFileHandle.setName(fName.toString());
                                dispatch('fileRead');
                            }
                        }
        );
    }
</script>

<input type="button" on:click={chooseFile} value="Open file"/>