import React, { useEffect, useState } from 'react';
import { Grid, Paper, Typography, Button, Container } from '@mui/material';

function HomePage() {
    const [serverStatus, setServerStatus] = useState('Unknown');
    const [cpuUsage, setCpuUsage] = useState(0);
    const [ramUsage, setRamUsage] = useState(0);
    const [apiPort, setApiPort] = useState(null);

    useEffect(() => {
        fetch('/api/status')
            .then(response => response.json())
            .then(data => setServerStatus(data.status));

        fetch('/api/cpu')
            .then(response => response.json())
            .then(data => setCpuUsage(parseFloat(data.cpu).toFixed(2)));

        fetch('/api/ram')
            .then(response => response.json())
            .then(data => setRamUsage(parseFloat(data.ram).toFixed(2)));
    }, []);

    useEffect(() => {
        fetch('http://localhost:8080/api/port')
            .then(response => response.json())
            .then(data => setApiPort(data.port))
            .catch(error => console.error("Error fetching port: ", error));
    }, []);

    return (
        <Container maxWidth="lg">
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
