import { useState } from 'react'
import './Visualizer.css'

type EvictionPolicy = 'LRU' | 'LFU' | 'FIFO'

function Visualizer() {
  const [cacheSize, setCacheSize] = useState(8)
  const [policy, setPolicy] = useState<EvictionPolicy>('LRU')
  const [blockSize, setBlockSize] = useState(64)
  const [associativity, setAssociativity] = useState(1)
  const [code, setCode] = useState('')

  const ramSize = 32
  const cacheSlots = Array.from({ length: cacheSize }, (_, i) => i)
  const ramSlots = Array.from({ length: ramSize }, (_, i) => i)

  return (
    <div className="visualizer">
      <div className="pane config-pane">
        <h2 className="pane-title">Configuration</h2>

        <div className="config-group">
          <label className="config-label">Cache Size (blocks)</label>
          <input
            type="number"
            className="config-input"
            value={cacheSize}
            min={1}
            max={64}
            onChange={(e) => setCacheSize(Number(e.target.value))}
          />
        </div>

        <div className="config-group">
          <label className="config-label">Block Size (bytes)</label>
          <select
            className="config-select"
            value={blockSize}
            onChange={(e) => setBlockSize(Number(e.target.value))}
          >
            <option value={16}>16</option>
            <option value={32}>32</option>
            <option value={64}>64</option>
            <option value={128}>128</option>
          </select>
        </div>

        <div className="config-group">
          <label className="config-label">Eviction Policy</label>
          <select
            className="config-select"
            value={policy}
            onChange={(e) => setPolicy(e.target.value as EvictionPolicy)}
          >
            <option value="LRU">LRU</option>
            <option value="LFU">LFU</option>
            <option value="FIFO">FIFO</option>
          </select>
        </div>

        <div className="config-group">
          <label className="config-label">Associativity</label>
          <select
            className="config-select"
            value={associativity}
            onChange={(e) => setAssociativity(Number(e.target.value))}
          >
            <option value={1}>Direct Mapped</option>
            <option value={2}>2-Way</option>
            <option value={4}>4-Way</option>
            <option value={0}>Fully Associative</option>
          </select>
        </div>

        <div className="config-group code-group">
          <label className="config-label">Code</label>
          <div className="code-editor-wrapper">
            <div className="line-numbers">
              {code.split('\n').map((_, i) => (
                <span key={i}>{i + 1}</span>
              ))}
            </div>
            <textarea
              className="code-editor"
              spellCheck={false}
              value={code}
              onChange={(e) => setCode(e.target.value)}
              placeholder={'int arr[8];\nfor (int i = 0; i < 8; i++)\n  arr[i] = i * 2;'}
            />
          </div>
        </div>

        <div className="config-actions">
          <button className="btn btn-run">Run</button>
          <button className="btn btn-step">Step</button>
          <button className="btn btn-reset">Reset</button>
        </div>
      </div>

      <div className="pane viz-pane">
        <h2 className="pane-title">Cache State</h2>

        <div className="cache-table-wrapper">
          <table className="cache-table">
            <thead>
              <tr>
                <th>Set</th>
                <th>Valid</th>
                <th>Tag</th>
                <th>Data</th>
                <th>State</th>
              </tr>
            </thead>
            <tbody>
              {cacheSlots.map((i) => (
                <tr key={i} className="cache-row empty">
                  <td className="cell-set">{i}</td>
                  <td className="cell-valid">0</td>
                  <td className="cell-tag">--</td>
                  <td className="cell-data">--</td>
                  <td className="cell-state">—</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <h2 className="pane-title ram-title">RAM</h2>

        <div className="ram-table-wrapper">
          <table className="cache-table">
            <thead>
              <tr>
                <th>Addr</th>
                <th>Data</th>
              </tr>
            </thead>
            <tbody>
              {ramSlots.map((i) => (
                <tr key={i} className="ram-row">
                  <td className="cell-addr">0x{(i * blockSize).toString(16).toUpperCase().padStart(4, '0')}</td>
                  <td className="cell-data">0x{(0).toString(16).toUpperCase().padStart(2, '0')}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <div className="viz-stats">
          <div className="stat">
            <span className="stat-label">Hits</span>
            <span className="stat-value hit">0</span>
          </div>
          <div className="stat">
            <span className="stat-label">Misses</span>
            <span className="stat-value miss">0</span>
          </div>
          <div className="stat">
            <span className="stat-label">Hit Rate</span>
            <span className="stat-value">0%</span>
          </div>
          <div className="stat">
            <span className="stat-label">Evictions</span>
            <span className="stat-value">0</span>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Visualizer
