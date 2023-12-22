// import React, { useState, useRef, useEffect, useCallback } from 'react';
import React from 'react';
import { SideMenu, SideMenuMode } from './SideMenu';
import './SplitDiv.css'; // Create a CSS file for styling
import { AnalyticalComputationWindow } from './component-windows/analytical-computation/AnalyticalComputationWindow';

interface SplitDivState {
    isDragging: boolean;
    mode: SideMenuMode;
}

export class SplitDiv extends React.Component<Record<string, never>, SplitDivState> {
    container: React.RefObject<HTMLDivElement>;
    splitHandle: React.RefObject<HTMLDivElement>;
    splitLeft: React.RefObject<SideMenu>;
    splitRight: React.RefObject<HTMLDivElement>;

    constructor(props: Record<string, never>) {
        super(props);
        
        const isDragging = false;
        const mode = SideMenuMode.AnalyticalComputation;

        this.state = { isDragging, mode };

        this.container = React.createRef();
        this.splitHandle = React.createRef();
        this.splitLeft = React.createRef();
        this.splitRight = React.createRef();
    }

    setMode(mode: SideMenuMode) {
        this.setState({ mode });
    }

    handleMouseEnter = () => {
        document.body.style.cursor = 'col-resize';
        this.splitHandle.current?.addEventListener('mousedown', this.handleMouseDown);
        window.addEventListener('mousemove', this.handleMouseMove);
    }

    handleMouseLeave = () => {
        document.body.style.cursor = 'default';
        this.splitHandle.current?.removeEventListener('mousedown', this.handleMouseDown);
    }

    handleMouseDown = () => {
        this.setState({ isDragging: true });
        console.log('mousedown');
        window.addEventListener('mouseup', this.handleMouseUp);
    }

    handleMouseUp = () => {
        this.setState({ isDragging: false });
        console.log('mouseup');
        window.removeEventListener('mouseup', this.handleMouseUp);
        window.removeEventListener('mousemove', this.handleMouseMove);
    }

    handleMouseMove = (e: MouseEvent) => {
        if (!this.state.isDragging) return;
        const splitLeftWidth = e.clientX;
        const splitRightWidth = window.innerWidth - e.clientX;
        if (this.splitLeft.current && this.splitLeft.current.divRef.current) {
            this.splitLeft.current.divRef.current.style.width = `${splitLeftWidth}px`;
        }
        if (this.splitRight.current) {
            this.splitRight.current.style.width = `${splitRightWidth}px`;
        }
    }

    componentDidMount() {
        const totalWidth = window.innerWidth;
        const splitLeftWidth = totalWidth * 1 / 5;
        const splitRightWidth = totalWidth * 4 / 5;
        if (this.splitLeft.current && this.splitLeft.current.divRef.current) {
            this.splitLeft.current.divRef.current.style.width = `${splitLeftWidth}px`;
        }
        if (this.splitRight.current) {
            this.splitRight.current.style.width = `${splitRightWidth}px`;
        }
    }

    componentWillUnmount() {
        this.splitHandle.current?.removeEventListener('mousedown', this.handleMouseDown);
        window.removeEventListener('mouseup', this.handleMouseUp);
        window.removeEventListener('mousemove', this.handleMouseMove);
    }

    render() {
        return (
            <div className="split-container">
                <SideMenu ref={this.splitLeft} className="split-left"></SideMenu>
                <div
                    ref={this.splitHandle}
                    className="split-handle"
                    onMouseEnter={this.handleMouseEnter}
                    onMouseLeave={this.handleMouseLeave}
                    // onMouseDown and onMouseMove are handled not here
                    // because onMouseEnter and onMouseLeave are triggered
                    // much earlier than mouseup and mousemove can be triggered.
                    // This is a problem with React. At least in Google Chrome.
                ></div>
                { this.state.mode === SideMenuMode.AnalyticalComputation && <AnalyticalComputationWindow splitRight={this.splitRight} className="split-right"></AnalyticalComputationWindow> }
            </div>
        );
    }
}
