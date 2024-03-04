import React, { FC, Dispatch, SetStateAction } from "react";
import { Box, Collapse, Group, Text, ThemeIcon, UnstyledButton, rem } from "@mantine/core";

import { prevent_default } from "@/utils/utils.ts";

import classes from "@/styles/Layout.module.css";
import { IconChevronRight } from "@tabler/icons-react";

interface CategoryProperties {
    icon: FC<any>;
    label: string;
    is_open: boolean;
    on_toggle: () => void;
    links?: { label: string; link: string }[];
}

export const toggle_category = (categories: boolean[], set_categories: Dispatch<SetStateAction<boolean[]>>, index: number) => {
    const updated_categories = new Array(categories.length).fill(false);
    updated_categories[index] = !categories[index];

    set_categories(updated_categories);
};
const Button = ({ label, link }: { label: string; link: string }) => {
    return (
        <Text<'a'> className={classes.category_link} component="a" key={label} href={link} onClick={prevent_default}>
            {label}
        </Text>
    );
}

export const NavigationCategory: React.FC<CategoryProperties> = ({ icon: Icon, label, is_open, on_toggle, links }) => {
    const items = (Array.isArray(links) ? links : []).map((link, index) => (
        <Button key={index} label={link.label} link={link.link} />
    ));

    return (
        <>
            <UnstyledButton onClick={on_toggle} className={classes.navigation_category}>
                <Group justify="space-between" gap={0}>
                    <Box style={{ display: 'flex', alignItems: 'center' }}>
                        <ThemeIcon variant="light" size={30}>
                            <Icon style={{ width: rem(18), height: rem(18) }} />
                        </ThemeIcon>
                        <Box ml="md">{label}</Box>
                    </Box>
                    {Array.isArray(links) && (
                        <IconChevronRight className={classes.category_chevron} stroke={1.5}
                            style={{
                                width: rem(16),
                                height: rem(16),
                                transform: is_open ? 'rotate(-90deg)' : 'none',
                            }}
                        />
                    )}
                </Group>
            </UnstyledButton>
            {Array.isArray(links) && <Collapse in={is_open}>{items.map((item) => (
                <div key={item.key}>{item}</div>
            ))}</Collapse>}
        </>
    );
}