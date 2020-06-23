import React, { useEffect, useRef, useCallback } from 'react';
import Stats from 'stats.js';
import { useEngines } from '../stores/use_engines';
import { config } from '../utils/config';

export function Video() {
    const videoRef = useRef<HTMLVideoElement>(null)
    const { mediaEngine, wasmEngine } = useEngines()


    useEffect(() => {
        // Play Video
        if (videoRef.current) {
            videoRef.current.onloadedmetadata = function (e) {
                videoRef.current!.play();
            };
        }

        // Render each frame to a canvas element for Rust to see
        mediaEngine.getMedia().then(() => {
            if (videoRef.current && mediaEngine.initalized) {
                videoRef.current!.srcObject = mediaEngine.instance.media;
                /**TODO get video resolution dimensions instead of hardcoded values */
                const wasm = new wasmEngine.instance!.WebLightShow("video_element", config.video.width, config.video.height);
                wasm.test();
                wasm.run();
            }
        })

    }, [videoRef, mediaEngine])

    return (
        <div>
            <video id="video_element" ref={videoRef}></video>
        </div>
    );
}