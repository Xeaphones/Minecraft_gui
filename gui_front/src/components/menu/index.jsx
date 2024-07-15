import React, {useState, useEffect} from 'react';
import { 
    Menu as MenuIcon,
    MenuOpen as MenuOpenIcon,
    Dns as ServerIcon,
    Settings as SettingsIcon,
} from '@mui/icons-material';
import './menu.scss';

const Menu = () => {
    const [currentPath, setCurrentPath] = useState('/');
    const [serverStatus, setServerStatus] = useState('Unknown');
    const [menuOpen, setMenuOpen] = useState(false);

    const MenuItem = [
        {'name': 'Server', 'icon': ServerIcon, 'path': '/'},
        {'name': 'Settings', 'icon': SettingsIcon, 'path': '/settings'},
        {'name': 'Server', 'icon': ServerIcon, 'path': '/server'},
        {'name': 'Server', 'icon': ServerIcon, 'path': '/server'},
    ];

    const getServerStatus = (status) => {
        switch(status) {
            case 'running':
                return <>Server status: <br/><span className='ok'>running</span></>;
            case 'stopped':
                return <>Server status: <br/><span className='error'>stopped</span></>;
            default:
                return <>Server status: <br/><span className='error'>unknown</span></>;
        }
    }

    const fetchData = () => {
        fetch('/api/stats')
            .then(response => response.json())
            .catch(() => setServerStatus('Unknown'))
            .then(data => {
                console.log(data);
                setServerStatus(data.status);
            });
    };

    useEffect(() => {
        fetchData();
        const interval = setInterval(fetchData, 4000);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className={['menu', menuOpen ? 'open' : 'closed'].join(' ')}>
            <div className='menu-div'>
                {menuOpen 
                    ? <MenuOpenIcon onClick={() => setMenuOpen(false)} className='icon' fontSize='large'/> 
                    : <MenuIcon onClick={() => setMenuOpen(true)} className='icon' fontSize='large'/>}
            </div>
            <div className={'server-status ' + serverStatus}>
                <div className='status-icon'></div>
                {
                    menuOpen && <p>{getServerStatus(serverStatus)}</p>
                }
            </div>
            <ul>
                {
                    MenuItem.map((item, index) => (
                        <li key={index} className={currentPath === item.path ? 'current' : null}>
                            <a href={item.path}>
                                <item.icon className='icon' fontSize='large'/>
                                {menuOpen && <p>{item.name}</p>}
                            </a>
                        </li>
                    ))
                }
            </ul>
        </div>
    )
} 

export default Menu