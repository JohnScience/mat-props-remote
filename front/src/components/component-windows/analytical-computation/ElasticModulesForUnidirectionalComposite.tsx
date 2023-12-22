import React, { ChangeEvent } from "react";
import { Benchmark } from "../Benchmark";
import { BenchmarkedResultSlot, WindowWithTauri } from "../../../tauri"
import { FixedArray } from "../../../util";

// TODO: add units (GPa, MPa, etc.)
// TODO: offer suggested values for E and v
export const ElasticModulesForUnidirectionalComposite: React.FC = () => {
    // FIXME: change the variable name to more English-like
    // FIXME: change the indexation to 0-based
    const [numberOfModel, setNumberOfModel] = React.useState<number>(1);
    const [fiberContent, setFiberContent] = React.useState<number>(0.2);
    const [eForFiber, setEForFiber] = React.useState<number>(100.0);
    const [nuForFiber, setNuForFiber] = React.useState<number>(0.3);
    const [eForMatrix, setEForMatrix] = React.useState<number>(5.0);
    const [nuForMatrix, setNuForMatrix] = React.useState<number>(0.2);
    const [computedValues, setComputedValues] = React.useState<BenchmarkedResultSlot<FixedArray<number, 9>>>([[],{secs: 0, nanos: 0}]);

    function handleNumberOfModelChange(event: ChangeEvent<HTMLSelectElement>) {
        setNumberOfModel(parseInt(event.target.value));
    }

    function handleFiberContentChange(event: ChangeEvent<HTMLInputElement>) {
        setFiberContent(parseFloat(event.target.value));
    }

    function handleEForFiberChange(event: ChangeEvent<HTMLInputElement>) {
        setEForFiber(parseFloat(event.target.value));
    }

    function handleNuForFiberChange(event: ChangeEvent<HTMLInputElement>) {
        setNuForFiber(parseFloat(event.target.value));
    }

    function handleEForMatrixChange(event: ChangeEvent<HTMLInputElement>) {
        setEForMatrix(parseFloat(event.target.value));
    }

    function handleNuForMatrixChange(event: ChangeEvent<HTMLInputElement>) {
        setNuForMatrix(parseFloat(event.target.value));
    }

    async function compute() {
        if (!("__TAURI__" in window)) {
            console.error("Tauri API is not available in browser");
            return;
        }

        const tauriWindow = window as WindowWithTauri;

        const response = await tauriWindow.__TAURI__.invoke("elastic_modules_for_unidirectional_composite", {
            numberOfModel: numberOfModel,
            fiberContent: fiberContent,
            eForFiber: eForFiber,
            nuForFiber: nuForFiber,
            eForMatrix: eForMatrix,
            nuForMatrix: nuForMatrix
        });
        console.log(response);
        setComputedValues(response);
    }

    return <>
        <form>
            <label>Модель:
                <select value={numberOfModel} onChange={handleNumberOfModelChange}>
                    <option value="1">Правило смеси</option>
                    <option value="2"> Модель Ванина</option>
                </select>
            </label>
            <br />
            <label>Доля объема волокон в композите (от 0 до 1):
                <input type="number" value={fiberContent} min="0" max="1" step="0.01" onChange={handleFiberContentChange} />
            </label>
            <br />
            <label>Модуль Юнга (E) для волокон:
                <input type="number" value={eForFiber} step="0.1" onChange={handleEForFiberChange} />
            </label>
            <br />
            <label>Коэффициент Пуассона (v) для волокон:
                <input type="number" value={nuForFiber} step="0.1" onChange={handleNuForFiberChange} />
            </label>
            <br />
            <label>Модуль Юнга (E) для матрицы:
                <input type="number" value={eForMatrix} step="0.1" onChange={handleEForMatrixChange} />
            </label>
            <br />
            <label>Коэффициент Пуассона (v) для матрицы:
                <input type="number" value={nuForMatrix} step="0.1" onChange={handleNuForMatrixChange} />
            </label>
            <br />
            <input type="button" value="Рассчитать" onClick={compute} />

            { computedValues[0].length == 9 &&
                <>
                    <h2>Значения:</h2>
                    <p>E1 = {computedValues[0][0].toFixed(10)}</p>
                    <p>E2 = {computedValues[0][1].toFixed(10)}</p>
                    <p>E3 = {computedValues[0][2].toFixed(10)}</p>
                    <p>v12 = {computedValues[0][3].toFixed(10)}</p>
                    <p>v13 = {computedValues[0][4].toFixed(10)}</p>
                    <p>v23 = {numberOfModel==1 ? "Не вычислимо в рамках модели" : computedValues[0][5].toFixed(10)}</p>
                    <p>G12 = {computedValues[0][6].toFixed(10)}</p>
                    <p>G13 = {computedValues[0][7].toFixed(10)}</p>
                    <p>G23 = {numberOfModel==1 ? "Не вычислимо в рамках модели" : computedValues[0][8].toFixed(10)}</p>
                    <Benchmark t={computedValues[1]} />
                </>
            }

        </form>
    </>
};

