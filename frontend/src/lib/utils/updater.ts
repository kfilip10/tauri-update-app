import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { getVersion } from '@tauri-apps/api/app';

// Get the current app version
export async function getCurrentVersion(): Promise<string> {
  try {
    return await getVersion();
  } catch (error) {
    console.error('Failed to get app version:', error);
    return 'unknown';
  }
}

// Check for updates with version comparison
export async function checkForAppUpdates(onUserClick: boolean = false) {
  try {
    // Get current version for display
    const currentVersion = await getCurrentVersion();
    console.log(`Current app version: ${currentVersion}`);
    
    const update = await check();
    console.log('Update check result:', update);
    
    // Handle the case when no update is available
    if (update === null) {
      if (onUserClick) {
        await message(`You are on the latest version (${currentVersion}).`, { 
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
        `Update ${update.version} is available!\n\nCurrent version: ${currentVersion}\n\n${
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
          await message('Update downloaded. The application will restart now.', {
            title: 'Update Ready',
            kind: 'info',
            okLabel: 'OK'
          });
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
      await message(`You are on the latest version (${currentVersion}). Stay awesome!`, { 
        title: 'No Update Available',
        kind: 'info',
        okLabel: 'OK'
      });
    }
  } catch (error) {
    console.error('Update check failed:', error);
    if (onUserClick) {
      const currentVersion = await getCurrentVersion().catch(() => 'unknown');
      await message(`Failed to check for updates: ${error}\nCurrent version: ${currentVersion}`, { 
        title: 'Error',
        kind: 'error',
        okLabel: 'OK'
      });
    }
  }
}

// Add a debug function to display version info
export async function displayVersionInfo() {
  try {
    const currentVersion = await getCurrentVersion();
    const update = await check().catch(() => null);
    
    let message = `Current version: ${currentVersion}\n`;
    
    if (update) {
      message += `Latest available version: ${update.version || 'unknown'}\n`;
      message += `Update available: ${update.available ? 'Yes' : 'No'}\n`;
      if (update.body) message += `Release notes: ${update.body}\n`;
    } else {
      message += "Couldn't retrieve update information.";
    }
    
    return message;
  } catch (error) {
    console.error('Failed to get version info:', error);
    return `Error getting version info: ${error}`;
  }
}