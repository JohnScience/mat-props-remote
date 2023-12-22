import React from "react";
import { Benchmark } from "../Benchmark";
import { BenchmarkedResultSlot, WindowWithTauri } from "../../../tauri"
import { FixedArray } from "../../../util";

export const ElasticModulesForHoneycomb: React.FC = () => {
    const numberOfModel = 1;
    const [lCellSideSize, setLCellSideSize] = React.useState(9.24);
    const [hCellSideSize, setHCellSideSize] = React.useState(8.4619);
    const [wallThickness, setWallThickness] = React.useState(0.4);
    const [angle, setAngle] = React.useState(Math.PI / 6);
    const [eForHoneycomb, setEForHoneycomb] = React.useState(7.07);
    const [nuForHoneycomb, setNuForHoneycomb] = React.useState(0.2);
    const [computedValues, setComputedValues] = React.useState<BenchmarkedResultSlot<FixedArray<number, 9>>>(([[],{secs: 0, nanos: 0}]));

    function handleLCellSideSizeChange(event: React.ChangeEvent<HTMLInputElement>) {
        setLCellSideSize(parseFloat(event.target.value));
    }

    function handleHCellSideSizeChange(event: React.ChangeEvent<HTMLInputElement>) {
        setHCellSideSize(parseFloat(event.target.value));
    }

    function handleWallThicknessChange(event: React.ChangeEvent<HTMLInputElement>) {
        setWallThickness(parseFloat(event.target.value));
    }

    function handleAngleChange(event: React.ChangeEvent<HTMLInputElement>) {
        setAngle(parseFloat(event.target.value));
    }
    
    function handleEForHoneycombChange(event: React.ChangeEvent<HTMLInputElement>) {
        setEForHoneycomb(parseFloat(event.target.value));
    }

    function handleNuForHoneycombChange(event: React.ChangeEvent<HTMLInputElement>) {
        setNuForHoneycomb(parseFloat(event.target.value));
    }

    async function compute() {
        if (!("__TAURI__" in window)) {
            console.error("Tauri API is not available in browser");
            return;
        }

        const tauriWindow = window as WindowWithTauri;

        const response = await tauriWindow.__TAURI__.invoke("elastic_modules_for_honeycomb", {
            numberOfModel: numberOfModel,
            lCellSideSize: lCellSideSize,
            hCellSideSize: hCellSideSize,
            wallThickness: wallThickness,
            angle: angle,
            eForHoneycomb: eForHoneycomb,
            nuForHoneycomb: nuForHoneycomb,
        });
        console.log(response);
        setComputedValues(response);
    }

    return <>
        <form>
            <label>Размер ячейки в длину:
                <input type="number" value={lCellSideSize} step="0.1" onChange={handleLCellSideSizeChange} />
            </label>
            <br />
            <label>Размер ячейки в высоту:
                <input type="number" value={hCellSideSize} step="0.1" onChange={handleHCellSideSizeChange} />
            </label>
            <br />
            <label>Толщина стенки:
                <input type="number" value={wallThickness} step="0.1" onChange={handleWallThicknessChange} />
            </label>
            <br />
            <label>Угол между горизонталью и наклонной стенкой ячейки соты (в радианах):
                <input type="number" value={angle} step="0.1" onChange={handleAngleChange} />
            </label>
            <br />
            <label>Модуль Юнга (E) для материала соты:
                <input type="number" value={eForHoneycomb} step="0.1" onChange={handleEForHoneycombChange} />
            </label>
            <br />
            <label>Коэффициент Пуассона (v) для материала соты:
                <input type="number" value={nuForHoneycomb} step="0.1" onChange={handleNuForHoneycombChange} />
            </label>
            <br />
            <input type="button" value="Рассчитать" onClick={compute} />

            { 
                computedValues[0].length == 9 &&
                <>
                    <h2>Значения:</h2>
                    <p>E1 = {computedValues[0][0].toFixed(10)}</p>
                    <p>E2 = {computedValues[0][1].toFixed(10)}</p>
                    <p>E3 = {computedValues[0][2].toFixed(10)}</p>
                    <p>v12 = {computedValues[0][3].toFixed(10)}</p>
                    <p>v13 = {computedValues[0][4].toFixed(10)}</p>
                    <p>v23 = {computedValues[0][5].toFixed(10)}</p>
                    <p>G12 = {computedValues[0][6].toFixed(10)}</p>
                    <p>G13 = {computedValues[0][7].toFixed(10)}</p>
                    <p>G23 = {computedValues[0][8].toFixed(10)}</p>
                    <Benchmark t={computedValues[1]} />
                </>
            }

        </form>
    </>
}
