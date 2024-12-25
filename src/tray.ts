
import { TrayIcon } from '@tauri-apps/api/tray';
import { Menu } from '@tauri-apps/api/menu';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { exit } from '@tauri-apps/plugin-process';
import { confirm } from '@tauri-apps/plugin-dialog';


export async function tray() {
    const menu = await Menu.new({
        items: [
            {
                id: 'quit',
                text: 'Quit',
                action: () => {
                    console.log('quit pressed');
                    // Creates a confirmation Ok/Cancel dialog
                    confirm(
                        'Are you sure you want to quit?',
                        { title: 'KubeUI', kind: 'warning' }
                    ).then((response) => {
                        if (response === true) {
                            exit(0);
                        }
                    });
    
    
                },
            },
        ],
    });
    
    const icon = await defaultWindowIcon();
    const options = {
        menu,
        menuOnLeftClick: true,
        icon: icon ?? 'src/assets/icons/tauri.ico', // Provide a fallback value for the icon
    };

    return TrayIcon.new(options);
}