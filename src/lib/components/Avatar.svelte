<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { onMount } from 'svelte';
	import * as UiAvatar from "$lib/components/ui/avatar"

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

	$: initials = data ? (data.display_name ? data.display_name.split(" ").filter((s: string) => s.charAt(0).toUpperCase()) : "?") : "?"
</script>

<UiAvatar.Root>
	<UiAvatar.Image src={data ? data.picture : ""} alt={data ? `@${data.name}` : "Your Profile"} />
	<UiAvatar.Fallback>{initials}</UiAvatar.Fallback>
</UiAvatar.Root>