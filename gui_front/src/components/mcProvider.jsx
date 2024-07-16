import React, { createContext, useContext, useEffect, useState } from 'react';

const DataContext = createContext();

export const DataProvider = ({ children }) => {
    const [serverStatus, setServerStatus] = useState('Unknown');
    const [cpuUsage, setCpuUsage] = useState(0);
    const [ramUsage, setRamUsage] = useState(0);
    const [logs, setLogs] = useState([]);
    const [numPlayers, setNumPlayers] = useState(0);
    const [players, setPlayers] = useState([]);
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
                console.log(data);
                setServerStatus(data.status);
                setCpuUsage(parseFloat((data.cpu.total_usage/data.cpu.system_cpu_usage) * data.cpu.online_cpus * 100).toFixed(2));
                setRamUsage(parseFloat((data.memory.usage/data.memory.limit) * 100).toFixed(2));
            });
    };

    const fetchPlayers = () => {
        fetch('/api/query/full')
            .then(response => response.json())
            .then(data => {
                console.log(data);
                setNumPlayers(data.num_players);
                setPlayers(data.players);
            })
            .catch(error => {
                console.error('Error fetching player data:', error);
                setError('Error fetching player data');
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
        let interval;

        if (serverStatus === 'running') {
            interval = setInterval(fetchPlayers, 4000);
        } else {
            setNumPlayers(0);
            setPlayers([]);
            clearInterval(interval);
        }

        eventSource.onmessage = (event) => {
            setLogs((prevLogs) => [...prevLogs, event.data]);
        };

        eventSource.onerror = (error) => {
            console.error('EventSource error:', error);
            eventSource.close();
            clearInterval(interval);
        };

        return () => {
            eventSource.close();
        }
    }, [serverStatus]);

    return (
        <DataContext.Provider value={{ serverStatus, cpuUsage, ramUsage, logs, numPlayers, players, error }}>
            {children}
        </DataContext.Provider>
    );
};

// Custom hook to use the data context
export const useData = () => {
    return useContext(DataContext);
};