<script lang="ts">
	import { onMount } from 'svelte';
	import { ndk } from "$lib/nostr"
	import { NDKUser, type NDKUserProfile } from '@nostr-dev-kit/ndk';

	export let pubkey: string;

	let profile: NDKUserProfile | null = null;

	onMount(async () => {
		if ($ndk.cacheAdapter) {
			// @ts-expect-error for some reason it thinks it's still undefined, idk why.
			profile = await $ndk.cacheAdapter.fetchProfile(pubkey)
		}
		if (!profile) { // (cache didn't work, or no cache)
			const user = new NDKUser({ pubkey })
			user.ndk = $ndk;
			profile = await user.fetchProfile();
			if ($ndk.cacheAdapter && profile)
				// @ts-expect-error again
				await $ndk.cacheAdapter.saveProfile(pubkey, profile)
		}
	})
</script>

<img class="rounded-full before:animate-pulse $$props.class" src={profile?.image} alt="Profile" />
