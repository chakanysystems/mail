<script lang="ts">
	import '../app.pcss';
	import { onMount } from 'svelte';
	import { page } from "$app/stores"
	import { invoke } from '@tauri-apps/api/tauri';
	import Avatar from '$lib/components/Avatar.svelte';
	import Name from '$lib/components/Name.svelte';
	import NostrAddress from '$lib/components/NostrAddress.svelte';
	import { Mail, MessageCircle, LifeBuoy, Cog } from "lucide-svelte";
	import { Input } from '$lib/components/ui/input/index';

	let publickey: string = '';
	onMount(async () => {
		document.documentElement.classList.add('dark');
		invoke('get_pubkey').then((k) => {
			console.log('key from rust:', k);
		});
	});

	$: path = $page.route.id ?? "/"
</script>

<div>
	<div class="flex h-full min-h-screen">
		<div class="flex border-r">
			<aside class="flex flex-col gap-5 h-full max-h-screen">
				<div class="flex font-semibold">
					Mail
				</div>
				<nav class="flex flex-col px-2 gap-3 h-full max-h-screen text-sm">
					<div class="flex flex-col gap-3 grow">
						<a
							href="/mail"
							class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary {path.startsWith('/mail') ? 'bg-muted text-primary' : 'text-muted-foreground'}"
						>
							<Mail />
							Mail
						</a>
						<a
							href="##"
							class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary"
						>
							<MessageCircle />
							Messages
						</a>
					</div>
					<div class="flex-none">
						<div class="flex flex-col gap-6">
							<div class="flex flex-col gap-1">
								<a
									href="##"
									class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary"
								>
									<LifeBuoy />
									Support
								</a>
								<a
									href="/settings"
									class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary {path.startsWith('/settings') ? 'bg-muted text-primary' : 'text-muted-foreground'}"
								>
									<Cog />
									Settings
								</a>
							</div>
							<hr />
							<div class="flex gap-3 pb-6 px-4">
								<Avatar />
								<div>
									<div>
										<Name />
									</div>
									<div>
										<NostrAddress />
									</div>
								</div>
							</div>
						</div>
					</div>
				</nav>
			</aside>
		</div>

		<div class="flex flex-col gap-3 w-full max-w-screen">
			<div class="px-2 py-4">
				<Input type="search" placeholder="Search..." class="" />
			</div>
			<hr />
			<div class="pl-2">
				<slot />
			</div>
		</div>
	</div>
</div>
