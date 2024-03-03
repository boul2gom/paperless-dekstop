import classes from "@/styles/Layout.module.css";
import {Code, Group, Skeleton, Image } from "@mantine/core";
import { Suspense } from "react";
import { useSuspenseQuery } from '@tanstack/react-query'
import { fetcher } from "@/components/Utils/Utils";

import logo from "@/assets/paperless-ngx.png";

const ReleaseSkeleton = () => {
    return (
        <Skeleton radius="xl" width={100} height={22.59} className={classes.release_block} />
    );
}

const ReleaseBlock = () => {
    const { data } = useSuspenseQuery({
        queryKey: ["latest_release"],
        queryFn: () => fetcher<string>("latest_release"),
    });

    return (
        <Code fw={700} className={classes.release_block}>{data}</Code>
    );
}

export default function Logo() {
    return (
        <Group justify="space-between" className={classes.logo}>
            <Image src={logo} alt="Paperless Logo" width={159} height={60} />
            <Suspense fallback={<ReleaseSkeleton />}>
                <ReleaseBlock />
            </Suspense>
        </Group>
    )
}