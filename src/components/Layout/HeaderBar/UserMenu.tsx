import { useState } from "react";
import { Avatar, Group, Menu, rem, Text, UnstyledButton, useMantineTheme } from "@mantine/core";

import cx from "clsx";
import classes from "@/styles/Layout.module.css";
import { IconChevronDown, IconFileTypePdf, IconFolderSearch, IconHeart, IconLogout, IconPhoto, IconSettings, IconSwitchHorizontal } from "@tabler/icons-react";

interface UserProperties {
    name: string;
    image: string;
}

export function UserMenu({ name, image }: UserProperties) {
    const theme = useMantineTheme();
    const [user_opened, set_user_opened] = useState(false);

    return (
        <>
            <Menu
                withArrow={true}
                width={260}
                position="bottom-end"
                transitionProps={{transition: 'pop-bottom-right'}}
                trigger="click-hover" openDelay={100} closeDelay={400}
                onClose={() => set_user_opened(false)}
                onOpen={() => set_user_opened(true)}
            >
                <Menu.Target>
                    <UnstyledButton className={cx(classes.user_menu, {[classes.user_menu_active]: user_opened})}>
                        <Group gap={7}>
                            <Avatar src={image} alt={name} radius="xl" size={20}/>
                            <Text fw={500} size="sm" lh={1} mr={3}>
                                {name}
                            </Text>
                            <IconChevronDown style={{width: rem(12), height: rem(12)}} stroke={1.5}/>
                        </Group>
                    </UnstyledButton>
                </Menu.Target>
                <Menu.Dropdown>
                    <Menu.Item
                        leftSection={
                            <IconHeart style={{width: rem(16), height: rem(16)}} color={theme.colors.red[6]} stroke={1.5}/>
                        }
                    > Favourites
                    </Menu.Item>
                    <Menu.Item
                        leftSection={
                            <IconPhoto style={{width: rem(16), height: rem(16)}} color={theme.colors.green[6]} stroke={1.5}/>
                        }
                    > Photos
                    </Menu.Item>
                    <Menu.Item
                        leftSection={
                            <IconFileTypePdf style={{width: rem(16), height: rem(16)}} color={theme.colors.orange[6]} stroke={1.5}/>
                        }
                    > Documents
                    </Menu.Item>
                    <Menu.Item
                        leftSection={
                            <IconFolderSearch style={{width: rem(16), height: rem(16)}} color={theme.colors.blue[6]} stroke={1.5}/>
                        }
                    > All documents
                    </Menu.Item>

                    <Menu.Divider/>

                    <Menu.Label>Settings</Menu.Label>
                    <Menu.Item
                        leftSection={
                            <IconSettings style={{width: rem(16), height: rem(16)}} stroke={1.5}/>
                        }
                    > Account settings
                    </Menu.Item>
                    <Menu.Item
                        leftSection={
                            <IconSwitchHorizontal style={{width: rem(16), height: rem(16)}} stroke={1.5}/>
                        }
                    > Change account
                    </Menu.Item>
                    <Menu.Item
                        color="red"
                        leftSection={
                            <IconLogout style={{width: rem(16), height: rem(16)}} stroke={1.5}/>
                        }
                    > Logout
                    </Menu.Item>
                </Menu.Dropdown>
            </Menu>
        </>
    )
}