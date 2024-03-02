import classes from "@/src/styles/Layout.module.css";
import Image from "next/image";
import {Code, Group, Skeleton} from "@mantine/core";
import { Suspense } from "react";
import { useSuspenseQuery } from '@tanstack/react-query'
import {fetcher} from "@/src/components/Utils/Utils";

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
            <Image src={"paperless-ngx.png"} alt={"Paperless Logo"} width={159} height={60}/>
            <Suspense fallback={<ReleaseSkeleton />}>
                <ReleaseBlock />
            </Suspense>
        </Group>
    )
}