import { MouseEvent } from "react";
import dynamic from "next/dynamic";
import { WrapperProperties } from "@/src/components/Utils/InvokeWrapper";
import {invoke, InvokeArgs} from "@tauri-apps/api/tauri";

export const prevent_default = (event: MouseEvent) => event.preventDefault();

export const dynamic_invoke = <T>() => {
    return dynamic<WrapperProperties<T>>(() => import(`@/src/components/Utils/InvokeWrapper`), {
        ssr: false,
    });
}

export function invoke_backend(caller: string, command: string, args?: InvokeArgs, callback?: (result: any) => void) {
    invoke(command, args)
        .then((result) => callback && callback(result))
        .catch((error) => console.error('[' + caller + '] Error: ' + error));

}