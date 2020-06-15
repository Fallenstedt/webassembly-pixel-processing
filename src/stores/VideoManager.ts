import { observable, action, runInAction, when } from 'mobx';

export interface VideoData {
    media: MediaStream | null;
    videoTrack: MediaStreamTrack | null;
    imageCapture: ImageCapture | null;
}

export class VideoManager {
    @observable
    public initalized = false;
    
    @observable
    public instance!: VideoData;

    @observable
    public capturedFrames: Blob[] = []

    constructor(private readonly constraints: MediaStreamConstraints) {
        when(
            () => this.initalized && this.instance.imageCapture !== null,
            () => {
                setInterval(() => {
                    this.instance.imageCapture!.takePhoto().then(data => {
                        this.capturedFrames.push(data)
                    }).catch(err => {})
                }, 10)

                setInterval(() => {
                    console.log(this.capturedFrames)
                    runInAction(() => {
                        this.capturedFrames = []
                    })
                }, 1000)
            }
        )
    }
    
    @action
    public async getMedia() {
        const payload: VideoData = {
            media: null,
            videoTrack: null,
            imageCapture: null
        }

        try {
            const stream = await navigator.mediaDevices.getUserMedia(this.constraints);
            if (stream) {
                payload.media = stream
            }

            const videoTracks = stream.getVideoTracks()
            
            if (videoTracks.length){
                payload.videoTrack = videoTracks[0]
                payload.imageCapture = new ImageCapture(videoTracks[0])
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
