import { useState, useEffect } from "preact/hooks";

import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { FunctionComponent } from "preact";

interface MonitorInfo {
    id: number;
    model: string;
    inputs: string[];
}

type IndexedMonitorInfo = { [id: number]: MonitorInfo };

interface MonitorShortcutProps {
    shortcut: string[];
}

const MonitorShortcut = () => {};

interface MonitorInfoListProps {
    monitor: MonitorInfo;
    toggled: boolean;
    onToggle: (id: number, toggled: boolean) => void;
}

const MonitorInfoList: FunctionComponent<MonitorInfoListProps> = ({
    monitor,
    toggled,
    onToggle,
}) => {
    return (
        <div className="monitor">
            <div
                className="monitor__header"
                onClick={() => onToggle(monitor.id, !toggled)}
            >
                {monitor.id + 1}. {monitor.model}
            </div>
            <div
                className="monitor__info"
                style={{ display: toggled ? "block" : "none" }}
            >
                {monitor.inputs.map((input) => (
                    <div className="monitor__info__input">
                        <div
                            onClick={() => {
                                invoke("switch_monitor_input", {
                                    monitorIdx: monitor.id,
                                    input,
                                });
                            }}
                            className="monitor__info__input__name"
                        >
                            {input}
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

interface MonitorListProps {
    monitors: IndexedMonitorInfo;
}

const MonitorList: FunctionComponent<MonitorListProps> = ({ monitors }) => {
    const [toggledMonitors, setToggledMonitors] = useState<{
        [key: number]: boolean;
    }>({});

    useEffect(() => {
        const toggledIds = Object.values(monitors).reduce(
            (ids: { [key: number]: boolean }, monitor) => {
                ids[monitor.id] = false;
                return ids;
            },
            {}
        );

        setToggledMonitors(toggledIds);
    }, []);

    return (
        <div className="monitor-list">
            {Object.values(monitors).map((monitor) => {
                return (
                    <MonitorInfoList
                        key={monitor.id}
                        monitor={monitor}
                        toggled={toggledMonitors[monitor.id]}
                        onToggle={(id, toggled) => {
                            setToggledMonitors({
                                ...toggledMonitors,
                                [id]: toggled,
                            });
                        }}
                    />
                );
            })}
        </div>
    );
};

export function App() {
    const [monitors, setMonitors] = useState<IndexedMonitorInfo>({});

    useEffect(() => {
        invoke("refresh_monitor_info");
    }, []);

    useEffect(() => {
        async function parseMonitorInfoEvent() {
            const listener = await listen<MonitorInfo[]>(
                "monitor-info",
                (event) => {
                    const monitors = event.payload.reduce(
                        (monitors: IndexedMonitorInfo, monitor) => {
                            monitors[monitor.id] = monitor;
                            return monitors;
                        },
                        {}
                    );
                    setMonitors(monitors);
                }
            );

            return () => listener();
        }

        parseMonitorInfoEvent();
    }, []);

    return (
        <div className="app">
            <div className="app__header">
                <div className="app__header__title">Current Displays</div>
                <div
                    className="app__header__refresh"
                    onClick={() => {
                        setMonitors({});
                        invoke("refresh_monitor_info");
                    }}
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        className="h-6 w-6"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        strokeWidth={2}
                    >
                        <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                        />
                    </svg>
                </div>
            </div>
            <div className="app__content">
                <MonitorList monitors={monitors} />
            </div>
        </div>
    );
}
