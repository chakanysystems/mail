<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { onMount } from 'svelte';

	let data: any;
	export let pubkey = "";

	onMount(async () => {
		try {
			if (pubkey) {
				data = await invoke("get_profile", { pubkey }); // not implemented
			} else {
				data = await invoke("get_profile");
			}
			console.log(data)
		} catch (e) {
			console.error(e);
		}
	})
</script>

{#if data}
	{data.display_name ?? (data.name ?? "??")}
{/if}