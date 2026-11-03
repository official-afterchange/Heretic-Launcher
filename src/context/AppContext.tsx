import { createContext, useCallback, useContext, useEffect, useState, ReactNode } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface Config {
    language: string;
    ram_mb: number;
    java_path: string | null;
    close_on_launch: boolean;
    selected_instance: string | null;
    player_username: string;
}

export interface Instance {
  id: string;
  name: string;
  minecraft_version: string;
  loader: string;
  loader_version: string | null;
  server_address: string | null;
  installed: boolean;
}

interface AppState {
  config: Config | null;
  instance: Instance | null;
  instances: Instance[];
  loading: boolean;
  updateConfig: (patch: Partial<Config>) => Promise<void>;
  selectInstance: (id: string) => Promise<void>;
  refresh: () => Promise<void>;
}

const AppContext = createContext<AppState | null>(null);

export function AppProvider({ children }: { children: ReactNode }) {
    const [config, setConfig] = useState<Config | null>(null);
    const [instance, setInstance] = useState<Instance | null>(null);
    const [instances, setInstances] = useState<Instance[]>([]);
    const [loading, setLoading] = useState(true);

    const refresh = useCallback(async () => {
        try {
            const [cfg, selected, all] = await Promise.all([
                invoke<Config>("get_config"),
                invoke<Instance>("get_selected_instance"),
                invoke<Instance[]>("list_instances"),
            ]);
            setConfig(cfg);
            setInstance(selected);
            setInstances(all);
        } catch (e) {
            console.error("Chargement de la config impossible :", e);
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        refresh();
    }, [refresh]);

    const updateConfig = useCallback(
        async (patch: Partial<Config>) => {
        if (!config) return;
        const next = { ...config, ...patch };
        await invoke("set_config", { newConfig: next });
        setConfig(next);
        },
        [config]
    );

    const selectInstance = useCallback(
        async (id: string) => {
        await invoke("select_instance", { id });
        await refresh(); // recharge la config (selected_instance) et l'instance
        },
        [refresh]
    );

    return (
        <AppContext.Provider
        value={{ config, instance, instances, loading, updateConfig, selectInstance, refresh }}
        >
        {children}
        </AppContext.Provider>
    );
}

export function useApp() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useApp doit être utilisé dans <AppProvider>");
  return ctx;
}