import { createApp } from "vue";
import App from "./App.vue";

// import { onOpenUrl } from '@tauri-apps/plugin-deep-link';

// await onOpenUrl((urls) => {
//     console.log('deep link:', urls);
// });

import { tray } from './tray';

tray();


createApp(App).mount("#app");


