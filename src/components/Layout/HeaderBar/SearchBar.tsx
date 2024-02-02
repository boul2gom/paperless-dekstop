import {useDebouncedState} from "@mantine/hooks";
import {ChangeEvent, useEffect, useState} from "react";
import {Spotlight, spotlight, SpotlightActionData} from "@mantine/spotlight";
import {invoke_backend} from "@/src/components/Utils/Utils";
import {IconFileText, IconSearch} from "@tabler/icons-react";
import {Group, rem, Text, UnstyledButton} from "@mantine/core";
import classes from "@/src/styles/Layout.module.css";

export function SearchBar() {
    const [search_query, set_search_query] = useDebouncedState('', 300)
    const [search_results, set_search_results] = useState<SpotlightActionData[]>([]);

    const handle_search_change = (event: ChangeEvent<HTMLInputElement>) => {
        set_search_query(event.target.value);
    };

    useEffect(() => {
        if (!search_query && search_results.length > 0) return;

        // Maybe group results by type (document, tag, etc.) will be better. Check doc fo this.
        invoke_backend("Header", "documents_query", {query: search_query}, results => {
            const formatted_results: SpotlightActionData[] = results.map((result: string, index: number) => ({
                id: index,
                label: result,
                description: 'Document: ' + result,
                leftSection: <IconFileText style={{ width: rem(24), height: rem(24) }} stroke={1.5} />,
            }));

            set_search_results(formatted_results);
        });
    }, [search_query]);

    return (
        <>
            <UnstyledButton className={classes.search_bar} onClick={() => spotlight.open()}>
                <Group style={{ justifyContent: 'space-between' }}>
                    <Group>
                        <IconSearch style={{ width: rem(15), height: rem(15) }} stroke={1.5} />
                        <Text fz="sm" c="dimmed" pr={80}>
                            Search for document...
                        </Text>
                    </Group>
                    <Text fw={700} className={classes.search_bar_shortcut}>
                        Ctrl + F
                    </Text>
                </Group>
            </UnstyledButton>
            <Spotlight
                onChange={handle_search_change}
                actions={search_results}
                shortcut={['mod + f']}

                limit={7}
                scrollable

                nothingFound="No documents found"
                searchProps={{
                    leftSection: <IconSearch style={{ width: rem(20), height: rem(20) }} stroke={1.5} />,
                    placeholder: 'Search for document...',
                }}
            />
        </>
    )
}