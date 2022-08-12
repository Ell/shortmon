import { useState, useEffect } from "preact/hooks";

import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

invoke("refresh_monitor_info");

await listen("monitor-info", (event) => {
    console.log("monitor-info", event.payload);
});

invoke("refresh_monitor_info");

interface MonitorInfo {
    id: number;
    model: string;
    inputs: string[];
}

type IndexedMonitorInfo = { [id: number]: MonitorInfo };

function renderMonitors(monitors: IndexedMonitorInfo) {
    return Object.values(monitors).map((monitor) => {
        return (
            <div key={monitor.id}>
                <div>{monitor.model}</div>
                {monitor.inputs.map((input) => (
                    <button
                        key={input}
                        onClick={() => {
                            console.log("invoking");
                            invoke("switch_monitor_input", {
                                monitorIdx: monitor.id,
                                input,
                            });
                        }}
                    >
                        {input}
                    </button>
                ))}
            </div>
        );
    });
}

export function App() {
    const [monitors, setMonitors] = useState<IndexedMonitorInfo>({});

    useEffect(() => {
        async function parseMonitorInfoEvent() {
            await listen<MonitorInfo[]>("monitor-info", (event) => {
                const monitors = event.payload.reduce(
                    (monitors: IndexedMonitorInfo, monitor) => {
                        monitors[monitor.id] = monitor;
                        return monitors;
                    },
                    {}
                );
                setMonitors(monitors);
            });
        }

        parseMonitorInfoEvent();
    }, []);

    return <>{renderMonitors(monitors)}</>;
}
