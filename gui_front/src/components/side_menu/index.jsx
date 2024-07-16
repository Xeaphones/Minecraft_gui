import React, { useState, useEffect } from 'react';
import { IconButton } from '@mui/material';
import {
  AccountCircle as AccountIcon,
  Settings as SettingsIcon,
  PowerSettingsNew as PowerIcon
} from '@mui/icons-material';
import './menu.scss';
import ContentSwitcher from './ContentSwitcher';

const SideMenu = () => {
  const [menuOpen, setMenuOpen] = useState(false);
  const [activeMenu, setActiveMenu] = useState(null);
  const [players, setPlayers] = useState([]);
  const [playerCount, setPlayerCount] = useState(0);

  const handleMenuClick = (menu) => {
    setMenuOpen(true);
    setActiveMenu(menu);
  };

  const handleClickOutside = (event) => {
    if (!event.target.closest('.side-menu') && !event.target.closest('.menu-icons .icon')) {
      setMenuOpen(false);
      setActiveMenu(null);
    }
  };

  useEffect(() => {
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  }, []);

  useEffect(() => {
    if (activeMenu === 'players') {
      fetch('/api/query/full')
        .then((response) => response.json())
        .then((data) => {
          setPlayers(data.players);
          setPlayerCount(data.num_players);
        })
        .catch((error) => console.error('Error fetching player data:', error));
    }
  }, [activeMenu]);

  return (
    <div className={`side-menu ${menuOpen ? 'open' : 'closed'}`}>
      <div className="menu-icons">
        <IconButton onClick={() => handleMenuClick('players')}>
          <AccountIcon className='icon' />
        </IconButton>
        <IconButton onClick={() => handleMenuClick('settings')}>
          <SettingsIcon className='icon' />
        </IconButton>
        <IconButton onClick={() => handleMenuClick('logout')}>
          <PowerIcon className='icon' />
        </IconButton>
      </div>
      {menuOpen && <ContentSwitcher activeMenu={activeMenu} players={players} playerCount={playerCount} />}
    </div>
  );
};

export default SideMenu;
