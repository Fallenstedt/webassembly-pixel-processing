import React, { useEffect, useRef, useCallback } from 'react';
import Stats from 'stats.js';
import { useEngines } from '../stores/use_engines';
import { config } from '../utils/config';

export function Video() {
    const canvasRef = useRef<HTMLCanvasElement>(null)
    const { mediaEngine, wasmEngine } = useEngines()


    useEffect(() => {
        // Render each frame to a canvas element for Rust to see
        mediaEngine.getMedia().then((data) => {
            if (mediaEngine.initalized && mediaEngine.instance.media && canvasRef.current !== null) {
                const renderingEngine = new wasmEngine.instance!.RenderingEngine();
                renderingEngine.add_target_canvas(canvasRef.current);
                renderingEngine.start(mediaEngine.instance.media);
            }
        })

    }, [canvasRef, mediaEngine])

    return (
        <div>
            <canvas ref={canvasRef} width="640" height="360"></canvas>
        </div>
    );
}