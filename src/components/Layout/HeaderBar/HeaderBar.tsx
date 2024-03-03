import { rem, Flex, ActionIcon, Tooltip } from '@mantine/core';
import { IconBell, IconGitBranch } from '@tabler/icons-react';

import { UserMenu } from "@/components/Layout/HeaderBar/UserMenu";
import { SearchBar } from "@/components/Layout/HeaderBar/SearchBar";

const user = {
    name: 'Template User',
    image: 'https://avatars.githubusercontent.com/u/56512795?v=4',
};

export function HeaderBar() {
    return (
        <>
            <Flex justify="center" align="center" gap="xl" style={{ height: '100%' }}>
                <SearchBar />

                <Flex justify="flex-end" align="center" gap="xs" style={{ width: "500px" }}>
                    <Tooltip.Group openDelay={500} closeDelay={100}>
                        <Tooltip label="Notifications" position="bottom">
                            <ActionIcon variant="default" size="lg" color="gray" mr={3}>
                                <IconBell style={{ width: rem(24), height: rem(24) }} stroke={1.5} />
                            </ActionIcon>
                        </Tooltip>

                        <Tooltip label="Source code" position="bottom">
                            <ActionIcon variant="default" size="lg" color="gray" mr={3} component="a" href="https://github.com/boul2gom/paperless-rs">
                                <IconGitBranch style={{ width: rem(24), height: rem(24) }} stroke={1.5} />
                            </ActionIcon>
                        </Tooltip>
                    </Tooltip.Group>

                    <UserMenu name={user.name} image={user.image} />
                </Flex>
            </Flex>
        </>
    );
}