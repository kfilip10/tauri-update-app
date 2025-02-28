import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

export async function checkForAppUpdates(onUserClick: boolean = false) {
  try {
    const update = await check();
    
    // Handle the case when no update is available
    if (update === null) {
      if (onUserClick) {
        await message('You are on the latest version or no updates available.', { 
          title: 'No Update Available',
          kind: 'info',
          okLabel: 'OK'
        });
      }
      return;
    }
    
    // Check if the update object has the necessary fields
    if (update?.available && update?.version) {
      const yes = await ask(
        `Update ${update.version || 'new version'} is available!\n\n${
          update.body || 'Release notes not available.'
        }`, { 
          title: 'Update Available',
          kind: 'info',
          okLabel: 'Update',
          cancelLabel: 'Cancel'
        }
      );
      
      if (yes) {
        try {
          await update.downloadAndInstall();
          await invoke("graceful_restart");
        } catch (error) {
          console.error('Update failed:', error);
          await message(`Update failed: ${error}`, { 
            title: 'Update Error',
            kind: 'error',
            okLabel: 'OK'
          });
        }
      }
    } else if (onUserClick) {
      await message('You are on the latest version. Stay awesome!', { 
        title: 'No Update Available',
        kind: 'info',
        okLabel: 'OK'
      });
    }
  } catch (error) {
    console.error('Update check failed:', error);
    if (onUserClick) {
      await message(`Failed to check for updates: ${error}`, { 
        title: 'Error',
        kind: 'error',
        okLabel: 'OK'
      });
    }
  }
}
