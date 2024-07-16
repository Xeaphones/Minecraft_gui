import React from 'react';
import HomePage from './components/HomePage';
import Menu from './components/menu';
import SideMenu from './components/side_menu';
import './app.scss';

function App() {
    return (
        <div className="App">
            <Menu />
            <HomePage />
            <SideMenu />
        </div>
    );
}

export default App;
