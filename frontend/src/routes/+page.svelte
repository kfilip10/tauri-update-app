<script lang="ts">
	import { Alert, Button } from 'flowbite-svelte';

	import UpdateProgress from '$lib/components/UpdateProgress.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	import {
		checkForUpdates,
		updateProgressVisible,
		handleUpdateComplete,
		handleUpdateError
	} from '$lib/utils/updater';

	import { launchShinyApp, stopShinyApp } from '$lib/utils/shiny';
	let rscriptPath = '';
	let shinyPath = '';
	let message = 'Click the button to run Rust backend!';

	async function callRust() {
		const { invoke } = await import('@tauri-apps/api/core');
		message = await invoke('greet', { name: 'User' });
	}

	async function testDownload() {
		const { invoke } = await import('@tauri-apps/api/core');
		const url =
			'https://github.com/kfilip10/tauri-update-app/releases/download/v0.0.2/tauri-updater_0.0.2_x64_en-US.msi.zip';
		try {
			await invoke('download_file', { url });
			console.log('Download successful!');
		} catch (error) {
			console.error('Download error:', error);
		}
	}

	async function handleUpdateCheck() {
		await checkForUpdates();
	}

	async function handleShinyStart() {
		await launchShinyApp();
	}
	async function handleShinyStop() {
		await stopShinyApp();
	}
	async function fetchRscriptPath() {
		const { invoke } = await import('@tauri-apps/api/core');

		try {
			rscriptPath = await invoke<string>('get_rscript_path');
			console.log('Rscript path:', rscriptPath);
			//combine the paths with a new line inbetween
			rscriptPath = rscriptPath;
		} catch (error) {
			console.error('Failed to get Rscript path:', error);
			rscriptPath = 'Error retrieving Rscript path';
		}
	}
	async function testRExecution() {
		try {
			const result = await invoke('test_r_script');
			console.log('R test result:', result);
		} catch (error) {
			console.error('R test error:', error);
		}
	}
	onMount(async () => {
		await fetchRscriptPath();
	});
</script>

<main>
	<h1>Welcome to Tauri and SvelteKit: This is an updated version v2</h1>
	<p>{message}</p>
	<Button on:click={callRust}>Call Rusty</Button>
	<div class="p-8">
		<Alert>
			<span class="font-medium">Info alert!</span>
			Change a few things up and try submitting again.
		</Alert>
	</div>
	<Button on:click={handleUpdateCheck}>Check for Updates</Button>

	<!-- Progress modal -->
	<UpdateProgress
		bind:open={$updateProgressVisible}
		onComplete={handleUpdateComplete}
		onError={handleUpdateError}
	/>

	<h1>Shiny App</h1>
	<p>Rscript Path: {rscriptPath}</p>
	<Button on:click={handleShinyStart}>Start Shiny App</Button>
	<Button on:click={handleShinyStop}>Stop Shiny App</Button>
	<Button on:click={testRExecution}>Test R Execution</Button>
</main>
