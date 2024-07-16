import React, { useEffect, useState } from 'react';
import { Grid, Paper, Typography, Button, Container, Input, TextField } from '@mui/material';
import { useData } from './mcProvider';
import './homePage.scss';

function HomePage() {
    const { serverStatus, cpuUsage, ramUsage, logs, sendCommand, serverToggle } = useData();
    const [command, setCommand] = useState('');

    const writelog = (logs) => {
        return logs.map((log, index) => <p style={{"margin": "unset"}} key={index}>{
            log
        }</p>);
    }

    const handleInputChange = (event) => {
        setCommand(event.target.value);
    };

    const handleServerStart = async () => {
        serverToggle();
    }

    const handleSubmit = async (event) => {
        event.preventDefault();
        try {
            let _command = command.trim();
            setCommand('');
            sendCommand(_command);
        } catch (error) {
            console.error('Error:', error);
        }
    };

    return (
        <Container maxWidth="xl">
            {/* <Menu/> */}
            <Grid container spacing={3} style={{ marginTop: '20px' }}>
                <Grid item xs={12} sm={4}>
                    <Paper style={{ padding: '20px', textAlign: 'center' }}>
                        <Typography variant="h6">Server status + On/off</Typography>
                        <Typography variant="h4">{serverStatus}</Typography>
                        <Button variant="contained" color="primary" style={{ marginTop: '20px' }} onClick={() => handleServerStart()}>
                            {serverStatus !== 'stopped' ? 'Stop' : 'Start'}
                        </Button>
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
                <Grid item xs={12} className='console-container'>
                    <Paper className='console'>
                        <Typography variant="h6" style={{ display: 'flex', flexDirection: 'column', alignItems: 'start'}}>
                            {
                                logs.length > 0 ? writelog(logs) : 'No logs yet'
                            }
                        </Typography>
                    </Paper>
                    <Paper className='console-input'>
                        <form onSubmit={handleSubmit}>
                                <TextField 
                                    fullWidth 
                                    placeholder="Enter command" 
                                    value={command}
                                    onChange={handleInputChange}
                                />
                                <Button type="submit" variant="contained">Send</Button>
                        </form>
                    </Paper>
                </Grid>
            </Grid>
        </Container>
    );
}

export default HomePage;