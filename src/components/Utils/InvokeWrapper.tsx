import { Dispatch, SetStateAction, useEffect } from 'react';
import {invoke_backend} from "@/src/components/Utils/Utils";

export interface WrapperProperties<T> {
    caller: string;
    command: string;
    set_state: Dispatch<SetStateAction<T>>

    dependencies?: any[];
}

export default function InvokeWrapper<T>({ caller, command, set_state, dependencies }: WrapperProperties<T>) {
    useEffect(() => {
        invoke_backend(caller, command, undefined, (result) => set_state(result as T));
    }, dependencies);

    return null;
}