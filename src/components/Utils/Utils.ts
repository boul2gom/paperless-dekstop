import { MouseEvent } from "react";
import {invoke, InvokeArgs} from "@tauri-apps/api/tauri";

export const prevent_default = (event: MouseEvent) => event.preventDefault();

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