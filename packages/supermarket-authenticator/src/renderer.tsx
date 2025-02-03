import { createRoot } from 'react-dom/client';

import { App } from '~/app';
import '~assets/css/index.css';

const root = createRoot(document.body);
root.render(<App />);
