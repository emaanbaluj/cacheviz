import { useState } from 'react'
import './App.css'
import Visualizer from './pages/Visualizer'

type Page = 'dashboard' | 'visualizer' | 'about' | 'settings'

function App() {
  const [page, setPage] = useState<Page>('visualizer')

  return (
    <div className="layout">
      <aside className="sidebar">
        <div className="sidebar-title">CacheViz</div>
        <nav className="sidebar-nav">
          <a href="#" className={`nav-item ${page === 'dashboard' ? 'active' : ''}`} onClick={() => setPage('dashboard')}>Dashboard</a>
          <a href="#" className={`nav-item ${page === 'visualizer' ? 'active' : ''}`} onClick={() => setPage('visualizer')}>Visualizer</a>
          <a href="#" className={`nav-item ${page === 'about' ? 'active' : ''}`} onClick={() => setPage('about')}>About</a>
          <a href="#" className={`nav-item ${page === 'settings' ? 'active' : ''}`} onClick={() => setPage('settings')}>Settings</a>
        </nav>
      </aside>
      <main className="content">
        {page === 'visualizer' && <Visualizer />}
        {page === 'dashboard' && <h1>Dashboard</h1>}
        {page === 'about' && <h1>About</h1>}
        {page === 'settings' && <h1>Settings</h1>}
      </main>
    </div>
  )
}

export default App
