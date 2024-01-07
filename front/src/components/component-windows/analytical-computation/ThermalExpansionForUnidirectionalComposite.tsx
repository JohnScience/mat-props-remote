import React, { ChangeEvent } from "react";
import { Benchmark } from "../Benchmark";
import { BenchmarkedResultSlot, WindowWithTauri } from "../../../tauri";
import { FixedArray } from "../../../util";
import { DEFAULT_BASE_URL, thermalExpansionForUnidirectionalComposite } from "../../../remote-compute";
import init, { download_results_for_thermal_expansion_for_unidirectional_composite } from "../../../xlsx-writer/pkg/xlsx_writer";

export const ThermalExpansionForUnidirectionalComposite: React.FC = () => {
    // Модель Ванина
    const numberOfModel = 1;

    const [fiberContent, setFiberContent] = React.useState(0.2);
    const [eForFiber, setEForFiber] = React.useState(100.0);
    const [nuForFiber, setNuForFiber] = React.useState(0.3);
    const [alphaForFiber, setAlphaForFiber] = React.useState(0.3);
    const [eForMatrix, setEForMatrix] = React.useState(5.0);
    const [nuForMatrix, setNuForMatrix] = React.useState(0.2);
    const [alphaForMatrix, setAlphaForMatrix] = React.useState(0.2);
    const [computedValues, setComputedValues] = React.useState<BenchmarkedResultSlot<FixedArray<number, 3>>>(([[],{secs: 0, nanos: 0}]));

    function handleFiberContentChange(event: ChangeEvent<HTMLInputElement>) {
        setFiberContent(parseFloat(event.target.value));
    }

    function handleEForFiberChange(event: ChangeEvent<HTMLInputElement>) {
        setEForFiber(parseFloat(event.target.value));
    }

    function handleNuForFiberChange(event: ChangeEvent<HTMLInputElement>) {
        setNuForFiber(parseFloat(event.target.value));
    }

    function handleAlphaForFiberChange(event: ChangeEvent<HTMLInputElement>) {
        setAlphaForFiber(parseFloat(event.target.value));
    }

    function handleEForMatrixChange(event: ChangeEvent<HTMLInputElement>) {
        setEForMatrix(parseFloat(event.target.value));
    }

    function handleNuForMatrixChange(event: ChangeEvent<HTMLInputElement>) {
        setNuForMatrix(parseFloat(event.target.value));
    }

    function handleAlphaForMatrixChange(event: ChangeEvent<HTMLInputElement>) {
        setAlphaForMatrix(parseFloat(event.target.value));
    }

    async function try_compute_remotely(): Promise<boolean> {
        const baseUrl = DEFAULT_BASE_URL;
        return thermalExpansionForUnidirectionalComposite(
            baseUrl,
            numberOfModel,
            fiberContent,
            eForFiber,
            nuForFiber,
            alphaForFiber,
            eForMatrix,
            nuForMatrix,
            alphaForMatrix
        ).then((response) => {
            console.log(response);
            setComputedValues([response, {secs: 0, nanos: 0}]);
            return true;
        }).catch((error) => {
            console.error(error);
            return false;
        })
    }

    async function try_compute_with_tauri(): Promise<boolean> {
        if (!("__TAURI__" in window)) {
            return false
        }

        const tauriWindow = window as WindowWithTauri;

        const response = await tauriWindow.__TAURI__.invoke("thermal_expansion_for_unidirectional_composite", {
            numberOfModel: numberOfModel,
            fiberContent: fiberContent,
            eForFiber: eForFiber,
            nuForFiber: nuForFiber,
            alphaForFiber: alphaForFiber,
            eForMatrix: eForMatrix,
            nuForMatrix: nuForMatrix,
            alphaForMatrix: alphaForMatrix,
        });
        console.log(response);
        setComputedValues(response);
        return true;
    }

    async function compute() {
        if (!(await try_compute_with_tauri() || await try_compute_remotely())) {
            console.error("Failed to compute because Tauri API is not available in browser and remote computation failed");
            return;
        }
    }

    function exportToExcel() {
        const array = new Float64Array(9);
        array[0] = computedValues[0][0] as number;
        array[1] = computedValues[0][1] as number;
        array[2] = computedValues[0][2] as number;
        init().then(() => {
            download_results_for_thermal_expansion_for_unidirectional_composite(array);
        });
    }

    return <>
        <form>
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
            <label>Коэффициент линейного теплового расширения (α) для волокон:
                <input type="number" value={alphaForFiber} step="0.1" onChange={handleAlphaForFiberChange} />
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
            <label>Коэффициент линейного теплового расширения (α) для матрицы:
                <input type="number" value={alphaForMatrix} step="0.1" onChange={handleAlphaForMatrixChange} />
            </label>
            <br />
            <input type="button" value="Рассчитать" onClick={compute} />

            { computedValues[0].length == 3 &&
                <>
                    <input type="button" value="Эксортировать как .xlsx" onClick={exportToExcel} />

                    <h2>Значения:</h2>
                    <p>α1 = {computedValues[0][0].toFixed(10)}</p>
                    <p>α2 = {computedValues[0][1].toFixed(10)}</p>
                    <p>α3 = {computedValues[0][2].toFixed(10)}</p>
                    <Benchmark t={computedValues[1]} />
                </>
            }

        </form>
    </>
}
