import React, {useState, useEffect} from 'react';
import { useLocation, Link } from 'react-router-dom';
import { 
    Menu as MenuIcon,
    MenuOpen as MenuOpenIcon,
    Dns as ServerIcon,
    Settings as SettingsIcon,
} from '@mui/icons-material';
import './menu.scss';
import { useData } from '../mcProvider';

const Menu = () => {
    const { serverStatus, numPlayers, maxPlayers } = useData();
    const [menuOpen, setMenuOpen] = useState(false);
    const currentPath = useLocation();

    const MenuItem = [
        {'name': 'Server', 'icon': ServerIcon, 'path': '/'},
        {'name': 'Settings', 'icon': SettingsIcon, 'path': '/settings'},
        {'name': 'Server', 'icon': ServerIcon, 'path': '/server'},
        {'name': 'Server', 'icon': ServerIcon, 'path': '/server'},
    ];

    const getServerStatus = (status) => {
        switch(status) {
            case 'starting':
                return <>Server status: <br/><span className='warning'>starting</span></>;
            case 'running':
                return <>Server status: <br/><span className='ok'>running</span></>;
            case 'stopping':
                return <>Server status: <br/><span className='warning'>stopping</span></>;
            case 'stopped':
                return <>Server status: <br/><span className='error'>stopped</span></>;
            default:
                return <>Server status: <br/><span className='unknown'>unknown</span></>;
        }
    }

    return (
        <div className={['menu', menuOpen ? 'open' : 'closed'].join(' ')}>
            <div className='menu-div'>
                {menuOpen 
                    ? <MenuOpenIcon onClick={() => setMenuOpen(false)} className='icon' fontSize='large'/> 
                    : <MenuIcon onClick={() => setMenuOpen(true)} className='icon' fontSize='large'/>}
            </div>
            <div className={'server-status ' + serverStatus}>
                <div style={{display: "flex", gap: "10px"}}>
                    <div className='status-icon'></div>
                    {
                        menuOpen && <p>{getServerStatus(serverStatus)}</p>
                    }
                </div>
                {
                    serverStatus === 'running' && <p>{numPlayers}/{maxPlayers}</p>
                }
            </div>
            <ul>
                {
                    MenuItem.map((item, index) => (
                        <li key={index} className={currentPath.pathname === item.path ? 'current' : null}>
                            <Link to={item.path}>
                                <item.icon className='icon' fontSize='large'/>
                                {menuOpen && <p>{item.name}</p>}
                            </Link>
                        </li>
                    ))
                }
            </ul>
        </div>
    )
} 

export default Menu