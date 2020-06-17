import React, { useEffect, useRef, useCallback } from 'react';
import Stats from 'stats.js';
import { useGlobalStore } from '../stores/use_video_manager';
import { config } from '../utils/config';
// import * as wasm from 'converter'


export function Video() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const videoRef = useRef<HTMLVideoElement>(null)
    const statsRef = useRef<HTMLSpanElement>(null)
    const {videoStore} = useGlobalStore()
    const stats = new Stats();
    stats.showPanel( 0 ); // 0: fps, 1: ms, 2: mb, 3+: custom


    const renderImageToCanvas = useCallback(() => {
        stats.begin()
        canvasRef.current!.getContext("2d")!.drawImage(videoRef.current!, 0, 0, (config.video.width), (config.video.height));
        stats.end()
        window.requestAnimationFrame(renderImageToCanvas)
    }, [stats])

    useEffect(() => {

        // Set Stats
        if (statsRef.current) {
            statsRef.current.appendChild(stats.dom)
        }

        // Play Video
        if (videoRef.current) {
            videoRef.current.onloadedmetadata = function(e) {
                videoRef.current!.play();
              };
        }

        // Render each frame to a canvas element for Rust to see
        videoStore.getMedia().then(() => {
            if (videoRef.current && canvasRef.current && videoStore.initalized) {
                videoRef.current!.srcObject = videoStore.instance.media;
                // wasm render go here
                renderImageToCanvas()
            }
        })

    }, [videoRef, statsRef, renderImageToCanvas, stats.dom, videoStore])


    return (
        <div>
            <span ref={statsRef}></span>
            <canvas id="canvas_element" width={config.video.width} height={config.video.height} ref={canvasRef}></canvas>
            <video style={{display: 'none'}} ref={videoRef}></video>
        </div>
    );
}