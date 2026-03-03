"use client";

import React, { useEffect, useState, useMemo } from 'react';

interface MacdResult {
  macd_line: number;
  signal_line: number;
  histogram: number;
  timestamp: number;
}

interface MacdConfig {
  fastPeriod: number;
  slowPeriod: number;
  signalPeriod: number;
}

interface MacdChartProps {
  symbol: string;
  data?: MacdResult[];
  config?: Partial<MacdConfig>;
  loading?: boolean;
}

const DEFAULT_CONFIG: MacdConfig = {
  fastPeriod: 12,
  slowPeriod: 26,
  signalPeriod: 9,
};

const COLORS = {
  macdLine: "#0055ff",
  signalLine: "#ff9900",
  positiveHistogram: "#26a69a",
  negativeHistogram: "#ef5350",
  zeroLine: "#888888",
};

// Mock data generator
const generateMockData = (count: number): MacdResult[] => {
  const data: MacdResult[] = [];
  const now = Date.now();
  for (let i = 0; i < count; i++) {
    const t = i / 10;
    const macd = Math.sin(t) * 2 + Math.cos(t * 0.5);
    const signal = Math.sin(t - 0.5) * 1.8;
    data.push({
      macd_line: macd,
      signal_line: signal,
      histogram: macd - signal,
      timestamp: now - (count - i) * 60000,
    });
  }
  return data;
};

export default function MacdChart({ symbol, data: initialData, config: initialConfig, loading: initialLoading }: MacdChartProps) {
  const [data, setData] = useState<MacdResult[]>(initialData || []);
  const [loading, setLoading] = useState(initialLoading ?? true);
  const config = { ...DEFAULT_CONFIG, ...initialConfig };

  useEffect(() => {
    if (!initialData) {
      // Simulate API call
      const timer = setTimeout(() => {
        setData(generateMockData(50));
        setLoading(false);
      }, 1000);
      return () => clearTimeout(timer);
    } else {
      setLoading(false);
    }
  }, [initialData]);

  const chartBounds = useMemo(() => {
    if (data.length === 0) return { min: 0, max: 0 };
    let min = Infinity;
    let max = -Infinity;
    data.forEach(d => {
      min = Math.min(min, d.macd_line, d.signal_line, d.histogram);
      max = Math.max(max, d.macd_line, d.signal_line, d.histogram);
    });
    // Add some padding
    const padding = (max - min) * 0.1;
    return { min: min - padding, max: max + padding };
  }, [data]);

  const getY = (val: number, height: number) => {
    const range = chartBounds.max - chartBounds.min;
    if (range === 0) return height / 2;
    return height - ((val - chartBounds.min) / range) * height;
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center w-full h-64 bg-gray-900 rounded-lg animate-pulse">
        <span className="text-gray-400">Loading MACD Chart for {symbol}...</span>
      </div>
    );
  }

  const width = 600;
  const height = 300;
  const barWidth = (width / data.length) * 0.8;

  return (
    <div className="p-4 bg-gray-900 rounded-lg shadow-xl text-white font-sans w-full max-w-2xl">
      <div className="flex justify-between items-center mb-4">
        <div>
          <h3 className="text-lg font-bold">MACD ({config.fastPeriod}, {config.slowPeriod}, {config.signalPeriod})</h3>
          <p className="text-xs text-gray-400">{symbol}</p>
        </div>
        <div className="flex gap-4 text-xs">
          <div className="flex items-center gap-1">
            <span className="w-3 h-0.5 bg-[blue]" style={{ backgroundColor: COLORS.macdLine }}></span>
            <span>MACD: {data[data.length - 1]?.macd_line.toFixed(2)}</span>
          </div>
          <div className="flex items-center gap-1">
            <span className="w-3 h-0.5" style={{ backgroundColor: COLORS.signalLine }}></span>
            <span>Signal: {data[data.length - 1]?.signal_line.toFixed(2)}</span>
          </div>
        </div>
      </div>

      <div className="relative border-l border-b border-gray-700">
        <svg width="100%" height={height} viewBox={`0 0 ${width} ${height}`} preserveAspectRatio="none">
          {/* Zero Line */}
          <line
            x1="0"
            y1={getY(0, height)}
            x2={width}
            y2={getY(0, height)}
            stroke={COLORS.zeroLine}
            strokeDasharray="4"
          />

          {/* Histogram */}
          {data.map((d, i) => {
            const x = (i / data.length) * width;
            const y0 = getY(0, height);
            const yVal = getY(d.histogram, height);
            return (
              <rect
                key={i}
                x={x}
                y={Math.min(y0, yVal)}
                width={barWidth}
                height={Math.abs(y0 - yVal)}
                fill={d.histogram >= 0 ? COLORS.positiveHistogram : COLORS.negativeHistogram}
                opacity="0.8"
              />
            );
          })}

          {/* MACD Line */}
          <polyline
            fill="none"
            stroke={COLORS.macdLine}
            strokeWidth="2"
            points={data.map((d, i) => `${(i / data.length) * width + barWidth/2},${getY(d.macd_line, height)}`).join(' ')}
          />

          {/* Signal Line */}
          <polyline
            fill="none"
            stroke={COLORS.signalLine}
            strokeWidth="2"
            points={data.map((d, i) => `${(i / data.length) * width + barWidth/2},${getY(d.signal_line, height)}`).join(' ')}
          />
        </svg>
      </div>
      
      <div className="mt-2 flex justify-between text-[10px] text-gray-500">
        <span>{new Date(data[0]?.timestamp).toLocaleTimeString()}</span>
        <span>{new Date(data[data.length - 1]?.timestamp).toLocaleTimeString()}</span>
      </div>
    </div>
  );
}
