import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import HomePage from './components/HomePage';
import Menu from './components/menu';
import SideMenu from './components/side_menu';
import ServerProperties from './components/ServerProperties';
import './app.scss';
import { DataProvider } from './components/mcProvider';

function App() {
    return (
        <Router>
            <div className="App">
                <Menu />
                <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/settings" element={<ServerProperties />} />
                </Routes>
                <SideMenu />
            </div>
        </Router>
    );
}

export default App;
