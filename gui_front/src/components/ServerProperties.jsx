import React, { useState } from 'react';
import './serverProperties.scss';
import { useData } from './mcProvider';
import { TextField, Switch } from '@mui/material';

const ServerProperties = () => {
    const { sendProperties } = useData();

    const [properties, setProperties] = useState({
        "allow-flight": false,
        "allow-nether": true,
        "broadcast-console-to-ops": true,
        "broadcast-rcon-to-ops": true,
        "difficulty": "easy",
        "enable-command-block": false,
        "force-gamemode": false,
        "gamemode": "survival",
        "hardcore": false,
        "hide-online-players": false,
        "max-players": 20,
        "motd": "A Minecraft Server",
        "online-mode": true,
        "op-permission-level": 4,
        "player-idle-timeout": 0,
        "pvp": true,
        "resource-pack": "",
        "resource-pack-prompt": "",
        "simulation-distance": 10,
        "spawn-animals": true,
        "spawn-monsters": true,
        "spawn-npcs": true,
        "spawn-protection": 16,
        "view-distance": 10,
        "white-list": false
    });

    const handleChange = (e) => {
        const { name, value, type, checked } = e.target;
        setProperties({
            ...properties,
            [name]: type === "checkbox" ? checked : value
        });
    };

    const handleSubmit = (e) => {
        e.preventDefault();
        sendProperties(properties);
    };

    return (
        <div className="server-properties">
            <h1>Propriétés du Serveur</h1>
            <form onSubmit={handleSubmit}>
                {Object.keys(properties).map((key) => (
                    <div key={key} className="property">
                        {typeof properties[key] === "boolean" ? (
                            <>
                                <label htmlFor={key}>{key.replace(/-/g, ' ')}</label>
                                <input
                                    type="checkbox"
                                    name={key}
                                    checked={properties[key]}
                                    onChange={handleChange}
                                />
                            </>
                        ) : typeof properties[key] === "number" ? (
                            <TextField
                                name={key}
                                value={properties[key]}
                                label={key.replace(/-/g, ' ')}
                                variant='outlined'
                                type="number"
                                onChange={handleChange}
                            />
                        ) : (
                            <TextField
                                name={key}
                                value={properties[key]}
                                label={key.replace(/-/g, ' ')}
                                variant='outlined'
                                onChange={handleChange}
                            />
                        )}
                    </div>
                ))}
                <button type="submit">Enregistrer</button>
            </form>
        </div>
    );
}

export default ServerProperties;
