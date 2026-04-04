import { LazyStore } from '@tauri-apps/plugin-store';

const dataStore = new LazyStore('settings.json');

export default dataStore;
