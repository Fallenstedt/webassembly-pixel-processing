import React, { useEffect, useRef } from 'react';
import { useGlobalStore } from '../stores/use_video_manager';


export function Video() {
    const videoEl = useRef<HTMLVideoElement>(null)
    const {videoStore} = useGlobalStore()
    
    

    // Play Video
    useEffect(() => {
        if (videoEl.current) {
            videoEl.current.onloadedmetadata = function(e) {
                videoEl.current!.play();
              };
        }
    }, [videoEl])


    useEffect(() => {
        videoStore.getMedia().then(() => {
            if (videoEl.current && videoStore.initalized) {
                videoEl.current.srcObject = videoStore.instance.media
            }
        })
    },[videoStore])

    return (
        <video ref={videoEl}></video>
    );
}