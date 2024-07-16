import React, { useState, useEffect } from 'react';
import { IconButton } from '@mui/material';
import {
  AccountCircle as AccountIcon,
  Settings as SettingsIcon,
  PowerSettingsNew as PowerIcon
} from '@mui/icons-material';
import './menu.scss';
import ContentSwitcher from './ContentSwitcher';
import { useData } from '../mcProvider';

const SideMenu = () => {
  const [menuOpen, setMenuOpen] = useState(false);
  const [activeMenu, setActiveMenu] = useState(null);
  const { players, numPlayers } = useData();

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
      {menuOpen && <ContentSwitcher activeMenu={activeMenu} players={players} playerCount={numPlayers} />}
    </div>
  );
};

export default SideMenu;
