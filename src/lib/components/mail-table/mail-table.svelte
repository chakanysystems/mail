<script lang="ts">
	import { createTable, Render, Subscribe, createRender } from 'svelte-headless-table';
	import { addSelectedRows } from "svelte-headless-table/plugins";
	import { readable } from 'svelte/store';
	import * as Table from "$lib/components/ui/table";
	import TableCheckbox from "./mail-table-checkbox.svelte"
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Button } from '$lib/components/ui/button';
	import { Pencil } from "lucide-svelte"

	type Msg = {
		id: string;
		author: string;
		content: string;
		date: string;
	}

	const data: Msg[] = [
		{
			id: "test",
			author: "Me",
			content: "Hello",
			date: "5 min ago"
		}
	]

	const table = createTable(readable(data), {
		select: addSelectedRows(),
	})

	const columns = table.createColumns([
		table.column({
			accessor: "id",
			cell: ({ row }, { pluginStates }) => {
				const { getRowState } = pluginStates.select;
				const { isSelected } = getRowState(row);

				return createRender(TableCheckbox, {
					checked: isSelected,
				});
			}
		}),
		table.column({
			accessor: "author",
			header: "Author"
		}),
		table.column({
			accessor: "content",
			header: "Content"
		}),
		table.column({
			accessor: "date",
			header: "Date"
		})
	])

	const { headerRows, pageRows, tableAttrs, tableBodyAttrs, flatColumns, rows, pluginStates } = table.createViewModel(columns);
	const { selectedDataIds, allPageRowsSelected } = pluginStates.select;
</script>

<div class="flex px-2 pb-2">
	<div class="flex gap-4 self-center grow">
		<div class="self-center">
			<Checkbox checked={$allPageRowsSelected} onCheckedChange={() => $allPageRowsSelected = !$allPageRowsSelected} />
		</div>
		<div class="font-semibold text-lg self-center">
			Inbox
		</div>
	</div>
	<div class="flex gap-4 self-center">
		<Button class="gap-1">
			Compose
			<Pencil />
		</Button>
	</div>
</div>
<div class="rounded-md border">
	<Table.Root {...$tableAttrs}>
		<Table.Header>
			{#each $headerRows as headerRow}
				<Subscribe rowAttrs={headerRow.attrs()}>
					<Table.Row>
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Header>
		<Table.Body {...$tableBodyAttrs}>
			{#each $pageRows as row (row.id)}
				<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
					<Table.Row
						{...rowAttrs}
						data-state={$selectedDataIds[row.id] && "selected"}
					>
						{#each row.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs>
								<Table.Cell {...attrs}>
									{#if cell.id == "date"}
										<div class="text-right">
											<Render of={cell.render()} />
										</div>
										{:else}
										<Render of={cell.render()} />
									{/if}
								</Table.Cell>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Body>
	</Table.Root>
</div>
<div class="flex-1 text-sm text-muted-foreground">
	{Object.keys($selectedDataIds).length} of{" "}
	{$rows.length} row(s) selected.
</div>