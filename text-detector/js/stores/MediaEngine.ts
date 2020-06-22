import { observable, action, runInAction } from 'mobx';

export interface VideoData {
    media: MediaStream | null;
}

export class MediaEngine {
    @observable
    public initalized = false;
    
    @observable
    public instance!: VideoData;

    constructor(private readonly constraints: MediaStreamConstraints) {}

    @action
    public async getMedia() {
        const payload: VideoData = {
            media: null,
        }

        try {
            const stream = await navigator.mediaDevices.getUserMedia(this.constraints);
            if (stream) {
                payload.media = stream
            }
            
        } catch (err) {
            throw err;
        }

        runInAction(() => {
            this.instance = payload
            this.initalized = true;
        })
    }
}
