import React, { createContext, useContext, useEffect, useState, useRef } from 'react';

const DataContext = createContext();

export const DataProvider = ({ children }) => {
    const [serverStatus, setServerStatus] = useState('Unknown');
    const [cpuUsage, setCpuUsage] = useState(0);
    const [ramUsage, setRamUsage] = useState(0);
    const [logs, setLogs] = useState([]);
    const [numPlayers, setNumPlayers] = useState(0);
    const [maxPlayers, setMaxPlayers] = useState(0);
    const [players, setPlayers] = useState([]);
    const [error, setError] = useState(null);

    const ws = useRef(null);

    useEffect(() => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            return;
        }
        ws.current = new WebSocket('ws://localhost:8080/ws/');

        ws.current.onopen = () => {
            console.log('WebSocket connection established');
        };

        ws.current.onmessage = (event) => {
            const message = JSON.parse(event.data);
            handleWebSocketMessage(message);
        };

        ws.current.onerror = (error) => {
            console.error('WebSocket error:', error);
            setError('WebSocket error');
        };

        ws.current.onpong = () => {
            console.log('Received pong');
        };

        ws.current.onclose = (event) => {
            if (event.wasClean) {
                console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
            } else {
                console.log(`[close] Connection died code=${event.code}, reason=${event.reason}`);
            }
        };

        const handleWebSocketMessage = (message) => {
            if (message.status !== 'ok') {
                setError(message.content.error);

                if (message.content_type === 'docker_stats') {
                    console.log('docker_stats:', message.content);
                    setServerStatus(message.content.status);
                }

                return;
            }

            switch (message.content_type) {
                case 'docker_stats':
                    console.log('docker_stats:', message.content);

                    setServerStatus(message.content.status);
                    setCpuUsage(parseFloat((message.content.cpu.total_usage / message.content.cpu.system_cpu_usage) * message.content.cpu.online_cpus * 100).toFixed(2));
                    setRamUsage(parseFloat((message.content.memory.usage / message.content.memory.limit) * 100).toFixed(2));
                    break;
                case 'server_stats':
                    console.log('server_stats:', message.content);
                    setNumPlayers(message.content.num_players);
                    setPlayers(message.content.players);
                    setMaxPlayers(message.content.max_players);
                    break;
                case 'log':
                    setLogs((prevLogs) => [...prevLogs, message.content]);
                    break;
                case 'error':
                    setError(message.content.error);
                    break;
                default:
                    console.log('Unknown message type:', message.content_type);
            }
        };

        return () => {
            if (ws.current) {
                ws.current.close();
                ws.current = null;
            }
        };
    }, []);

    const sendCommand = (command) => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            const message = JSON.stringify({ 
                status: "sending",
                content: command,
                content_type: "console",
             });
            ws.current.send(message);
        } else {
            console.error('WebSocket is not open');
        }
    };

    const serverToggle = () => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            const message = JSON.stringify({ 
                status: "sending",
                content: 'toggle',
                content_type: "command",
             });
            ws.current.send(message);
        } else {
            console.error('WebSocket is not open');
        }
    }

    const sendProperties = (properties) => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            const message = JSON.stringify({ 
                status: "sending",
                content: properties,
                content_type: "properties",
             });
            ws.current.send(message);
        } else {
            console.error('WebSocket is not open');
        }
    };

    return (
        <DataContext.Provider value={
            { 
                serverStatus, 
                cpuUsage, ramUsage, 
                logs, 
                numPlayers, maxPlayers, players, 
                error, 
                sendCommand, serverToggle, sendProperties
            }}>
            {children}
        </DataContext.Provider>
    );
};

// Custom hook to use the data context
export const useData = () => {
    return useContext(DataContext);
};