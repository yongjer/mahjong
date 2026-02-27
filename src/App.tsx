import { useState, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Recommendation {
  discard_tile: string;
  shanten_after: number;
  ukeire: string[];
  ukeire_count: number;
  score: number;
}

interface AnalysisResult {
  current_shanten: number;
  recommendations: Recommendation[];
}

const ALL_TILES = [
  ...[1, 2, 3, 4, 5, 6, 7, 8, 9].map(n => `${n}m`),
  ...[1, 2, 3, 4, 5, 6, 7, 8, 9].map(n => `${n}p`),
  ...[1, 2, 3, 4, 5, 6, 7, 8, 9].map(n => `${n}s`),
  ...[1, 2, 3, 4, 5, 6, 7].map(n => `${n}z`),
];

const TILE_NAMES: Record<string, string> = {
  "1z": "East", "2z": "South", "3z": "West", "4z": "North",
  "5z": "Zhong", "6z": "Fa", "7z": "Bai"
};

function App() {
  const [hand, setHand] = useState<string[]>([]);
  const [discards, setDiscards] = useState<string[]>([]);
  const [activeMode, setActiveMode] = useState<"hand" | "discards">("hand");
  
  const [result, setResult] = useState<AnalysisResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const handStr = useMemo(() => {
    // Group tiles for the backend parser (e.g. 1m2m3m -> 123m)
    const groups: Record<string, string[]> = { m: [], p: [], s: [], z: [] };
    hand.forEach(t => {
      const val = t[0];
      const suite = t[1];
      groups[suite].push(val);
    });
    return Object.entries(groups)
      .map(([suite, vals]) => vals.sort().join("") + suite)
      .filter(s => s.length > 1)
      .join("");
  }, [hand]);

  async function analyze() {
    if (hand.length !== 17) {
      setError("Hand must have exactly 17 tiles for analysis.");
      return;
    }
    setLoading(true);
    setError("");
    try {
      const res = await invoke<AnalysisResult>("analyze_hand", { 
        handStr, 
        discards: discards 
      });
      setResult(res);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  const addTile = (tile: string) => {
    if (activeMode === "hand") {
      if (hand.length < 17) setHand([...hand, tile]);
    } else {
      setDiscards([...discards, tile]);
    }
    setResult(null);
  };

  const removeHandTile = (index: number) => {
    const newHand = [...hand];
    newHand.splice(index, 1);
    setHand(newHand);
    setResult(null);
  };

  const removeDiscardTile = (index: number) => {
    const newDiscards = [...discards];
    newDiscards.splice(index, 1);
    setDiscards(newDiscards);
    setResult(null);
  };

  const getTileLabel = (tile: string) => {
    if (tile.endsWith("z")) return TILE_NAMES[tile] || tile;
    return tile;
  };

  return (
    <main className="container">
      <h1>Taiwanese Mahjong Solver</h1>

      <div className="selector-layout">
        <div className="tile-grid-container">
          <h3>Available Tiles</h3>
          <div className="tile-grid">
            {ALL_TILES.map(tile => (
              <button key={tile} className={`tile-btn ${tile.endsWith('z') ? 'honor' : ''}`} onClick={() => addTile(tile)}>
                {getTileLabel(tile)}
              </button>
            ))}
          </div>
        </div>

        <div className="state-container">
          <div className={`pool ${activeMode === 'hand' ? 'active' : ''}`} onClick={() => setActiveMode('hand')}>
            <h3>Hand ({hand.length}/17) {activeMode === 'hand' && "◀"}</h3>
            <div className="tile-pool">
              {hand.map((tile, i) => (
                <span key={i} className="tile-item" onClick={(e) => { e.stopPropagation(); removeHandTile(i); }}>
                  {getTileLabel(tile)}
                </span>
              ))}
              {hand.length === 0 && <span className="placeholder">Click tiles to add to hand</span>}
            </div>
          </div>

          <div className={`pool ${activeMode === 'discards' ? 'active' : ''}`} onClick={() => setActiveMode('discards')}>
            <h3>Visible Discards ({discards.length}) {activeMode === 'discards' && "◀"}</h3>
            <div className="tile-pool">
              {discards.map((tile, i) => (
                <span key={i} className="tile-item discard" onClick={(e) => { e.stopPropagation(); removeDiscardTile(i); }}>
                  {getTileLabel(tile)}
                </span>
              ))}
              {discards.length === 0 && <span className="placeholder">Click tiles to add to discards</span>}
            </div>
          </div>

          <div className="actions">
            <button className="analyze-btn" onClick={analyze} disabled={loading || hand.length !== 17}>
              {loading ? "Analyzing..." : "Analyze Hand"}
            </button>
            <button className="clear-btn" onClick={() => { setHand([]); setDiscards([]); setResult(null); }}>Clear All</button>
          </div>
          {error && <div className="error">{error}</div>}
        </div>
      </div>

      {result && (
        <div className="result-section">
          <h2>Current Shanten: {result.current_shanten}</h2>
          <table>
            <thead>
              <tr>
                <th>Discard</th>
                <th>Shanten After</th>
                <th>Ukeire (Improvements)</th>
                <th>Outs</th>
              </tr>
            </thead>
            <tbody>
              {result.recommendations.map((rec, idx) => (
                <tr key={idx}>
                  <td className="bold">{getTileLabel(rec.discard_tile)}</td>
                  <td>{rec.shanten_after}</td>
                  <td className="ukeire-list">
                    {rec.ukeire.map(getTileLabel).join(", ")}
                  </td>
                  <td>{rec.ukeire_count}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </main>
  );
}

export default App;
