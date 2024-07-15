import React from 'react';
import HomePage from './components/HomePage';
import Menu from './components/menu';
import './app.scss';

function App() {
    return (
        <div className="App">
            <Menu />
            <HomePage />
        </div>
    );
}

export default App;
