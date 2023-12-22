import * as React from 'react';
import './NavBar.css'

export class NavBar extends React.Component<Record<string, never>, Record<string, never>> {
    render() {
        return <ul className="navbar">
            <li><button type="button">Вычисления</button></li>
            <li><button type="button">Обратная связь</button></li>
        </ul>
    }
}
