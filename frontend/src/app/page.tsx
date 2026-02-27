import React from 'react';

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
        <h1 className="text-4xl font-bold mb-8">Drill Dashboard</h1>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 w-full">
          {/* Indicators Card */}
          <div className="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6 border border-gray-200 dark:border-gray-700">
            <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-gray-100">Indicators</h2>
            <div className="flex items-center justify-between">
              <span className="text-gray-600 dark:text-gray-300">RSI (14)</span>
              <span className="text-2xl font-bold text-blue-600 dark:text-blue-400">71.23</span>
            </div>
            <div className="mt-4 text-sm text-gray-500 dark:text-gray-400">
              Status: <span className="text-red-500 font-medium">Overbought</span>
            </div>
          </div>

          {/* Placeholder for other cards */}
          <div className="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6 border border-gray-200 dark:border-gray-700 opacity-50">
            <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-gray-100">Volume</h2>
            <div className="text-gray-500">Coming soon...</div>
          </div>
        </div>
      </div>
    </main>
  );
}
