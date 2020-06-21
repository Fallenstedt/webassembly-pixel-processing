import React, { useEffect, useRef, useCallback } from 'react';
import Stats from 'stats.js';
import { useEngines } from '../stores/use_engines';
import { config } from '../utils/config';

export function Video() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const videoRef = useRef<HTMLVideoElement>(null)
    const statsRef = useRef<HTMLSpanElement>(null)
    const {mediaEngine, wasmEngine } = useEngines()
    const stats = new Stats();
    stats.showPanel( 0 ); // 0: fps, 1: ms, 2: mb, 3+: custom


    const renderImageToCanvas = useCallback(() => {
        stats.begin()
        handleVideoFrameOnCanvas();
        stats.end()
        window.requestAnimationFrame(renderImageToCanvas)
    }, [stats])


    function handleVideoFrameOnCanvas() {
        const ctx = canvasRef.current!.getContext("2d")!
        ctx.drawImage(videoRef.current!, 0, 0, (config.video.width), (config.video.height));
        wasmEngine.instance!.on_animation_frame();
    }

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
        mediaEngine.getMedia().then(() => {
            if (videoRef.current && canvasRef.current && mediaEngine.initalized) {
                videoRef.current!.srcObject = mediaEngine.instance.media;
                renderImageToCanvas()
            }
        })

    }, [videoRef, statsRef, renderImageToCanvas, stats.dom, mediaEngine])
    
    return (
        <div>
            <span ref={statsRef}></span>
            <canvas id="canvas_element" width={config.video.width} height={config.video.height} ref={canvasRef}></canvas>
            <video style={{display: 'none'}} ref={videoRef}></video>
        </div>
    );
}