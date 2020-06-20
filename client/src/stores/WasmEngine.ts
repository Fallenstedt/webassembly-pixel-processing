import { observable, runInAction, action } from "mobx";
import wasm from "converter";

type ConverterType = typeof wasm

export class WasmEngine {
    @observable
    public instance: ConverterType | undefined = undefined;

    @observable
    public error: Error | undefined = undefined;

    @action
    public async initialize() {
        try {
            const wasm = await import("converter");
            runInAction(() => {
                this.instance = wasm
            })
        } catch (error) {
            runInAction(() => {
                this.error = error
            })
        }
    }
}