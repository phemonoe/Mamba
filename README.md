# 🃏 Poker Analyzer

A real-time AI-powered poker strategy assistant built with **Tauri** and **React**. Take screenshots of your poker tables and get instant strategic advice from GPT-4o.

## ✨ Features

- **Fast Screenshot Capture**: Native Rust performance for instant screen capture
- **AI-Powered Analysis**: GPT-4o vision model analyzes poker situations
- **Beautiful UI**: Modern React interface with Tailwind CSS
- **Global Hotkeys**: Quick access via keyboard shortcuts (coming soon)
- **Low Latency**: Optimized for real-time poker decision making
- **Cross-Platform**: Works on macOS, Windows, and Linux

## 🚀 Quick Start

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Bun**: Install from [bun.sh](https://bun.sh/)
- **OpenAI API Key**: Get one from [OpenAI Platform](https://platform.openai.com/api-keys)

### Installation

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd Mamba
   ```

2. **Install frontend dependencies**
   ```bash
   bun install
   ```

3. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env and add your OpenAI API key
   ```

4. **Run the development server**
   ```bash
   cargo tauri dev
   ```

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
OPENAI_API_KEY=your_openai_api_key_here
```

### OpenAI API Setup

1. Go to [OpenAI Platform](https://platform.openai.com/api-keys)
2. Create a new API key
3. Add it to your `.env` file
4. Ensure you have credits in your OpenAI account

## 🎮 Usage

### Taking Screenshots

1. **Click "Take Screenshot"** - Captures your entire screen
2. **Preview** - See the captured image in the left panel
3. **Click "Analyze Poker"** - Send to AI for analysis

### Understanding Results

The AI provides:

- **Action Recommendation**: Fold, Call, Raise, or All-in
- **Hand Strength**: Assessment of your current hand
- **Reasoning**: Detailed explanation of the recommendation
- **Pot Odds**: Mathematical analysis when applicable
- **Confidence**: AI's confidence level (0-100%)

### Keyboard Shortcuts (Coming Soon)

- `Cmd+Shift+P` - Quick Analysis
- `Cmd+Shift+S` - Screenshot Only
- `Cmd+Shift+H` - Hand Strength Assessment

## 🏗️ Project Structure

```
├── src/                    # React frontend
│   ├── App.tsx            # Main application component
│   ├── main.tsx           # React entry point
│   └── index.css          # Styles with Tailwind
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   ├── lib.rs         # Tauri commands
│   │   ├── screenshot.rs  # Screen capture logic
│   │   └── poker_analyzer.rs # OpenAI integration
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── package.json           # Frontend dependencies
├── vite.config.ts         # Vite configuration
└── tailwind.config.js     # Tailwind CSS config
```

## 🔍 Technical Details

### Tech Stack

- **Frontend**: React 19 + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri
- **Build Tool**: Vite + Bun
- **AI**: OpenAI GPT-4o Vision API
- **Image Processing**: Rust image crate
- **HTTP Client**: reqwest

### Performance Optimizations

- **Native screenshot capture** using Rust for minimal latency
- **Image compression** before sending to OpenAI API
- **Async processing** to prevent UI blocking
- **Smart resizing** to meet API size limits
- **Base64 encoding** for efficient data transfer

## 🛠️ Development

### Building for Production

```bash
cargo tauri build
```

### Testing Screenshot Functionality

```bash
# Test screenshot capture only
cargo tauri dev
# Click "Take Screenshot" button
```

### Testing OpenAI Integration

```bash
# Ensure OPENAI_API_KEY is set
cargo tauri dev
# Take a screenshot, then click "Analyze Poker"
```

## 🐛 Troubleshooting

### Common Issues

1. **"OPENAI_API_KEY not set"**
   - Check your `.env` file exists and contains the API key
   - Restart the development server after adding the key

2. **Screenshot fails**
   - On macOS: Grant screen recording permissions in System Preferences
   - On Windows: Run as administrator if needed
   - On Linux: Ensure X11 or Wayland permissions

3. **OpenAI API errors**
   - Check your API key is valid and has credits
   - Verify your internet connection
   - Check OpenAI service status

4. **App won't start**
   - Ensure Rust is installed: `rustc --version`
   - Install Tauri CLI: `cargo install tauri-cli`
   - Check Node.js version compatibility

## 📝 License

MIT License - feel free to use this for learning and personal projects.

## 🤝 Contributing

This is a learning project! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Share improvements

## ⚠️ Disclaimer

This tool is for educational purposes only. Please check your local laws regarding poker software assistance before using in real games.

---

**Happy analyzing! 🎰**
