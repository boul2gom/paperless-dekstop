import { MouseEvent, useEffect } from "react";
import dynamic from "next/dynamic";
import { WrapperProperties } from "@/src/components/Utils/InvokeWrapper";
import {invoke, InvokeArgs} from "@tauri-apps/api/tauri";

export const prevent_default = (event: MouseEvent) => event.preventDefault();

export const dynamic_invoke = <T>() => {
    return dynamic<WrapperProperties<T>>(() => import(`@/src/components/Utils/InvokeWrapper`), {
        ssr: false,
    });
}

export function invoke_backend(caller: string, command: string, args?: InvokeArgs, callback?: (result: any) => void, error?: (error: any) => void) {
    invoke(command, args)
        .then((result) => callback && callback(result))
        .catch((exception) => {
            error && error(exception);
            throw new Error('[' + caller + '] Error: ' + exception);
        });
}

export async function fetch_backend<T>(caller: string, command: string, args?: InvokeArgs): Promise<T> {
    return new Promise((resolve, reject) => {
        invoke_backend(caller, command, args, (result) => resolve(result), (error) => reject(error));
    });
}