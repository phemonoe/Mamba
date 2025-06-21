import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

interface PokerAnalysis {
  action_recommendation: string
  reasoning: string
  hand_strength: string
  pot_odds: string | null
  confidence: number
}

function App() {
  const [analysis, setAnalysis] = useState<PokerAnalysis | null>(null)
  const [isAnalyzing, setIsAnalyzing] = useState(false)
  const [screenshot, setScreenshot] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const takeScreenshot = async () => {
    try {
      setError(null)
      const base64Image = await invoke<string>('take_screenshot')
      setScreenshot(`data:image/jpeg;base64,${base64Image}`)
    } catch (err) {
      setError(`Screenshot failed: ${err}`)
      console.error('Screenshot error:', err)
    }
  }

  const analyzePoker = async () => {
    try {
      setError(null)
      setIsAnalyzing(true)
      const result = await invoke<PokerAnalysis>('analyze_poker_screenshot')
      setAnalysis(result)
    } catch (err) {
      setError(`Analysis failed: ${err}`)
      console.error('Analysis error:', err)
    } finally {
      setIsAnalyzing(false)
    }
  }

  const getConfidenceColor = (confidence: number) => {
    if (confidence >= 0.8) return 'text-green-600'
    if (confidence >= 0.6) return 'text-yellow-600'
    return 'text-red-600'
  }

  const getRecommendationColor = (action: string) => {
    const lowerAction = action.toLowerCase()
    if (lowerAction.includes('fold')) return 'bg-red-500'
    if (lowerAction.includes('call')) return 'bg-blue-500'
    if (lowerAction.includes('raise') || lowerAction.includes('bet')) return 'bg-green-500'
    if (lowerAction.includes('all-in')) return 'bg-purple-500'
    return 'bg-gray-500'
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800 text-white">
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold mb-2 bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
            🃏 Poker Analyzer
          </h1>
          <p className="text-gray-300">AI-Powered Poker Strategy Assistant</p>
        </div>

        {/* Action Buttons */}
        <div className="flex justify-center gap-4 mb-8">
          <button
            onClick={takeScreenshot}
            className="px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors duration-200 flex items-center gap-2"
          >
            📸 Take Screenshot
          </button>
          <button
            onClick={analyzePoker}
            disabled={isAnalyzing}
            className="px-6 py-3 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 rounded-lg font-semibold transition-colors duration-200 flex items-center gap-2"
          >
            {isAnalyzing ? '🤔 Analyzing...' : '🧠 Analyze Poker'}
          </button>
        </div>

        {/* Error Display */}
        {error && (
          <div className="mb-6 p-4 bg-red-900/50 border border-red-500 rounded-lg">
            <p className="text-red-300">{error}</p>
          </div>
        )}

        {/* Main Content Grid */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Screenshot Preview */}
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">📱 Screenshot Preview</h2>
            {screenshot ? (
              <img
                src={screenshot}
                alt="Poker table screenshot"
                className="w-full rounded-lg border border-gray-600"
              />
            ) : (
              <div className="aspect-video bg-gray-700 rounded-lg flex items-center justify-center">
                <p className="text-gray-400">No screenshot taken yet</p>
              </div>
            )}
          </div>

          {/* Analysis Results */}
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">🎯 Analysis Results</h2>
            {analysis ? (
              <div className="space-y-4">
                {/* Action Recommendation */}
                <div className="p-4 rounded-lg bg-gray-700">
                  <h3 className="font-semibold mb-2">Recommended Action</h3>
                  <span className={`px-3 py-1 rounded-full text-white font-bold ${getRecommendationColor(analysis.action_recommendation)}`}>
                    {analysis.action_recommendation.toUpperCase()}
                  </span>
                </div>

                {/* Hand Strength */}
                <div className="p-4 rounded-lg bg-gray-700">
                  <h3 className="font-semibold mb-2">Hand Strength</h3>
                  <p className="text-gray-300">{analysis.hand_strength}</p>
                </div>

                {/* Pot Odds */}
                {analysis.pot_odds && (
                  <div className="p-4 rounded-lg bg-gray-700">
                    <h3 className="font-semibold mb-2">Pot Odds</h3>
                    <p className="text-gray-300">{analysis.pot_odds}</p>
                  </div>
                )}

                {/* Confidence */}
                <div className="p-4 rounded-lg bg-gray-700">
                  <h3 className="font-semibold mb-2">Confidence</h3>
                  <p className={`font-bold ${getConfidenceColor(analysis.confidence)}`}>
                    {(analysis.confidence * 100).toFixed(0)}%
                  </p>
                </div>

                {/* Reasoning */}
                <div className="p-4 rounded-lg bg-gray-700">
                  <h3 className="font-semibold mb-2">Reasoning</h3>
                  <p className="text-gray-300 leading-relaxed">{analysis.reasoning}</p>
                </div>
              </div>
            ) : (
              <div className="text-center py-8">
                <p className="text-gray-400">Click "Analyze Poker" to get AI insights</p>
              </div>
            )}
          </div>
        </div>

        {/* Keyboard Shortcuts Info */}
        <div className="mt-8 p-4 bg-gray-800 rounded-lg">
          <h3 className="font-semibold mb-2">⌨️ Keyboard Shortcuts</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
            <div className="text-gray-300">
              <span className="font-mono bg-gray-700 px-2 py-1 rounded">Cmd+Shift+P</span> - Quick Analysis
            </div>
            <div className="text-gray-300">
              <span className="font-mono bg-gray-700 px-2 py-1 rounded">Cmd+Shift+S</span> - Screenshot Only
            </div>
            <div className="text-gray-300">
              <span className="font-mono bg-gray-700 px-2 py-1 rounded">Cmd+Shift+H</span> - Hand Strength
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default App
