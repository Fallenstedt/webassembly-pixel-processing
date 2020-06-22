import React from 'react';
import { MediaEngine } from "./MediaEngine";
import { config } from '../utils/config';
import { WasmEngine } from './WasmEngine';


const globalContext = React.createContext({
    mediaEngine: new MediaEngine({
        audio: false,
        video: { width: config.video.width, height: config.video.height }
      }),
    wasmEngine: new WasmEngine()
});

export const useEngines = () => React.useContext(globalContext);

