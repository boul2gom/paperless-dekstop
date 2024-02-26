import classes from "@/src/styles/Layout.module.css";
import Image from "next/image";
import {Code, Group} from "@mantine/core";
import { Suspense } from "react";
import { fetch_backend } from "../../Utils/Utils";
import useSWR from "swr";

const LogoSkeleton = () => {
    return (
        <Code fw={700} className={classes.release_block}>...</Code>
    );
}

export default function Logo() {
    const { data } = useSWR("latest_release", () => fetch_backend<string>("Logo", "fetch_latest_paperless_release"));

    return (
        <>
            <Group justify="space-between" className={classes.logo}>
                <Image src={"paperless-ngx.png"} alt={"Paperless Logo"} width={159} height={60}/>
                <Suspense fallback={<LogoSkeleton />}>
                    <Code fw={700} className={classes.release_block}>{data}</Code>
                </Suspense>
            </Group>
        </>
    )
}