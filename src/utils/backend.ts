import {invoke, InvokeArgs} from "@tauri-apps/api/tauri";

export async function invoke_backend<T>(command: string, args?: InvokeArgs): Promise<T> {
    return new Promise((resolve, reject) => {
        invoke(command, args)
            .then((result) => resolve(result as T))
            .catch((exception) => {
                    reject(exception);
                    throw new Error('[' + command + '] Error: ' + exception);
                }
            );
    });
}

export const fetcher = async <T>(command: string) => invoke_backend<T>(command);
export const fetcher_with_args = async <T>(command: string, args: InvokeArgs) => invoke_backend<T>(command, args);