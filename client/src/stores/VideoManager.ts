import { observable, action, runInAction } from 'mobx';

export interface VideoData {
    media: MediaStream | null;
    videoTrack: MediaStreamTrack | null;
}

export class VideoManager {
    @observable
    public initalized = false;
    
    @observable
    public instance!: VideoData;

    constructor(private readonly constraints: MediaStreamConstraints) {}

    @action
    public async getMedia() {
        const payload: VideoData = {
            media: null,
            videoTrack: null,
        }

        try {
            const stream = await navigator.mediaDevices.getUserMedia(this.constraints);
            if (stream) {
                payload.media = stream
            }

            const videoTracks = stream.getVideoTracks()
            
            if (videoTracks.length){
                payload.videoTrack = videoTracks[0]
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
