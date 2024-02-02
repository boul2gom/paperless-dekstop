import classes from "@/src/styles/Layout.module.css";
import Image from "next/image";
import {Code, Group} from "@mantine/core";
import {useState} from "react";
import {dynamic_invoke} from "@/src/components/Utils/Utils";

export function Logo() {
    const [latest_release, set_latest_release] = useState("0.0.0");
    const TauriWrapComponent = dynamic_invoke<string>();

    return (
        <>
            <Group justify="space-between" className={classes.logo}>
                <Image src={"paperless-ngx.png"} alt={"Paperless Logo"} width={159} height={60}/>
                <Code fw={700} className={classes.release_block}>{latest_release}</Code>
            </Group>
            
            <TauriWrapComponent caller="Navigation" command="fetch_latest_paperless_release" set_state={set_latest_release} />
        </>
    )
}