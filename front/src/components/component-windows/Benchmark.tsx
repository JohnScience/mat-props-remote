import React from 'react';

type BenchmarkProps = {
  t: {
    secs: number;
    nanos: number;
  };
};

export const Benchmark: React.FC<BenchmarkProps> = ({ t: { secs: s, nanos: ns } }) => {
  return (
    <p>
      Вычислено за {s}с {Math.floor(ns / 1e6)}мс {Math.floor((ns % 1e6) / 1e3)}мкс {ns % 1e3}нс
    </p>
  );
};
