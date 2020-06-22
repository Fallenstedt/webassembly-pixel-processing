import { observable, runInAction, action } from "mobx";

type ConverterType = typeof import("../../pkg/index.js");

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
            //@ts-ignore
            const wasm = await import("../../pkg/index.js");
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