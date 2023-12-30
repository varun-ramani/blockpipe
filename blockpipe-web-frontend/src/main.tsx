import { MantineProvider, createTheme } from '@mantine/core';
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

import '@fontsource/iosevka';
import '@fontsource/iosevka-curly';
import '@mantine/core/styles.css';

// Render the App component inside the root div
const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);

// create a theme
const mantineTheme = createTheme({
  fontFamily: "Iosevka Curly",
})

root.render(
  <React.StrictMode>
    <MantineProvider theme={mantineTheme} defaultColorScheme='light'>
      <App />
    </MantineProvider>
  </React.StrictMode>,
);
