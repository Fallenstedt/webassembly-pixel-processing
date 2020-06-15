import React from 'react';
import { VideoManager } from "./VideoManager";


const globalContext = React.createContext({
    videoStore: new VideoManager({
        audio: true,
        video: { width: 1280, height: 720 }
      }),
});
export const useGlobalStore = () => React.useContext(globalContext);

