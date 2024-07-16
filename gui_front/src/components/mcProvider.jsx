import React, { createContext, useContext, useEffect, useState } from 'react';

const DataContext = createContext();

export const DataProvider = ({ children }) => {
    const [serverStatus, setServerStatus] = useState('Unknown');
    const [cpuUsage, setCpuUsage] = useState(0);
    const [ramUsage, setRamUsage] = useState(0);
    const [logs, setLogs] = useState([]);
    const [error, setError] = useState(null);

    const fetchData = () => {
        fetch('/api/stats')
            .then(response => response.json())
            .catch(() => {
                setServerStatus('unknown');
                setCpuUsage(0);
                setRamUsage(0);
            })
            .then(data => {
                setServerStatus(data.status);
                setCpuUsage(parseFloat((data.cpu.total_usage/data.cpu.system_cpu_usage) * data.cpu.online_cpus * 100).toFixed(2));
                setRamUsage(parseFloat((data.memory.usage/data.memory.limit) * 100).toFixed(2));
            });
    };

    useEffect(() => {
        fetchData();
        const interval = setInterval(fetchData, 4000);

        return () => {
            clearInterval(interval);
        };
    }, []);

    useEffect(() => {
        const eventSource = new EventSource('/api/logs');

        eventSource.onmessage = (event) => {
            setLogs((prevLogs) => [...prevLogs, event.data]);
        };

        eventSource.onerror = (error) => {
            console.error('EventSource error:', error);
            eventSource.close();
        };

        return () => {
            eventSource.close();
        }
    }, [serverStatus]);

    return (
        <DataContext.Provider value={{ serverStatus, cpuUsage, ramUsage, logs, error }}>
            {children}
        </DataContext.Provider>
    );
};

// Custom hook to use the data context
export const useData = () => {
    return useContext(DataContext);
};