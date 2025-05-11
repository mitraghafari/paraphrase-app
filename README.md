# 🧠 AI Text Paraphrasing App

This is a full-stack web app that allows users to select and paraphrase text using AI.  
Built with:

- ✅ Frontend: React + TypeScript + Vite
- ✅ Backend: Rust + Actix-web
- ✅ AI API: OpenAI (ChatGPT)
- ✅ Frontend Deployment: Vercel
- ✅ Backend Deployment: Shuttle

---

## 🔗 Live Demo

🌐 https://paraphrase-app.vercel.app  
📦 GitHub: https://github.com/mitraghafari/paraphrase-app

---

## ✨ Features

- Users can type or paste text in a content-editable area
- Can select part of the text
- A paraphrase button appears after selecting
- Sends selected text to backend API
- Backend calls OpenAI and returns a paraphrased version
- Replaces selected text with new one

---

## ⚙️ Local Setup Instructions

### ✅ Prerequisites

- Node.js v14+
- Rust & Cargo
- OpenAI API Key

---

Architecture
Frontend: React (Vite) with contentEditable to support text selection

Backend: Rust with Actix-web and Reqwest to call OpenAI

AI: OpenAI’s chat/completions endpoint used for paraphrasing

🧠 Challenges
Managing CORS between Shuttle and Vercel

Handling async errors in Rust

Setting and securing secrets in Shuttle

Vite build config on Vercel

🔐 Security
API key stored in .env and never committed

In production, OpenAI key is set using shuttle secret

CORS properly configured to allow frontend access only

🔄 Trade-offs & Improvements
Could use a rich text editor like Slate for better UX

Add loading spinner during API call

Add multiple paraphrasing styles (casual, formal, etc.)

Improve error messages and edge-case handling

 Final Links
🚀 Website: https://paraphrase-app.vercel.app

🧾 Repo: https://github.com/mitraghafari/paraphrase-app


