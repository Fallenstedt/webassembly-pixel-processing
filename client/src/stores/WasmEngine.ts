import { observable, runInAction, action } from "mobx";
import * as wasm from "converter";

type ConverterType = typeof wasm

export class WasmEngine {
    @observable
    public instance: ConverterType | undefined = undefined;

    @observable
    public loading: boolean = true;

    @observable
    public error: Error | undefined = undefined;

    @action
    public async initialize() {
        try {
            const wasm = await import("converter");
            wasm.init_web()
            runInAction(() => {
                this.loading = false;
                this.instance = wasm
            })
        } catch (error) {
            runInAction(() => {
                this.loading = false;
                this.error = error
            })
        }
    }
}