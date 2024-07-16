import React from 'react';

const ContentSwitcher = ({ activeMenu, players, playerCount }) => {
  const renderContent = () => {
    switch (activeMenu) {
      case 'players':
        return (
          <div>
            <p>Nombre de joueurs : {playerCount}</p>
            <div>Liste des joueurs:
              <ul>{players.map((player, index) => <li key={index}>{player}</li>)}</ul>
            </div>
          </div>
        );
      case 'settings':
        return <div>Settings content...</div>;
      case 'logout':
        return <div>Logging out...</div>;
      default:
        return null;
    }
  };

  return (
    <div className="menu-content">
      {renderContent()}
    </div>
  );
};

export default ContentSwitcher;
