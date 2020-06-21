import { observable, runInAction, action } from "mobx";
import wasm from "converter";

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
            wasm.init("canvas_element")
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