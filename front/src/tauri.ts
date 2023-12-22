import { FixedArray } from "./util";

type TauriInvokeFn =
  | "elastic_modules_for_unidirectional_composite"
  | "thermal_conductivity_for_unidirectional_composite"
  | "elastic_modules_for_honeycomb"
  | "thermal_expansion_for_honeycomb"
  | "thermal_expansion_for_unidirectional_composite";

type TauriInvokeArgsMap = {
  elastic_modules_for_unidirectional_composite: {
    numberOfModel: number,
    fiberContent: number,
    eForFiber: number,
    nuForFiber: number,
    eForMatrix: number,
    nuForMatrix: number,
  };
  thermal_conductivity_for_unidirectional_composite: {
    numberOfModel: number,
    fiberContent: number,
    kForFiber: number,
    kForMatrix: number,
  };
  elastic_modules_for_honeycomb: {
    numberOfModel: 1;
    lCellSideSize: number,
    hCellSideSize: number,
    wallThickness: number,
    angle: number,
    eForHoneycomb: number,
    nuForHoneycomb: number,
  };
  thermal_expansion_for_honeycomb: {
    numberOfModel: number,
    lCellSideSize: number,
    hCellSideSize: number,
    wallThickness: number,
    angle: number,
    alphaForHoneycomb: number,
  };
  thermal_expansion_for_unidirectional_composite: {
    numberOfModel: number,
    fiberContent: number,
    eForFiber: number,
    nuForFiber: number,
    alphaForFiber: number,
    eForMatrix: number,
    nuForMatrix: number,
    alphaForMatrix: number,
  };
};

type BenchmarkedResult<T> = Promise<[T, { secs: number; nanos: number }]>;
export type BenchmarkedResultSlot<T extends number[]> = [T | [], { secs: number; nanos: number }];

type TauriInvokeReturnMap = {
  elastic_modules_for_unidirectional_composite: BenchmarkedResult<FixedArray<number, 9>>;
  thermal_conductivity_for_unidirectional_composite: BenchmarkedResult<FixedArray<number, 3>>;
  elastic_modules_for_honeycomb: BenchmarkedResult<FixedArray<number, 9>>;
  thermal_expansion_for_honeycomb: BenchmarkedResult<FixedArray<number, 3>>;
  thermal_expansion_for_unidirectional_composite: BenchmarkedResult<FixedArray<number, 3>>;
};

type TauriInvokeArgs<T extends TauriInvokeFn> = TauriInvokeArgsMap[T];
type TauriInvokeReturn<T extends TauriInvokeFn> = TauriInvokeReturnMap[T];

interface GlobalTauri {
  invoke<T extends TauriInvokeFn>(
    cmd: T,
    args: TauriInvokeArgs<T>
  ): TauriInvokeReturn<T>;
}

export interface WindowWithTauri extends Window {
    __TAURI__: GlobalTauri
}
