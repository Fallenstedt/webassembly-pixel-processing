import React from 'react';
import { VideoManager } from "./VideoManager";
import { config } from '../utils/config';


const globalContext = React.createContext({
    videoStore: new VideoManager({
        audio: false,
        video: { width: config.video.width, height: config.video.height }
      }),
});
export const useGlobalStore = () => React.useContext(globalContext);

