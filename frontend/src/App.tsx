import { useState, useEffect } from 'react';
import { appWindow } from '@tauri-apps/api/window';
import SpotlightModal from './components/SpotlightModal';
import MainApp from './MainApp'; // We'll create this component

const App = () => {
  const [isSpotlight, setIsSpotlight] = useState(false);

  useEffect(() => {
    // Determine if this window is the spotlight
    if (appWindow.label === 'spotlight') {
      setIsSpotlight(true);
    }
  }, []);

  if (isSpotlight) {
    return <SpotlightModal />;
  }

  return <MainApp />;
};

export default App;

