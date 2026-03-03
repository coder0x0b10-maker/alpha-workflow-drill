import MacdChart from './components/MacdChart';

export default function Home() {
  return (
    <main className="p-8 bg-black text-white min-h-screen">
      <h1 className="text-2xl font-bold mb-8">AlphaSignal Drill - Dashboard</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        <section className="p-4 bg-emerald-900/30 border border-emerald-800 rounded">
          <h3 className="font-bold mb-2 text-emerald-400">RSI Indicator</h3>
          <p className="text-3xl font-mono">71.23</p>
          <p className="text-xs text-emerald-600 mt-1 uppercase tracking-wider font-bold">Overbought</p>
        </section>

        <section>
          <MacdChart symbol="BTC/USDT" />
        </section>
      </div>
    </main>
  );
}
