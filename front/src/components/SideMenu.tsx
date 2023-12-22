// import React, { useState, useRef, useEffect, useCallback } from 'react';
import React from 'react';
import './SideMenu.css'; // Create a CSS file for styling

interface SplitDivProps {
    className?: string,
}

export enum SideMenuMode {
    AnalyticalComputation,
    NumericalComputation,
}

export class SideMenu extends React.Component<SplitDivProps, Record<string, never>> {
    divRef: React.RefObject<HTMLDivElement>;

    constructor(props: SplitDivProps) {
        super(props);

        this.divRef = React.createRef<HTMLDivElement>();
    }

    render() {
        return (
            <div ref={this.divRef} className={this.props.className}>
                <p>Рассчитать аналитически</p>
                <p>Рассчитать численно</p>
            </div>
        );
    }
}
