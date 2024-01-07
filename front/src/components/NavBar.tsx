import * as React from 'react';
import './NavBar.css'
//import init, { download_results_for_elastic_modules_for_unidirectional_composite } from '../xlsx-writer/pkg/xlsx_writer.js'

export class NavBar extends React.Component<Record<string, never>, Record<string, never>> {
    render() {
        // init().then(() => {
        //     const array = new Float64Array(9);
        //     array[0] = 0.0;
        //     array[1] = 0.0;
        //     array[2] = 0.0;
        //     array[3] = 0.0;
        //     array[4] = 0.0;
        //     array[5] = 0.0;
        //     array[6] = 0.0;
        //     array[7] = 0.0;
        //     array[8] = 0.0;
        //     download_results_for_elastic_modules_for_unidirectional_composite(array);
        // });
        return <ul className="navbar">
            <li><button type="button">Вычисления</button></li>
            <li><button type="button">Обратная связь</button></li>
        </ul>
    }
}
