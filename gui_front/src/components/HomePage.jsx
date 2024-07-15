import React, { useEffect, useState } from 'react';
import { Grid, Paper, Typography, Button, Container } from '@mui/material';
import Menu from './menu';

function HomePage() {
    const [serverStatus, setServerStatus] = useState('Unknown');
    const [cpuUsage, setCpuUsage] = useState(0);
    const [ramUsage, setRamUsage] = useState(0);

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
        return () => clearInterval(interval);
    }, []);

    return (
        <Container maxWidth="lg">
            {/* <Menu/> */}
            <Grid container spacing={3} style={{ marginTop: '20px' }}>
                <Grid item xs={12} sm={4}>
                    <Paper style={{ padding: '20px', textAlign: 'center' }}>
                        <Typography variant="h6">Server status + On/off</Typography>
                        <Typography variant="h4">{serverStatus}</Typography>
                        <Button variant="contained" color="primary" style={{ marginTop: '20px' }}>Toggle</Button>
                    </Paper>
                </Grid>
                <Grid item xs={12} sm={4}>
                    <Paper style={{ padding: '20px', textAlign: 'center' }}>
                        <Typography variant="h6">CPU %</Typography>
                        <Typography variant="h4">{cpuUsage}%</Typography>
                    </Paper>
                </Grid>
                <Grid item xs={12} sm={4}>
                    <Paper style={{ padding: '20px', textAlign: 'center' }}>
                        <Typography variant="h6">RAM %</Typography>
                        <Typography variant="h4">{ramUsage}%</Typography>
                    </Paper>
                </Grid>
                <Grid item xs={12}>
                    <Paper style={{ padding: '20px', textAlign: 'center' }}>
                        <Typography variant="h6">Console</Typography>
                    </Paper>
                </Grid>
            </Grid>
        </Container>
    );
}

export default HomePage;