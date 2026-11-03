import { useEffect, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";

export interface ServerStatus {
  online: boolean;
  players_online: number;
  players_max: number;
  ping_ms: number;
}

export function useServerStatus(address: string | null | undefined, intervalMs = 30_000) {
  const [status, setStatus] = useState<ServerStatus | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!address || !isTauri()) {
      setStatus(null);
      setLoading(false);
      return;
    }

    let cancelled = false;
    setLoading(true);

    const fetchStatus = async () => {
      try {
        const res = await invoke<ServerStatus>("get_server_status", { address });
        if (!cancelled) setStatus(res);
      } catch (e) {
        console.error(e);
        if (!cancelled) setStatus(null);
      } finally {
        if (!cancelled) setLoading(false);
      }
    };

    fetchStatus();
    const id = setInterval(fetchStatus, intervalMs);

    return () => {
      cancelled = true;
      clearInterval(id);
    };
  }, [address, intervalMs]);

  return { status, loading };
}