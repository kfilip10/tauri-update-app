import { invoke } from '@tauri-apps/api/core';

export async function launchShinyApp() {
    try {
        const url = await invoke<string>('start_r_shiny');
        window.open(url, '_blank');
    } catch (error) {
        console.error("Failed to launch R Shiny app:", error);
    }
}

export async function stopShinyApp() {
    try {
        await invoke('stop_r_shiny');
        console.log("R Shiny app stopped.");
    } catch (error) {
        console.error("Failed to stop R:", error);
    }
}
