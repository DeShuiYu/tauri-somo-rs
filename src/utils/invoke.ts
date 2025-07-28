import { invoke } from "@tauri-apps/api/core";


export async function get_network_address_info() {
    return await invoke("command_network_info", { name });
}

